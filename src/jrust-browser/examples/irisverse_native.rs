use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Irisverse Native ===\n");
    
    let html = fs::read_to_string("C:/Users/a/Documents/comate/irisverse-org/index-bundle.html")
        .expect("Failed to read HTML");
    
    println!("📄 HTML: {} bytes, {} lines\n", html.len(), html.lines().count());
    
    let config = BrowserConfig {
        width: 1280,
        height: 800,
        title: "Irisverse - Native".into(),
        enable_js: true,
        enable_gui: true,
    };
    
    println!("🖥️  Creating native window...\n");
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    browser.set_html(&html).expect("Failed to set HTML");
    
    println!("✅ Irisverse loaded\n");
    println!("📸 Rendering to PNG...\n");
    
    let png = browser.render();
    fs::write("examples/irisverse-native.png", &png).expect("Failed to write PNG");
    
    println!("✅ Screenshot saved: examples/irisverse-native.png");
    println!("   Size: {} bytes\n", png.len());
    
    println!("🎉 Irisverse Native 运行中！");
    println!("   窗口大小: 1280x800");
    println!("   按 Ctrl+C 退出\n");
    
    // 保持运行
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
