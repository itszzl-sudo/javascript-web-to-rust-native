//! 测试事件传递到 jrust-runtime

use jrust_browser::{BrowserConfig, BrowserInstance};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn main() {
    println!("=== 事件传递测试 ===\n");
    
    // 创建浏览器实例
    let config = BrowserConfig::new(800, 600)
        .with_title("Event Test".to_string())
        .with_js(false)
        .with_gui(false);
    
    let mut browser = BrowserInstance::new(config).unwrap();
    
    // 设置 HTML
    let html = r#"<!DOCTYPE html>
<html>
<body>
    <button id="btn" style="width: 100px; height: 50px;">Click Me</button>
    <div id="result">Not clicked</div>
</body>
</html>"#;
    
    browser.set_html(html).unwrap();
    
    // 渲染
    let png = browser.render();
    println!("初始渲染: {} bytes\n", png.len());
    
    // 查找按钮
    let btn_id = browser.query("#btn");
    println!("按钮 ID: {:?}\n", btn_id);
    
    // 获取按钮位置
    let rect = browser.get_rect("#btn");
    println!("按钮位置: {:?}\n", rect);
    
    // 注册点击事件
    let click_count = Arc::new(AtomicI32::new(0));
    browser.on_click("#btn", {
        let click_count = click_count.clone();
        move |x, y| {
            let count = click_count.fetch_add(1, Ordering::SeqCst) + 1;
            println!("✅ 按钮被点击! 位置: ({}, {}), 次数: {}", x, y, count);
        }
    });
    
    // 模拟点击
    if let Some((x, y, w, h)) = rect {
        let click_x = x + w / 2.0;
        let click_y = y + h / 2.0;
        
        println!("模拟点击位置: ({}, {})\n", click_x, click_y);
        
        let handled = browser.handle_click(click_x, click_y);
        println!("事件是否处理: {}\n", handled);
    }
    
    println!("总点击次数: {}\n", click_count.load(Ordering::SeqCst));
    
    // 测试属性设置
    if let Some(id) = btn_id {
        browser.set_attr(id, "data-clicked", "true").unwrap();
        let attr = browser.get_attr(id, "data-clicked");
        println!("按钮 data-clicked 属性: {:?}\n", attr);
    }
    
    // 测试文本内容
    if let Some(id) = btn_id {
        let text = browser.text(id);
        println!("按钮文本: {:?}\n", text);
    }
    
    println!("=== 测试完成 ===");
}
