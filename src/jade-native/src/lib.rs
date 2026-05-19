use napi_derive::napi;
use napi::bindgen_prelude::*;
use cranelift_compiler::{Program, Module, Function, IrType, Param, CraneliftCompiler, linker::Linker};
use serde::{Deserialize, Serialize};

#[napi(object)]
pub struct IrResult {
    pub functions: Vec<String>,
    pub globals: Vec<String>,
    pub imports: Vec<String>,
}

#[napi]
pub struct JadeNative {
    target: String,
}

#[napi]
impl JadeNative {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            target: "x86_64-pc-windows-msvc".to_string(),
        }
    }

    #[napi]
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    #[napi]
    pub fn generate_ir(&self, ast_json: String) -> Result<IrResult> {
        let ast: serde_json::Value = serde_json::from_str(&ast_json)
            .map_err(|e| Error::from_reason(format!("AST parse error: {}", e)))?;
        
        let program = self.convert_ast_to_ir(&ast);
        
        Ok(IrResult {
            functions: program.modules.iter()
                .flat_map(|m| m.functions.keys().cloned())
                .collect(),
            globals: vec![],
            imports: program.modules.iter()
                .flat_map(|m| m.imports.clone())
                .collect(),
        })
    }

    #[napi]
    pub fn generate_code(&self, _ir_json: String) -> Result<Buffer> {
        let program = Program::new();
        
        let compiler = CraneliftCompiler::with_target(&self.target)
            .map_err(|e| Error::from_reason(e))?;
        
        let obj_bytes = compiler.compile(&program)
            .map_err(|e| Error::from_reason(e))?;
        
        Ok(Buffer::from(obj_bytes))
    }

    #[napi]
    pub fn link(&self, obj_bytes: Buffer, lib_path: String, output_path: String) -> Result<()> {
        let linker = Linker::new(&self.target)
            .map_err(|e| Error::from_reason(e))?;
        
        linker.link_exe(&obj_bytes, &lib_path, &output_path)
            .map_err(|e| Error::from_reason(e))
    }

    #[napi]
    pub fn link_lib(&self, obj_bytes: Buffer, lib_name: String, _output_path: String) -> Result<()> {
        let linker = Linker::new(&self.target)
            .map_err(|e| Error::from_reason(e))?;
        
        linker.link_lib(&obj_bytes, &lib_name, &_output_path)
            .map_err(|e| Error::from_reason(e))
    }

    fn convert_ast_to_ir(&self, ast: &serde_json::Value) -> Program {
        let mut program = Program::new();
        let mut module = Module::new("main".to_string());
        
        self.visit_ast(ast, &mut module);
        
        program.modules.push(module);
        program
    }

    fn visit_ast(&self, node: &serde_json::Value, module: &mut Module) {
        let type_ = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
        
        match type_ {
            "FunctionDeclaration" => {
                let name = node.get("identifier")
                    .and_then(|id| id.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("anonymous")
                    .to_string();
                
                let params = node.get("params")
                    .and_then(|p| p.as_array())
                    .map(|arr| self.extract_params(arr))
                    .unwrap_or_default();
                
                module.add_function(Function {
                    name,
                    params,
                    return_ty: IrType::Void,
                    body: vec![],
                    is_pub: false,
                    is_extern: false,
                });
            }
            "Module" | "Program" => {
                if let Some(body) = node.get("body").and_then(|b| b.as_array()) {
                    for child in body {
                        self.visit_ast(child, module);
                    }
                }
            }
            "ExportDeclaration" | "ExportNamedDeclaration" => {
                if let Some(decl) = node.get("declarations").and_then(|d| d.as_array()) {
                    for d in decl {
                        self.visit_ast(d, module);
                    }
                }
            }
            _ => {}
        }
    }

    fn extract_params(&self, params: &[serde_json::Value]) -> Vec<Param> {
        params.iter().map(|p| {
            let name = p.get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("_")
                .to_string();
            Param {
                name,
                ty: IrType::I64,
            }
        }).collect()
    }
}

impl Default for JadeNative {
    fn default() -> Self {
        Self::new()
    }
}
