use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== 检查背景色 ===\n");
    
    // css-test (成功) - 检查CSS
    println!("1. css-test.html CSS:");
    let html1 = fs::read_to_string("examples/css-test.html").unwrap();
    for line in html1.lines() {
        if line.contains("background") && !line.trim().starts_with("/*") {
            println!("  {}", line.trim());
        }
    }
    
    // simple-demo (白屏?) - 检查CSS
    println!("\n2. simple-demo.html CSS:");
    let html2 = fs::read_to_string("examples/simple-demo.html").unwrap();
    for line in html2.lines() {
        if line.contains("background") && !line.trim().starts_with("/*") {
            println!("  {}", line.trim());
        }
    }
    
    // final-demo (白屏?) - 检查CSS  
    println!("\n3. final-demo.html CSS:");
    let html3 = fs::read_to_string("examples/final-demo.html").unwrap();
    for line in html3.lines() {
        if line.contains("background") && !line.trim().starts_with("/*") {
            println!("  {}", line.trim());
        }
    }
    
    // 测试渲染
    println!("\n=== 实际渲染测试 ===\n");
    
    // 用一个简单的例子测试
    let test_html = r#"
<html>
<head>
<style>
.box {
    width: 200px;
    height: 200px;
    background: #ff0000;
}
</style>
</head>
<body>
    <div class="box"></div>
</body>
</html>
"#;
    
    let mut b = BrowserInstance::new(BrowserConfig {
        width: 400, height: 400, title: "T".into(), enable_js: false, enable_gui: false
    }).unwrap();
    b.set_html(test_html).unwrap();
    let rects = b.all_rects();
    
    println!("Test div:");
    for (_, tag, _, _, w, h) in &rects {
        if *w > 0.0 && *h > 0.0 {
            println!("  {} - {:.1}x{:.1}", tag, w, h);
        }
    }
    
    let png = b.render();
    println!("\nPNG size: {} bytes", png.len());
    fs::write("examples/test-simple-box.png", &png).unwrap();
}
