use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    let html = fs::read_to_string("examples/final-demo.html").unwrap();
    
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
    
    // 统计有内容的节点
    let with_size = rects.iter().filter(|(_, _, _, _, w, h)| *w > 0.0 && *h > 0.0).count();
    let with_bg = rects.iter().filter(|(_, _, _, _, _, _)| true).count();
    
    println!("Total: {}, With size: {}", rects.len(), with_size);
    
    // 显示有尺寸的节点
    for (_id, tag, _x, _y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  {} - {:.1}x{:.1}", tag, w, h);
        }
    }
    
    let png = browser.render();
    println!("\nPNG size: {} bytes", png.len());
    
    // 检查PNG是否真的是白色
    let non_white = png.iter().filter(|&&b| b != 255 && b != 0).count();
    println!("Non-white/black pixels: {}", non_white);
    
    std::fs::write("examples/final-check.png", &png).unwrap();
}
