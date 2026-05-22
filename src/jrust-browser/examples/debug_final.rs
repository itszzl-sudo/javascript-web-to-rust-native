use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Final Render Debug ===\n");
    
    let html = fs::read_to_string("examples/final-demo.html")
        .expect("Failed to read final-demo.html");
    
    println!("HTML:\n{}\n", html);
    
    let config = BrowserConfig {
        width: 1024,
        height: 768,
        title: "Debug".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed");
    browser.set_html(&html).unwrap();
    
    let rects = browser.all_rects();
    println!("Total nodes: {}\n", rects.len());
    
    for (id, tag, x, y, w, h) in &rects {
        println!("{} ({}) - pos: ({:.1}, {:.1}) size: ({:.1}x{:.1})", 
            tag, id, x, y, w, h);
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/final-debug2.png", &png).unwrap();
}
