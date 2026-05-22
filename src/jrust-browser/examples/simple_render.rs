use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Simple CSS Demo (Compatible) ===\n");
    
    let config = BrowserConfig {
        width: 800,
        height: 700,
        title: "Simple Demo".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    
    let html = fs::read_to_string("examples/simple-demo.html")
        .expect("Failed to read simple-demo.html");
    
    browser.set_html(&html).unwrap();
    println!("✅ HTML loaded\n");
    
    let rects = browser.all_rects();
    println!("📊 Layout nodes: {}", rects.len());
    
    for (id, tag, x, y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  {} - pos: ({:.1}, {:.1}) size: ({:.1}x{:.1})", tag, x, y, w, h);
        }
    }
    
    let png = browser.render();
    std::fs::write("examples/simple-render.png", &png).unwrap();
    println!("\n✅ PNG saved: {} bytes", png.len());
}
