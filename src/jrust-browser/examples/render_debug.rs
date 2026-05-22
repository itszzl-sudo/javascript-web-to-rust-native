use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 渲染调试 ===\n");
    
    let html = r#"
<html>
<head>
<style>
.red {
    width: 300px;
    height: 300px;
    background: #ff0000;
}
.green {
    width: 200px;
    height: 200px;
    background: #00ff00;
}
.blue {
    width: 100px;
    height: 100px;
    background: #0000ff;
}
</style>
</head>
<body>
    <div class="red"></div>
    <div class="green"></div>
    <div class="blue"></div>
</body>
</html>
"#;
    
    let mut browser = BrowserInstance::new(BrowserConfig {
        width: 400,
        height: 700,
        title: "Debug".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("将渲染的元素:");
    for (id, tag, x, y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  [{}] {} at ({:.0}, {:.0}) size {:.0}x{:.0}", 
                id, tag, x, y, w, h);
        }
    }
    
    println!("\n渲染顺序（按Y坐标）:");
    let mut sorted: Vec<_> = rects.iter().filter(|(_, _, _, _, w, h)| *w > 0.0 && *h > 0.0).collect();
    sorted.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    
    for (id, tag, x, y, w, h) in sorted {
        println!("  {} at y={:.0}", tag, y);
    }
    
    let png = browser.render();
    println!("\nPNG: {} bytes", png.len());
    std::fs::write("examples/render-debug.png", &png).unwrap();
}
