use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 检查 all_rects 返回值 ===\n");
    
    let html = r#"
<html>
<head>
<style>
.red {
    width: 300px;
    height: 300px;
    background: #ff0000;
}
</style>
</head>
<body>
    <div class="red"></div>
</body>
</html>
"#;
    
    let mut browser = BrowserInstance::new(BrowserConfig {
        width: 400,
        height: 400,
        title: "T".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("all_rects 返回:");
    for (id, tag, x, y, w, h) in &rects {
        println!("  id={}, tag='{}', x={:.1}, y={:.1}, w={:.1}, h={:.1}", 
            id, tag, x, y, w, h);
    }
    
    // 查找红色方块
    for (id, tag, x, y, w, h) in &rects {
        if *w == 300.0 && *h == 300.0 {
            println!("\n✅ 找到红色方块:");
            println!("  id={}", id);
            println!("  位置: ({:.1}, {:.1})", x, y);
            println!("  尺寸: {:.1}x{:.1}", w, h);
        }
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/red-box-only.png", &png).unwrap();
}
