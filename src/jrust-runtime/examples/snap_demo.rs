
//! Snap 功能演示 - 展示静默检测和自动 Snap 生成

use jrust_runtime::director::{Director, JRustApp};
use jrust_runtime::dom::element::Element;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    println!("🚀 === Snap 功能演示 ===\n");

    // === 演示 1: JRustApp 自动 Snap ===
    println!("=== 演示 1: JRustApp 自动 Snap ===\n");
    
    {
        // 创建应用，设置自动保存路径
        let mut app = JRustApp::new()
            .with_snap_output(PathBuf::from("auto_app.snap"));
        
        // 初始化 DOM
        let window = &mut app.window;
        let mut body = window.document.body_mut();
        
        let mut div = Element::new("div");
        div.set_id("content");
        div.set_text_content("Hello from JRust Auto-Snap!");
        body.append_child(div);
        
        let mut button = Element::new("button");
        button.set_text_content("Click me!");
        body.append_child(button);
        
        println!("✅ DOM 初始化完成");
        
        // 在这里，作用域结束 → app 被 drop → 自动保存 snap！✨
    }
    
    // === 演示 2: 使用 Director 显式 Snap ===
    println!("\n=== 演示 2: Director 显式 Snap ===\n");
    
    let director = Director::new();
    
    // 创建一个 DOM 文档
    let mut document = jrust_runtime::dom::document::Document::new();
    document.set_title("My Snap Document");
    
    let mut body = document.body_mut();
    let mut h1 = Element::new("h1");
    h1.set_text_content("Snap Demo");
    body.append_child(h1);
    
    let mut p = Element::new("p");
    p.set_text_content("This DOM will be saved to a binary snap file!");
    body.append_child(p);
    
    // 显式生成 snap
    let snap_path = PathBuf::from("manual_app.snap");
    director.save_snap_to_file(&document, &snap_path)?;
    
    // === 演示 3: 从 Snap 恢复 ===
    println!("\n=== 演示 3: 从 Snap 恢复 ===\n");
    
    let restored_doc = director.load_snap_from_file(&snap_path)?;
    println!("✅ 已从 Snap 恢复文档！");
    println!("   标题: {}", restored_doc.title());
    
    println!("\n🎉 === 所有演示完成 ===\n");
    Ok(())
}
