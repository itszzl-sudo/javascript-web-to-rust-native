use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== 最终验证 ===\n");
    
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
        width: 350,
        height: 700,
        title: "Final".into(),
        enable_js: false,
        enable_gui: false,
    }).unwrap();
    
    browser.set_html(html).unwrap();
    
    let rects = browser.all_rects();
    println!("布局结果:");
    for (id, tag, x, y, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 && tag != "html" && tag != "head" && tag != "body" && !tag.is_empty() {
            println!("  {} - 位置({:.0}, {:.0}) 尺寸{:.0}x{:.0}", tag, x, y, w, h);
        }
    }
    
    let png = browser.render();
    println!("\n渲染完成:");
    println!("  PNG 大小: {} bytes", png.len());
    println!("  文件: examples/final-verify.png");
    
    fs::write("examples/final-verify.png", &png).unwrap();
    
    println!("\n请检查 examples/final-verify.png");
    println!("应该看到:");
    println!("  红色方块 300x300");
    println!("  绿色方块 200x200");
    println!("  蓝色方块 100x100");
}
