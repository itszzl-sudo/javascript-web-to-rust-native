use napi_derive::napi;
use napi::bindgen_prelude::*;
use cranelift_compiler::{
    Program, Module, Function, Function as IrFunction, 
    Stmt, Expr, IrType, Param, BinOp, UnaryOp,
    CraneliftCompiler, linker::Linker
};
use serde::{Deserialize, Serialize};

#[napi(object)]
pub struct IrResult {
    pub program_json: String,
    pub functions: Vec<String>,
    pub globals: Vec<String>,
    pub imports: Vec<String>,
    pub stats: IrStats,
}

#[napi(object)]
pub struct IrStats {
    pub total_functions: u32,
    pub total_variables: u32,
    pub total_imports: u32,
    pub supported_features: Vec<String>,
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
        
        let mut program = Program::new();
        let mut module = Module::new("main".to_string());
        let mut stats = IrStats {
            total_functions: 0,
            total_variables: 0,
            total_imports: 0,
            supported_features: vec![],
        };
        
        self.visit_ast(&ast, &mut module, &mut stats);
        
        stats.total_functions = module.functions.len() as u32;
        stats.total_imports = module.imports.len() as u32;
        
        program.modules.push(module);
        
        let program_json = serde_json::to_string(&program)
            .map_err(|e| Error::from_reason(format!("Program serialize error: {}", e)))?;
        
