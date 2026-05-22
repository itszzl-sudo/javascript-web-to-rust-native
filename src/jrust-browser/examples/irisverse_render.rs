use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Irisverse Native Renderer ===\n");
    
    let html = fs::read_to_string("C:/Users/a/Documents/comate/irisverse-org/index-bundle.html")
        .expect("Failed to read HTML");
    
    println!("HTML: {} bytes\n", html.len());
    
    let config = BrowserConfig {
        width: 1280,
        height: 800,
        title: "Irisverse".into(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed");
    browser.set_html(&html).expect("Failed to set HTML");
    
    println!("Rendering...\n");
    
    let png = browser.render();
    fs::write("examples/irisverse-render.png", &png).expect("Failed");
    
    println!("Saved: examples/irisverse-render.png");
    println!("Size: {} bytes", png.len());
}
