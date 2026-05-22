use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== CSS Debug Test ===\n");
    
    // 简单的HTML+CSS
    let html = r#"
<!DOCTYPE html>
<html>
<head>
<style>
body {
    background: #ff0000;
}
div {
    width: 200px;
    height: 200px;
    background: #00ff00;
}
</style>
</head>
<body>
    <div></div>
</body>
</html>
"#;
    
    let config = BrowserConfig {
        width: 400,
        height: 400,
        title: "CSS Debug".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("Nodes:");
    for (id, tag, x, y, w, h) in &rects {
        println!("  {} - pos: ({:.1}, {:.1}) size: ({:.1}x{:.1})", tag, x, y, w, h);
    }
    
    let png = browser.render();
    std::fs::write("examples/css-debug.png", &png).unwrap();
    println!("\nPNG: {} bytes", png.len());
}