        Ok(IrResult {
            program_json,
            functions: program.modules.iter()
                .flat_map(|m| m.functions.keys().cloned())
                .collect(),
            globals: vec![],
            imports: program.modules.iter()
                .flat_map(|m| m.imports.clone())
                .collect(),
            stats,
        })
    }

    #[napi]
    pub fn generate_code(&self, program_json: String) -> Result<Buffer> {
        let program: Program = serde_json::from_str(&program_json)
            .map_err(|e| Error::from_reason(format!("Program parse error: {}", e)))?;
        
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

    fn visit_ast(&self, node: &serde_json::Value, module: &mut Module, stats: &mut IrStats) {
        let type_ = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
        
        match type_ {
            "AsyncFunctionDeclaration" | "AsyncFunctionExpression" => {
                if let Some(func) = self.parse_function(node) {
                    module.add_function(func);
                    stats.supported_features.push("async-function".to_string());
                }
            }
            
            "TryStatement" => {
                self.parse_try_statement(node, module, stats);
                stats.supported_features.push("try-catch".to_string());
            }
            
            "ThrowStatement" => {
                stats.supported_features.push("throw".to_string());
            }
            
            "SwitchStatement" => {
                self.parse_switch_statement(node, module, stats);
                stats.supported_features.push("switch".to_string());
            }
            
            "ArrayExpression" => {
                stats.supported_features.push("array".to_string());
            }
            
            "ObjectExpression" => {
                stats.supported_features.push("object".to_string());
            }
            
            "ArrowFunctionExpression" => {
                if let Some(func) = self.parse_arrow_function(node, stats) {
                    module.add_function(func);
                    stats.supported_features.push("arrow-function".to_string());
                }
            }
            
            "TemplateLiteral" => {
                stats.supported_features.push("template-literal".to_string());
            }
            
            "SpreadElement" => {
                stats.supported_features.push("spread".to_string());
            }
            
            "ObjectPattern" | "ArrayPattern" => {
                stats.supported_features.push("destructuring".to_string());
            }
            
            "FunctionDeclaration" => {
                if let Some(func) = self.parse_function(node) {
                    module.add_function(func);
                    stats.supported_features.push("function".to_string());
                }
            }
            
            "FunctionExpression" => {
                if let Some(func) = self.parse_function(node) {
                    module.add_function(func);
                }
            }
            
            "VariableDeclaration" => {
                self.parse_variable_declaration(node, module, stats);
                stats.supported_features.push("variable".to_string());
            }
            
            "ClassDeclaration" => {
                self.parse_class_declaration(node, module, stats);
                stats.supported_features.push("class".to_string());
            }
            
            "ClassExpression" => {
                self.parse_class_declaration(node, module, stats);
            }
            
            "ImportDeclaration" => {
                if let Some(source) = node.get("source").and_then(|s| s.as_str()) {
                    module.imports.push(source.to_string());
                }
                stats.supported_features.push("import".to_string());
            }
            
            "ExportDeclaration" | "ExportNamedDeclaration" => {
                if let Some(decl) = node.get("declaration") {
                    self.visit_ast(decl, module, stats);
                } else if let Some(specifiers) = node.get("specifiers").and_then(|s| s.as_array()) {
                    for spec in specifiers {
                        self.visit_ast(spec, module, stats);
                    }
                }
                stats.supported_features.push("export".to_string());
            }
            
            "ExportDefaultDeclaration" => {
                if let Some(decl) = node.get("declaration") {
                    self.visit_ast(decl, module, stats);
                }
                stats.supported_features.push("export-default".to_string());
            }
            
            "Module" | "Program" | "Script" => {
                if let Some(body) = node.get("body").and_then(|b| b.as_array()) {
                    for child in body {
                        self.visit_ast(child, module, stats);
                    }
                }
            }
            
            "ExpressionStatement" => {}
            
            "DoWhileStatement" => {
                stats.supported_features.push("do-while".to_string());
            }
            
            "ForInStatement" => {
                stats.supported_features.push("for-in".to_string());
            }
            
            "ForOfStatement" => {
                stats.supported_features.push("for-of".to_string());
            }
            
            "LabeledStatement" => {
                stats.supported_features.push("label".to_string());
            }
            
            "WithStatement" => {
                stats.supported_features.push("with".to_string());
            }
            
            "DebuggerStatement" => {
                stats.supported_features.push("debugger".to_string());
            }
            
            _ => {}
        }
    }
    
    fn parse_arrow_function(&self, node: &serde_json::Value, stats: &IrStats) -> Option<Function> {
        let params = node.get("params")
            .and_then(|p| p.as_array())
            .map(|arr| self.extract_params(arr))
            .unwrap_or_default();
        
        let body = if let Some(body_node) = node.get("body") {
            if body_node.get("type").and_then(|t| t.as_str()) == Some("BlockStatement") {
                self.parse_function_body(body_node).unwrap_or_default()
            } else {
                vec![Stmt::Return(self.parse_expression(body_node))]
            }
        } else {
            vec![]
        };
        
        let return_ty = self.infer_return_type(&body);
        
        Some(Function {
            name: format!("arrow_{}", stats.total_functions),
            params,
            return_ty,
            body,
            is_pub: false,
            is_extern: false,
        })
    }
    
    fn parse_try_statement(&self, node: &serde_json::Value, _module: &mut Module, stats: &mut IrStats) {
        stats.total_functions += 1;
    }
    
    fn parse_switch_statement(&self, node: &serde_json::Value, _module: &mut Module, stats: &mut IrStats) {
        if let Some(cases) = node.get("cases").and_then(|c| c.as_array()) {
            stats.total_functions += cases.len() as u32;
        }
    }

    fn parse_function(&self, node: &serde_json::Value) -> Option<Function> {
        let name = node.get("identifier")
            .and_then(|id| id.get("value"))
            .and_then(|v| v.as_str())
            .or_else(|| node.get("id").and_then(|id| id.get("value")).and_then(|v| v.as_str()))
            .unwrap_or("anonymous")
            .to_string();
        
        let params = node.get("params")
            .and_then(|p| p.as_array())
            .map(|arr| self.extract_params(arr))
            .unwrap_or_default();
        
        // 解析函数体
        let body = node.get("body")
            .and_then(|b| self.parse_function_body(b))
            .unwrap_or_default();
        
        // 推断返回类型
        let return_ty = self.infer_return_type(&body);
        
        Some(Function {
            name,
            params,
            return_ty,
            body,
            is_pub: false,
            is_extern: false,
        })
    }

    fn parse_function_body(&self, body_node: &serde_json::Value) -> Option<Vec<Stmt>> {
        let mut stmts = vec![];
        
        if let Some(body) = body_node.get("stmts").and_then(|s| s.as_array()) {
            for stmt in body {
                if let Some(s) = self.parse_statement(stmt) {
                    stmts.push(s);
                }
            }
        } else if let Some(stmts_arr) = body_node.get("body").and_then(|b| b.as_array()) {
            for stmt in stmts_arr {
                if let Some(s) = self.parse_statement(stmt) {
                    stmts.push(s);
                }
            }
        }
        
        if stmts.is_empty() {
            None
        } else {
            Some(stmts)
        }
    }

    fn parse_statement(&self, node: &serde_json::Value) -> Option<Stmt> {
        let type_ = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
        
        match type_ {
            "ReturnStatement" => {
                let value = node.get("argument")
                    .and_then(|arg| self.parse_expression(arg));
                Some(Stmt::Return(value))
            }
            
            "VariableDeclaration" | "VarDecl" => {
                let name = node.get("name")
                    .and_then(|n| n.as_str())
                    .or_else(|| node.get("id").and_then(|id| id.get("value")).and_then(|v| v.as_str()))
                    .unwrap_or("_")
                    .to_string();
                
                let init = node.get("init")
                    .and_then(|i| self.parse_expression(i));
                
                let ty = init.as_ref().map(|e| self.infer_expr_type(e)).unwrap_or(IrType::I64);
                
                Some(Stmt::VarDecl(cranelift_compiler::LocalVar {
                    name,
                    ty,
                    init,
                }))
            }
            
            "ExpressionStatement" => {
                node.get("expression")
                    .and_then(|expr| self.parse_expression(expr))
                    .map(Stmt::ExprStmt)
            }
            
            "IfStatement" => {
                let cond = node.get("test").and_then(|t| self.parse_expression(t))?;
                let then_block = node.get("consequent")
                    .and_then(|c| self.parse_statement(c))
                    .map(|s| vec![s])
                    .unwrap_or_default();
                
                let else_block = node.get("alternate")
                    .and_then(|a| self.parse_statement(a))
                    .map(|s| vec![s]);
                
                Some(Stmt::If {
                    cond,
                    then_block,
                    else_block,
                })
            }
            
            "WhileStatement" => {
                let cond = node.get("test").and_then(|t| self.parse_expression(t))?;
                let body = node.get("body")
                    .and_then(|b| self.parse_statement(b))
                    .map(|s| vec![s])
                    .unwrap_or_default();
                
                Some(Stmt::While { cond, body })
            }
            
            "DoWhileStatement" => {
                let body = node.get("body")
                    .and_then(|b| self.parse_statement(b))
                    .map(|s| vec![s])
                    .unwrap_or_default();
                let cond = node.get("test").and_then(|t| self.parse_expression(t))?;
                
                Some(Stmt::Block(vec![
                    Stmt::Block(body.clone()),
                    Stmt::While { cond, body },
                ]))
            }
            
            "ForInStatement" | "ForOfStatement" => {
                let body = node.get("body")
                    .and_then(|b| self.parse_statement(b))
                    .map(|s| vec![s])
                    .unwrap_or_default();
                
                Some(Stmt::Block(body))
            }
            
            "SwitchStatement" => {
                let cases = node.get("cases")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|case| self.parse_statement(case))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                
                Some(Stmt::Block(cases))
            }
            
            "SwitchCase" => {
                let consequent = node.get("consequent")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| self.parse_statement(s))
                            .collect()
                    })
                    .unwrap_or_default();
                
                Some(Stmt::Block(consequent))
            }
            
            "TryStatement" => {
                let block = node.get("block")
                    .and_then(|b| self.parse_statement(b))
                    .unwrap_or(Stmt::Block(vec![]));
                
                let handler = node.get("handler")
                    .and_then(|h| self.parse_statement(h))
                    .unwrap_or(Stmt::Block(vec![]));
                
                let finalizer = node.get("finalizer")
                    .and_then(|f| self.parse_statement(f))
                    .map(|s| vec![s]);
                
                Some(Stmt::Block(vec![block, handler]))
            }
            
            "CatchClause" => {
                let body = node.get("body")
                    .and_then(|b| self.parse_statement(b))
                    .unwrap_or(Stmt::Block(vec![]));
                
                Some(body)
            }
            
            "ThrowStatement" => {
                let _arg = node.get("argument").and_then(|a| self.parse_expression(a));
                Some(Stmt::Return(None))
            }
            
            "LabeledStatement" => {
                node.get("body")
                    .and_then(|b| self.parse_statement(b))
            }
            
            "ContinueStatement" => {
                Some(Stmt::Return(None))
            }
            
            "BreakStatement" => {
                Some(Stmt::Return(None))
            }
            
            "ForStatement" => {
                let init = node.get("init")
                    .and_then(|i| self.parse_statement(i))
                    .map(|s| Box::new(s))
                    .unwrap_or_else(|| Box::new(Stmt::Block(vec![])));
                
                let cond = node.get("test").and_then(|t| self.parse_expression(t)).unwrap_or(Expr::ConstBool(true));
                let update = node.get("update")
                    .and_then(|u| self.parse_statement(u))
                    .map(|s| Box::new(s))
                    .unwrap_or_else(|| Box::new(Stmt::Block(vec![])));
                
                let body = node.get("body")
                    .and_then(|b| self.parse_statement(b))
                    .map(|s| vec![s])
                    .unwrap_or_default();
                
                Some(Stmt::For {
                    init,
                    cond,
                    update,
                    body,
                })
            }
            
            "BlockStatement" | "Block" => {
                let stmts = node.get("stmts")
                    .or_else(|| node.get("body"))
                    .and_then(|s| s.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| self.parse_statement(s))
                            .collect()
                    })
                    .unwrap_or_default();
                
                Some(Stmt::Block(stmts))
            }
            
            _ => None
        }
    }

    fn parse_expression(&self, node: &serde_json::Value) -> Option<Expr> {
        let type_ = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
        
        match type_ {
            "NumericLiteral" | "NumberLiteral" => {
                let value = node.get("value").and_then(|v| v.as_i64())? as i32;
                Some(Expr::ConstI32(value))
            }
            
            "StringLiteral" => {
                let value = node.get("value").and_then(|v| v.as_str())?.to_string();
                Some(Expr::ConstString(value))
            }
            
            "BooleanLiteral" => {
                let value = node.get("value").and_then(|v| v.as_bool())?;
                Some(Expr::ConstBool(value))
            }
            
            "NullLiteral" => Some(Expr::ConstNull),
            
            "Identifier" | "IdentifierReference" => {
                let name = node.get("value")
                    .or_else(|| node.get("name"))
                    .and_then(|v| v.as_str())?
                    .to_string();
                Some(Expr::Var(name))
            }
            
            "BinaryExpression" => {
                let op = node.get("operator").and_then(|o| o.as_str()).unwrap_or("+");
                let left = node.get("left").and_then(|l| self.parse_expression(l))?;
                let right = node.get("right").and_then(|r| self.parse_expression(r))?;
                
                let bin_op = match op {
                    "+" => BinOp::Add,
                    "-" => BinOp::Sub,
                    "*" => BinOp::Mul,
                    "/" => BinOp::Div,
                    "%" => BinOp::Mod,
                    "==" | "===" => BinOp::Eq,
                    "!=" | "!==" => BinOp::Ne,
                    "<" => BinOp::Lt,
                    "<=" => BinOp::Le,
                    ">" => BinOp::Gt,
                    ">=" => BinOp::Ge,
                    "&&" => BinOp::And,
                    "||" => BinOp::Or,
                    "&" => BinOp::BitAnd,
                    "|" => BinOp::BitOr,
                    "^" => BinOp::BitXor,
                    "<<" => BinOp::Shl,
                    ">>" => BinOp::Shr,
                    _ => BinOp::Add,
                };
                
                Some(Expr::BinaryOp {
                    op: bin_op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            
            "UnaryExpression" => {
                let op = node.get("operator").and_then(|o| o.as_str()).unwrap_or("+");
                let expr = node.get("argument")
                    .or_else(|| node.get("expr"))
                    .and_then(|e| self.parse_expression(e))?;
                
                let unary_op = match op {
                    "-" => UnaryOp::Neg,
                    "!" => UnaryOp::Not,
                    "+" => UnaryOp::Neg,
                    "~" => UnaryOp::Not,
                    _ => UnaryOp::Neg,
                };
                
                Some(Expr::UnaryOp {
                    op: unary_op,
                    expr: Box::new(expr),
                })
            }
            
            "UpdateExpression" => {
                let op = node.get("operator").and_then(|o| o.as_str()).unwrap_or("++");
                let arg = node.get("argument").and_then(|a| self.parse_expression(a))?;
                
                let inc = match op {
                    "++" => Expr::BinaryOp {
                        op: BinOp::Add,
                        left: Box::new(arg.clone()),
                        right: Box::new(Expr::ConstI32(1)),
                    },
                    "--" => Expr::BinaryOp {
                        op: BinOp::Sub,
                        left: Box::new(arg.clone()),
                        right: Box::new(Expr::ConstI32(1)),
                    },
                    _ => arg.clone(),
                };
                
                Some(inc)
            }
            
            "ConditionalExpression" => {
                let cond = node.get("test").and_then(|t| self.parse_expression(t))?;
                let then_expr = node.get("consequent").and_then(|c| self.parse_expression(c))?;
                let else_expr = node.get("alternate").and_then(|a| self.parse_expression(a))?;
                
                Some(Expr::BinaryOp {
                    op: BinOp::Mul,
                    left: Box::new(Expr::BinaryOp {
                        op: BinOp::And,
                        left: Box::new(cond),
                        right: Box::new(then_expr),
                    }),
                    right: Box::new(else_expr),
                })
            }
            
            "SequenceExpression" => {
                node.get("expressions")
                    .and_then(|e| e.as_array())
                    .and_then(|arr| arr.last())
                    .and_then(|last| self.parse_expression(last))
            }
            
            "NewExpression" => {
                let callee = node.get("callee").and_then(|c| self.parse_expression(c))?;
                let args = node.get("arguments")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|arg| self.parse_expression(arg))
                            .collect()
                    })
                    .unwrap_or_default();
                
                if let Expr::Var(func_name) = &callee {
                    Some(Expr::Call {
                        func: format!("new_{}", func_name),
                        args,
                    })
                } else {
                    Some(Expr::Call {
                        func: "new".to_string(),
                        args,
                    })
                }
            }
            
            "AwaitExpression" => {
                node.get("argument")
                    .and_then(|a| self.parse_expression(a))
            }
            
            "YieldExpression" => {
                node.get("argument")
                    .and_then(|a| self.parse_expression(a))
            }
            
            "MetaProperty" => {
                Some(Expr::ConstNull)
            }
            
            "CallExpression" => {
                let callee = node.get("callee").and_then(|c| self.parse_expression(c))?;
                
                let args = node.get("arguments")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|arg| self.parse_expression(arg))
                            .collect()
                    })
                    .unwrap_or_default();
                
                // 如果调用的是标识符，直接使用函数名
                if let Expr::Var(func_name) = &callee {
                    Some(Expr::Call {
                        func: func_name.clone(),
                        args,
                    })
                } else {
                    // 复杂调用（如成员函数调用）
                    Some(Expr::Call {
                        func: "unknown".to_string(),
                        args,
                    })
                }
            }
            
            "MemberExpression" => {
                let obj = node.get("object").and_then(|o| self.parse_expression(o))?;
                let prop = node.get("property")
                    .and_then(|p| p.get("value").and_then(|v| v.as_str()))
                    .or_else(|| node.get("property").and_then(|p| p.as_str()))
                    .unwrap_or("_")
                    .to_string();
                
                Some(Expr::FieldAccess {
                    base: Box::new(obj),
                    field: prop,
                })
            }
            
            "AssignmentExpression" => {
                let op = node.get("operator").and_then(|o| o.as_str()).unwrap_or("=");
                let left = node.get("left").and_then(|l| self.parse_expression(l));
                let right = node.get("right").and_then(|r| self.parse_expression(r))?;
                
                match op {
                    "=" => Some(right),
                    "+=" => Some(Expr::BinaryOp {
                        op: BinOp::Add,
                        left: Box::new(left?),
                        right: Box::new(right),
                    }),
                    "-=" => Some(Expr::BinaryOp {
                        op: BinOp::Sub,
                        left: Box::new(left?),
                        right: Box::new(right),
                    }),
                    "*=" => Some(Expr::BinaryOp {
                        op: BinOp::Mul,
                        left: Box::new(left?),
                        right: Box::new(right),
                    }),
                    "/=" => Some(Expr::BinaryOp {
                        op: BinOp::Div,
                        left: Box::new(left?),
                        right: Box::new(right),
                    }),
                    "%=" => Some(Expr::BinaryOp {
                        op: BinOp::Mod,
                        left: Box::new(left?),
                        right: Box::new(right),
                    }),
                    _ => Some(right),
                }
            }
            
            "LogicalExpression" => {
                let op = node.get("operator").and_then(|o| o.as_str()).unwrap_or("&&");
                let left = node.get("left").and_then(|l| self.parse_expression(l))?;
                let right = node.get("right").and_then(|r| self.parse_expression(r))?;
                
                let bin_op = match op {
                    "&&" => BinOp::And,
                    "||" => BinOp::Or,
                    "??" => BinOp::Or,
                    _ => BinOp::And,
                };
                
                Some(Expr::BinaryOp {
                    op: bin_op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            
            "OptionalMemberExpression" => {
                let obj = node.get("object").and_then(|o| self.parse_expression(o))?;
                let prop = node.get("property")
                    .and_then(|p| p.get("value").and_then(|v| v.as_str()))
                    .or_else(|| node.get("property").and_then(|p| p.as_str()))
                    .unwrap_or("_")
                    .to_string();
                
                Some(Expr::FieldAccess {
                    base: Box::new(obj),
                    field: prop,
                })
            }
            
            "OptionalCallExpression" => {
                let callee = node.get("callee").and_then(|c| self.parse_expression(c))?;
                let args = node.get("arguments")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|arg| self.parse_expression(arg))
                            .collect()
                    })
                    .unwrap_or_default();
                
                if let Expr::Var(func_name) = &callee {
                    Some(Expr::Call {
                        func: func_name.clone(),
                        args,
                    })
                } else {
                    Some(Expr::Call {
                        func: "optional_call".to_string(),
                        args,
                    })
                }
            }
            
            "PrivateName" => {
                let name = node.get("id")
                    .and_then(|id| id.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("_")
                    .to_string();
                Some(Expr::Var(format!("private_{}", name)))
            }
            
            "Super" => {
                Some(Expr::Var("super".to_string()))
            }
            
            "Import" => {
                Some(Expr::ConstNull)
            }
            
            "TaggedTemplateExpression" => {
                node.get("quasi")
                    .and_then(|q| self.parse_expression(q))
            }
            
            "ClassExpression" => {
                Some(Expr::ConstNull)
            }
            
            _ => None
        }
    }

    fn parse_variable_declaration(&self, node: &serde_json::Value, _module: &mut Module, stats: &mut IrStats) {
        if let Some(decls) = node.get("declarations").and_then(|d| d.as_array()) {
            stats.total_variables += decls.len() as u32;
        }
    }

    fn parse_class_declaration(&self, node: &serde_json::Value, _module: &mut Module, stats: &mut IrStats) {
        let _name = node.get("identifier")
            .and_then(|id| id.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("Anonymous");
        
        if let Some(body) = node.get("body").and_then(|b| b.get("body")).and_then(|b| b.as_array()) {
            for member in body {
                if member.get("type").and_then(|t| t.as_str()) == Some("MethodDefinition") {
                    stats.total_functions += 1;
                }
            }
        }
    }

    fn extract_params(&self, params: &[serde_json::Value]) -> Vec<Param> {
        params.iter().map(|p| {
            let name = p.get("value")
                .or_else(|| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("_")
                .to_string();
            Param {
                name,
                ty: IrType::I64,
            }
        }).collect()
    }

    fn infer_return_type(&self, body: &[Stmt]) -> IrType {
        for stmt in body {
            if let Stmt::Return(Some(expr)) = stmt {
                return self.infer_expr_type(expr);
            }
        }
        IrType::Void
    }

    fn infer_expr_type(&self, expr: &Expr) -> IrType {
        match expr {
            Expr::ConstI32(_) => IrType::I32,
            Expr::ConstI64(_) => IrType::I64,
            Expr::ConstF32(_) => IrType::F32,
            Expr::ConstF64(_) => IrType::F64,
            Expr::ConstBool(_) => IrType::Bool,
            Expr::ConstString(_) => IrType::String,
            Expr::ConstNull => IrType::Ptr,
            Expr::BinaryOp { .. } => IrType::I64,
            Expr::Call { .. } => IrType::I64,
            _ => IrType::I64,
        }
    }
}

impl Default for JadeNative {
    fn default() -> Self {
        Self::new()
    }
}
