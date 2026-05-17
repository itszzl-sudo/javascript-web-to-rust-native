//! # Binding Generator
//! 
//! 自动生成 JavaScript → Rust 绑定的 proc-macro

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat, Type};

#[proc_macro_attribute]
pub fn js_binding(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // 解析函数信息
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    
    // 生成注册包装
    let output = quote! {
        // 原始函数
        #input
        
        // 注册辅助函数
        paste::paste! {
            pub fn [<register_ #fn_name>](registry: &mut jrust_runtime::bindings::BindingRegistry) {
                registry.register(stringify!(#fn_name), |args| -> Result<jrust_runtime::JsValue, String> {
                    // 简单的参数处理和调用
                    // 实际实现可以更完整，处理类型转换等
                    #fn_block;
                    Ok(jrust_runtime::JsValue::Undefined)
                });
            }
        }
    };
    
    output.into()
}
