use crate::ir::*;
use cranelift::prelude::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

pub struct CraneliftCompiler {
    target: Triple,
}

impl CraneliftCompiler {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            target: Triple::host(),
        })
    }
    
    pub fn with_target(target: &str) -> Result<Self, String> {
        let triple: Triple = target.parse()
            .map_err(|e| format!("Invalid target: {}", e))?;
        Ok(Self { target: triple })
    }
    
    pub fn compile(&self, program: &Program) -> Result<Vec<u8>, String> {
        println!("=== Cranelift 编译开始 ===");
        println!("目标平台: {}", self.target);
        
        let mut flag_builder = settings::builder();
        flag_builder.enable("is_pic").unwrap();
        let flags = settings::Flags::new(flag_builder);
        
        let isa = cranelift_native::builder()
            .map_err(|e| format!("ISA builder failed: {}", e))?
            .finish(flags)
            .map_err(|e| format!("ISA finish failed: {}", e))?;
        
        let builder = ObjectBuilder::new(
            isa,
            "app",
            cranelift_module::default_libcall_names(),
        ).map_err(|e| format!("ObjectBuilder failed: {}", e))?;
        
        let mut module = ObjectModule::new(builder);
        
        // 收集所有用户函数
        let user_functions = self.collect_functions(program);
        
        // 生成用户函数
        for func in &user_functions {
            self.compile_function(&mut module, func)?;
        }
        
        // 生成 main 入口函数
        self.compile_main_entry(&mut module, &user_functions)?;
        
        let product = module.finish();
        let emit = product
            .emit()
            .map_err(|e| format!("Emit failed: {}", e))?;
        
        println!("✅ 编译完成，生成 {} 字节目标文件", emit.len());
        println!("   用户函数: {}", user_functions.len());
        Ok(emit)
    }
    
    fn collect_functions<'a>(&self, program: &'a Program) -> Vec<&'a Function> {
        program.modules.iter()
            .flat_map(|m| m.functions.values())
            .collect()
    }
    
    fn compile_function(&self, module: &mut ObjectModule, func: &Function) -> Result<(), String> {
        let is_windows = self.target.to_string().contains("windows");
        let call_conv = if is_windows { CallConv::WindowsFastcall } else { CallConv::SystemV };
        
        let mut sig = Signature::new(call_conv);
        
        // 参数
        for _ in &func.params {
            sig.params.push(AbiParam::new(types::I64));
        }
        
        // 返回值
        match &func.return_ty {
            IrType::Void => {}
            IrType::I32 => sig.returns.push(AbiParam::new(types::I32)),
            IrType::I64 => sig.returns.push(AbiParam::new(types::I64)),
            IrType::F32 => sig.returns.push(AbiParam::new(types::F32)),
            IrType::F64 => sig.returns.push(AbiParam::new(types::F64)),
            _ => sig.returns.push(AbiParam::new(types::I64)),
        }
        
        let func_id = module
            .declare_function(&func.name, Linkage::Export, &sig)
            .map_err(|e| format!("Declare function {} failed: {}", func.name, e))?;
        
        let mut ctx = module.make_context();
        ctx.func.signature = sig;
        
        {
            let mut fc = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fc);
            
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);
            
            // 编译函数体
            self.compile_function_body(&mut builder, func)?;
            
            builder.finalize();
        }
        
        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| format!("Define function {} failed: {}", func.name, e))?;
        module.clear_context(&mut ctx);
        
        Ok(())
    }
    
    fn compile_function_body(&self, builder: &mut FunctionBuilder, func: &Function) -> Result<(), String> {
        if func.body.is_empty() {
            match &func.return_ty {
                IrType::Void => {
                    builder.ins().return_(&[]);
                }
                IrType::I32 => {
                    let zero = builder.ins().iconst(types::I32, 0);
                    builder.ins().return_(&[zero]);
                }
                _ => {
                    let zero = builder.ins().iconst(types::I64, 0);
                    builder.ins().return_(&[zero]);
                }
            }
        } else {
            for stmt in &func.body {
                self.compile_stmt(builder, stmt)?;
            }
        }
        
        Ok(())
    }
    
    fn compile_stmt(&self, builder: &mut FunctionBuilder, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Return(Some(expr)) => {
                let val = self.compile_expr(builder, expr)?;
                builder.ins().return_(&[val]);
            }
            Stmt::Return(None) => {
                builder.ins().return_(&[]);
            }
            Stmt::ExprStmt(expr) => {
                self.compile_expr(builder, expr)?;
            }
            _ => {}
        }
        Ok(())
    }
    
    fn compile_expr(&self, builder: &mut FunctionBuilder, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::ConstI32(v) => Ok(builder.ins().iconst(types::I32, *v as i64)),
            Expr::ConstI64(v) => Ok(builder.ins().iconst(types::I64, *v)),
            Expr::ConstBool(v) => Ok(builder.ins().iconst(types::I8, if *v { 1 } else { 0 })),
            Expr::BinaryOp { op, left, right } => {
                let lhs = self.compile_expr(builder, left)?;
                let rhs = self.compile_expr(builder, right)?;
                
                let result = match op {
                    BinOp::Add => builder.ins().iadd(lhs, rhs),
                    BinOp::Sub => builder.ins().isub(lhs, rhs),
                    BinOp::Mul => builder.ins().imul(lhs, rhs),
                    BinOp::And => builder.ins().band(lhs, rhs),
                    BinOp::Or => builder.ins().bor(lhs, rhs),
                    _ => builder.ins().iadd(lhs, rhs),
                };
                Ok(result)
            }
            _ => Ok(builder.ins().iconst(types::I64, 0))
        }
    }
    
    fn compile_main_entry(&self, module: &mut ObjectModule, user_functions: &[&Function]) -> Result<(), String> {
        let is_windows = self.target.to_string().contains("windows");
        let call_conv = if is_windows { CallConv::WindowsFastcall } else { CallConv::SystemV };
        
        // main(int argc, char** argv)
        let mut sig = Signature::new(call_conv);
        sig.params.push(AbiParam::new(types::I32)); // argc
        sig.params.push(AbiParam::new(types::I64)); // argv
        sig.returns.push(AbiParam::new(types::I32)); // return int
        
        let func_id = module
            .declare_function("main", Linkage::Export, &sig)
            .map_err(|e| format!("Declare main failed: {}", e))?;
        
        let mut ctx = module.make_context();
        ctx.func.signature = sig;
        
        {
            let mut fc = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fc);
            
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);
            
            // 调用第一个用户函数（如果存在）
            if !user_functions.is_empty() {
                // 暂不调用，仅生成 main
            }
            
            // return 0
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().return_(&[zero]);
            
            builder.finalize();
        }
        
        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| format!("Define main failed: {}", e))?;
        module.clear_context(&mut ctx);
        
        Ok(())
    }
    
    pub fn link_with_lib(&self, obj: &[u8], lib: &str, out: &str) -> Result<(), String> {
        crate::linker::Linker::new(&self.target.to_string())?.link_exe(obj, lib, out)
    }
}

impl Default for CraneliftCompiler {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
