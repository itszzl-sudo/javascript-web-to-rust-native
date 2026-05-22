use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 红色方块测试 ===\n");
    
    let html = r#"
<html>
<head>
<style>
body {
    margin: 0;
    background: #ffffff;
}
.red-box {
    width: 300px;
    height: 300px;
    background: #ff0000;
}
.green-box {
    width: 200px;
    height: 200px;
    background: #00ff00;
}
.blue-box {
    width: 100px;
    height: 100px;
    background: #0000ff;
}
</style>
</head>
<body>
    <div class="red-box"></div>
    <div class="green-box"></div>
    <div class="blue-box"></div>
</body>
</html>
"#;
    
    let mut browser = BrowserInstance::new(BrowserConfig {
        width: 400,
        height: 700,
        title: "Color Test".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("节点:");
    for (_, tag, x, y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  {} at ({:.1}, {:.1}) size {:.1}x{:.1}", tag, x, y, w, h);
        }
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/color-test.png", &png).unwrap();
    println!("已保存: examples/color-test.png");
}
