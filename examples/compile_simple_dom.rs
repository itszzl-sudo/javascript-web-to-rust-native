
//! 编译 simple-dom.js 的示例

use jrust_translator::*;
use std::fs;

fn main() {
    // 读取 JavaScript 文件
    let js_content = fs::read_to_string("examples/simple-dom.js").expect("Failed to read file");
    
    println!("=== Compiling simple-dom.js ===");
    println!("{}", js_content);
    
    // 编译
    match compile(&js_content) {
        Ok(result) => {
            println!("\n=== Generated Rust code ===");
            println!("{}", result.code);
            
            // 保存生成的代码
            fs::write("examples/simple-dom-generated.rs", &result.code)
                .expect("Failed to write generated code");
            println!("\n✓ Saved to examples/simple-dom-generated.rs");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
