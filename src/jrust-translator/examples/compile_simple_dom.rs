
//! 编译 simple-dom.js 的示例

use jrust_translator::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 获取项目根目录
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .canonicalize()
        .unwrap();
    
    // 读取 JavaScript 文件
    let js_path = project_root.join("examples/simple-dom.js");
    let js_content = fs::read_to_string(&js_path).expect("Failed to read file");
    
    println!("=== Compiling simple-dom.js ===");
    println!("{}", js_content);
    
    // 编译
    match compile(&js_content) {
        Ok(result) => {
            println!("\n=== Generated Rust code ===");
            println!("{}", result.code);
            
            // 保存生成的代码
            let output_path = project_root.join("examples/simple-dom-generated.rs");
            fs::write(&output_path, &result.code)
                .expect("Failed to write generated code");
            println!("\n✓ Saved to {:?}", output_path);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
