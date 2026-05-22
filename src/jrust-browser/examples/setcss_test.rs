use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== set_css API 测试 ===\n");
    
    let config = BrowserConfig {
        width: 600,
        height: 600,
        title: "set_css Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    
    // 先设置 HTML
    let html = r#"
<!DOCTYPE html>
<html>
<body>
    <h1>测试标题</h1>
    <div id="box">方块</div>
</body>
</html>
"#;
    browser.set_html(html).unwrap();
    println!("✅ HTML 设置成功");
    
    // 渲染无 CSS 版本
    let png_no_css = browser.render();
    std::fs::write("examples/setcss-before.png", &png_no_css).unwrap();
    println!("无 CSS PNG: {} bytes", png_no_css.len());
    
    // 使用 set_css API 添加 CSS
    let css = r#"
h1 {
    color: red;
    font-size: 48px;
    background: blue;
}
#box {
    width: 200px;
    height: 200px;
    background: #ff6b6b;
    border: 5px solid black;
}
"#;
    browser.set_css(css).unwrap();
    println!("✅ CSS 设置成功");
    
    // 渲染有 CSS 版本
    let png_with_css = browser.render();
    std::fs::write("examples/setcss-after.png", &png_with_css).unwrap();
    println!("有 CSS PNG: {} bytes", png_with_css.len());
    
    // 对比
    println!("\n=== 结果 ===");
    if png_with_css.len() != png_no_css.len() {
        println!("✅ set_css() 有效 - PNG 大小不同");
    } else {
        // 检查像素差异
        let diff: usize = png_no_css.iter().zip(png_with_css.iter())
            .filter(|(a, b)| a != b).count();
        if diff > 0 {
            println!("⚠️  PNG 大小相同但内容不同 (差异: {} bytes)", diff);
        } else {
            println!("❌ CSS 未生效 - set_css() 无效果");
        }
    }
    
    // 测试 set_style
    println!("\n=== 测试 set_style ===");
    browser.set_style("h1", "color", "green").unwrap();
    browser.set_style("#box", "background", "yellow").unwrap();
    let png_styled = browser.render();
    std::fs::write("examples/setcss-styled.png", &png_styled).unwrap();
    println!("set_style PNG: {} bytes", png_styled.len());
}
