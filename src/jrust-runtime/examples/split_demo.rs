
//! 自动分裂演示：jrust → jrusti(initializer+snap) + jruste(event handler)
use jrust_runtime::director::*;
use jrust_runtime::dom::element::Element;
use jrust_runtime::dom::document::Document;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    println!("🚀 === 自动分裂演示 === 🚀\n");

    let director = Director::new();

    // === 步骤 1: 创建并初始化 DOM ===
    println!("--- 1. 初始化 DOM ---");
    let mut document = Document::new();
    document.set_title("JRust Split Demo");

    let mut body = document.body_mut();
    
    let mut div = Element::new("div");
    div.set_id("app");
    div.set_text_content("Hello from JRust Split!");
    body.append_child(div);

    let mut button = Element::new("button");
    button.set_text_content("Click Me!");
    body.append_child(button);
    
    println!("✅ DOM 初始化完成！");

    // === 步骤 2: 自动分裂！===
    println!("\n--- 2. 自动分裂开始！---");
    let output_dir = PathBuf::from("dist/split_output");
    
    match director.auto_split_into_jrusti_jruste(&document, &output_dir) {
        Ok(_) => {
            println!("\n✅ 分裂成功！");
            println!("\n输出目录：{:?}", output_dir);
            println!("内容：");
            println!("  - app.snap (DOM Snap)");
            println!("  - jrusti.rs (Initializer)");
            println!("  - jruste.rs (Event Handler)");
        }
        Err(e) => {
            eprintln!("\n❌ 分裂失败：{}", e);
        }
    }

    println!("\n🎉 === 分裂演示完成！ ===");
    Ok(())
}
