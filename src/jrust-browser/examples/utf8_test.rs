use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== UTF-8 Test ===\n");
    
    let html = r#"
<!DOCTYPE html>
<html>
<head>
<style>
.box {
    width: 300px;
    height: 200px;
    background: #6366f1;
}
</style>
</head>
<body>
    <div class="box">🎉 Hello 你好</div>
</body>
</html>
"#;
    
    let mut browser = BrowserInstance::new(BrowserConfig {
        width: 400,
        height: 300,
        title: "UTF-8".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("Nodes:");
    for (_, tag, x, y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  {} - {:.1}x{:.1}", tag, w, h);
        }
    }
    
    let png = browser.render();
    std::fs::write("examples/utf8-test.png", &png).unwrap();
    println!("\nPNG: {} bytes", png.len());
}
