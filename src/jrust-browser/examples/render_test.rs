use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 真实渲染测试 ===\n");

    let config = BrowserConfig {
        width: 800,
        height: 600,
        title: "Render Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };

    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser 创建成功\n");

    // 设置 HTML
    browser.set_html(r#"
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Hello World</h1>
            <div id="app">Test Content</div>
        </body>
        </html>
    "#).unwrap();
    println!("✅ HTML 设置成功\n");

    // 渲染为 PNG
    println!("渲染中...");
    let png_data = browser.render();
    println!("✅ 渲染完成! PNG 大小: {} bytes", png_data.len());

    // 保存 PNG
    std::fs::write("test_output.png", &png_data).expect("Failed to write PNG");
    println!("✅ 已保存到 test_output.png");

    // 验证 PNG 头
    if png_data.len() > 8 {
        println!("\nPNG 头: {:02x?} (应为 89 50 4e 47)", &png_data[0..4]);
    }

    println!("\n=== 测试完成 ===");
}
