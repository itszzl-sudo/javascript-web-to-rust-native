use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 详细布局检查 ===\n");
    
    let html = r#"
<html>
<head>
<style>
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
        title: "Test".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("所有节点:");
    for (id, tag, x, y, w, h) in &rects {
        println!("  {} (id:{}) at ({:.1}, {:.1}) size {:.1}x{:.1}", 
            tag, id, x, y, w, h);
    }
    
    // 专门检查红色方块
    println!("\n期望:");
    println!("  红色: 300x300");
    println!("  绿色: 200x200");
    println!("  蓝色: 100x100");
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/layout-check.png", &png).unwrap();
}
