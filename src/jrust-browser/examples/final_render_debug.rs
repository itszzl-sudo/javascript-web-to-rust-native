use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Final CSS Rendering Demo (Debug) ===\n");
    
    let config = BrowserConfig {
        width: 1024,
        height: 768,
        title: "Jade Final Demo".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser created\n");
    
    let html = fs::read_to_string("examples/final-demo.html")
        .expect("Failed to read final-demo.html");
    
    println!("📄 HTML size: {} bytes\n", html.len());
    
    browser.set_html(&html).unwrap();
    println!("✅ HTML loaded with CSS\n");
    
    // 获取所有布局节点
    let rects = browser.all_rects();
    println!("📊 Layout nodes: {}", rects.len());
    
    for (i, (id, tag, x, y, w, h)) in rects.iter().enumerate() {
        println!("  [{}] {} (id:{}) - pos: ({:.1}, {:.1}) size: ({:.1}x{:.1})", 
            i, tag, id, x, y, w, h);
    }
    
    println!("\nRendering...");
    let png_data = browser.render();
    println!("✅ Render complete! PNG size: {} bytes", png_data.len());
    
    std::fs::write("examples/final-render-debug.png", &png_data).expect("Failed to write PNG");
    println!("✅ Saved to examples/final-render-debug.png");
    
    println!("\n📈 Stats:");
    println!("  Total nodes: {}", rects.len());
}
