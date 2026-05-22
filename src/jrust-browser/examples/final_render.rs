use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Final CSS Rendering Demo ===\n");
    
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
    
    println!("Rendering...");
    let png_data = browser.render();
    println!("✅ Render complete! PNG size: {} bytes", png_data.len());
    
    std::fs::write("examples/final-render.png", &png_data).expect("Failed to write PNG");
    println!("✅ Saved to examples/final-render.png");
    
    println!("\n=== CSS Rendering Success ===");
}
