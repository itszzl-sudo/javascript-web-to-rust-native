use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== CSS Selector Match Test ===\n");
    
    let html = r#"
<!DOCTYPE html>
<html>
<head>
<style>
.container {
    width: 800px;
    height: 700px;
    background: #ffffff;
}
.counter-card {
    width: 760px;
    height: 200px;
    background: #f093fb;
}
.stat-card1 {
    width: 240px;
    height: 100px;
    background: #667eea;
}
</style>
</head>
<body>
    <div class="container">
        <div class="counter-card"></div>
        <div class="stat-card1"></div>
    </div>
</body>
</html>
"#;
    
    let config = BrowserConfig {
        width: 800,
        height: 700,
        title: "Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed");
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("Nodes:\n");
    
    for (id, tag, x, y, w, h) in &rects {
        if *w > 0.0 || *h > 0.0 {
            println!("✅ {} ({}) - ({:.1}, {:.1}) {:.1}x{:.1}", tag, id, x, y, w, h);
        } else {
            println!("❌ {} ({}) - ({:.1}, {:.1}) {:.1}x{:.1}", tag, id, x, y, w, h);
        }
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/selector-test.png", &png).unwrap();
}
