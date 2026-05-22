use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== Selector Build Debug ===\n");
    
    let html = r#"
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
h1 {
    width: 800px;
    height: 60px;
    background: #333333;
}
</style>
</head>
<body>
    <div class="container">
        <h1></h1>
        <div class="counter-card"></div>
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
    println!("Layout result:");
    for (_id, tag, _x, _y, w, h) in &rects {
        let status = if *w > 0.0 && *h > 0.0 { "✅" } else { "❌" };
        println!("{} {} - {:.1}x{:.1}", status, tag, w, h);
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/selector-build-test.png", &png).unwrap();
}
