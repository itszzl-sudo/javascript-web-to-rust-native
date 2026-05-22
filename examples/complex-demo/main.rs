use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Jade Complex Demo ===\n");
    
    let config = BrowserConfig {
        width: 1024,
        height: 768,
        title: "Jade Complex Demo".to_string(),
        enable_js: true,
        enable_gui: true,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser 创建成功\n");
    
    let html = fs::read_to_string("examples/complex-demo/demo.html")
        .expect("Failed to read demo.html");
    
    browser.set_html(&html).unwrap();
    println!("✅ HTML 加载成功");
    println!("📄 文件大小: {} bytes\n", html.len());
    
    println!("渲染中...");
    let png_data = browser.render();
    println!("✅ 渲染完成! PNG 大小: {} bytes", png_data.len());
    
    std::fs::write("examples/complex-demo/output.png", &png_data).expect("Failed to write PNG");
    println!("✅ 已保存到 examples/complex-demo/output.png");
    
    println!("\n=== Demo 运行中 (按 Ctrl+C 退出) ===");
    
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
