use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== CSS 渲染验证测试 ===\n");
    
    let config = BrowserConfig {
        width: 600,
        height: 600,
        title: "CSS Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser 创建成功\n");
    
    let html = fs::read_to_string("examples/css-test.html")
        .expect("Failed to read css-test.html");
    
    println!("📄 HTML 内容:");
    println!("{}", html);
    println!("\n");
    
    browser.set_html(&html).unwrap();
    println!("✅ HTML 设置成功\n");
    
    println!("渲染中...");
    let png_data = browser.render();
    println!("✅ 渲染完成! PNG 大小: {} bytes", png_data.len());
    
    std::fs::write("examples/css-render-output.png", &png_data).expect("Failed to write PNG");
    println!("✅ 已保存到 examples/css-render-output.png");
    
    if png_data.len() > 8 {
        println!("\nPNG 头: {:02x?}", &png_data[0..8]);
    }
    
    println!("\n=== 请查看生成的 PNG 文件验证 CSS 效果 ===");
}
