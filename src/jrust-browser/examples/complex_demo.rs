use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Complex Demo 渲染测试 ===\n");
    
    let config = BrowserConfig {
        width: 1024,
        height: 768,
        title: "Jade Complex Demo".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser 创建成功\n");
    
    let html = fs::read_to_string("examples/complex-demo/demo.html")
        .expect("Failed to read demo.html");
    
    println!("📄 HTML 文件大小: {} bytes", html.len());
    println!("📄 HTML 行数: {} lines\n", html.lines().count());
    
    browser.set_html(&html).unwrap();
    println!("✅ HTML 设置成功\n");
    
    println!("渲染中...");
    let png_data = browser.render();
    println!("✅ 渲染完成! PNG 大小: {} bytes", png_data.len());
    
    std::fs::write("examples/complex-demo/output.png", &png_data).expect("Failed to write PNG");
    println!("✅ 已保存到 examples/complex-demo/output.png");
    
    if png_data.len() > 8 {
        println!("\nPNG 头: {:02x?} (应为 89 50 4e 47)", &png_data[0..4]);
    }
    
    println!("\n=== 测试完成 ===");
}
