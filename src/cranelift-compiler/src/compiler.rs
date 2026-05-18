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
    
    pub fn compile(&self, _program: &Program) -> Result<Vec<u8>, String> {
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
        
        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(types::I32));
        
        let func_id = module
            .declare_function("app_main", Linkage::Export, &sig)
            .map_err(|e| format!("Declare function failed: {}", e))?;
        
        let mut ctx = module.make_context();
        ctx.func.signature = sig;
        
        {
            let mut fc = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fc);
            
            let block = builder.create_block();
            builder.append_block_params_for_function_params(block);
            builder.switch_to_block(block);
            builder.seal_block(block);
            
            let zero = builder.ins().iconst(types::I32, 0);
            builder.ins().return_(&[zero]);
            
            builder.finalize();
        }
        
        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| format!("Define function failed: {}", e))?;
        module.clear_context(&mut ctx);
        
        let product = module.finish();
        let emit = product
            .emit()
            .map_err(|e| format!("Emit failed: {}", e))?;
        
        println!("✅ 编译完成，生成 {} 字节目标文件", emit.len());
        Ok(emit)
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
