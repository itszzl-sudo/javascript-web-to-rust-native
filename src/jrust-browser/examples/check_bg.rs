use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    let html = fs::read_to_string("examples/final-demo.html").unwrap();
    
    // 检查 HTML 中的背景色
    println!("HTML backgrounds:");
    for line in html.lines() {
        if line.contains("background:") {
            println!("  {}", line.trim());
        }
    }
    
    let config = BrowserConfig {
        width: 1024,
        height: 768,
        title: "Final".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).unwrap();
    browser.set_html(&html).unwrap();
    
    let rects = browser.all_rects();
    
    println!("\nLayout nodes with background:");
    for (id, tag, x, y, w, h) in &rects {
        // 获取背景色 - 需要直接访问 layout_tree
        println!("  {} (id:{}) - {:.1}x{:.1} at ({:.1}, {:.1})", 
            tag, id, w, h, x, y);
    }
}
