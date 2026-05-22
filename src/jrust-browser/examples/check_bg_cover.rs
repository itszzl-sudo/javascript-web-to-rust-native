use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 检查背景色覆盖 ===\n");
    
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
    
    // 检查每个元素的背景色
    println!("需要检查每个元素的背景色:");
    println!("但 all_rects 不返回背景色信息\n");
    
    println!("解决方案:");
    println!("1. 渲染时应该按Z顺序（子元素在父元素上面）");
    println!("2. 或者子元素应该覆盖父元素\n");
    
    let png = browser.render();
    println!("PNG: {} bytes\n", png.len());
    
    // 测试2：移除body背景
    let html2 = r#"
<html>
<head>
<style>
body {
    background: transparent;
}
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
    
    let mut browser2 = BrowserInstance::new(BrowserConfig {
        width: 400,
        height: 400,
        title: "T".into(),
        enable_js: false,
        enable_gui: false
    }).unwrap();
    
    browser2.set_html(html2).unwrap();
    let png2 = browser2.render();
    println!("PNG (transparent body): {} bytes", png2.len());
    std::fs::write("examples/red-transparent-bg.png", &png2).unwrap();
}
