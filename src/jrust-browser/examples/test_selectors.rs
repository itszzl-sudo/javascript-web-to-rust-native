use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== CSS Match Debug ===\n");
    
    // 简单测试：明确的标签选择器
    let html = r#"
<html>
<head>
<style>
div {
    width: 300px;
    height: 200px;
    background: #ff0000;
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
        title: "Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config.clone()).expect("Failed");
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("Test 1 - Tag selector 'div':");
    for (_id, tag, _x, _y, w, h) in &rects {
        println!("  {} - {:.1}x{:.1}", tag, w, h);
    }
    
    let png = browser.render();
    println!("PNG: {} bytes\n", png.len());
    std::fs::write("examples/test-tag.png", &png).unwrap();
    
    // 测试类选择器
    let html2 = r#"
<html>
<head>
<style>
.box {
    width: 300px;
    height: 200px;
    background: #00ff00;
}
</style>
</head>
<body>
    <div class="box"></div>
</body>
</html>
"#;
    
    let mut browser2 = BrowserInstance::new(config).expect("Failed");
    browser2.set_html(html2).unwrap();
    
    let rects2 = browser2.all_rects();
    println!("Test 2 - Class selector '.box':");
    for (_id, tag, _x, _y, w, h) in &rects2 {
        println!("  {} - {:.1}x{:.1}", tag, w, h);
    }
    
    let png2 = browser2.render();
    println!("PNG: {} bytes", png2.len());
    std::fs::write("examples/test-class.png", &png2).unwrap();
}
