//! 测试表单事件传递

use jrust_browser::{BrowserConfig, BrowserInstance};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    println!("=== 表单事件测试 ===\n");
    
    let config = BrowserConfig::new(800, 600)
        .with_title("Form Test".to_string())
        .with_js(false)
        .with_gui(false);
    
    let mut browser = BrowserInstance::new(config).unwrap();
    
    // 设置包含表单的 HTML
    let html = r#"<!DOCTYPE html>
<html>
<body>
    <form id="login-form">
        <input type="text" name="username" value="testuser">
        <input type="password" name="password" value="testpass">
        <input type="hidden" name="token" value="abc123">
        <textarea name="comment">Hello World</textarea>
        <button type="submit">Submit</button>
    </form>
</body>
</html>"#;
    
    browser.set_html(html).unwrap();
    
    // 查询表单
    let form_id = browser.query("#login-form");
    println!("表单 ID: {:?}\n", form_id);
    
    // 获取表单标签名
    if let Some(id) = form_id {
        let tag = browser.tag_name(id);
        println!("标签名: {:?}\n", tag);
    }
    
    // 查询所有 input
    let inputs = browser.query_all("input");
    println!("Input 元素数量: {}\n", inputs.len());
    
    // 检查父节点
    if !inputs.is_empty() {
        let parent = browser.parent_node(inputs[0]);
        println!("第一个 input 的父节点: {:?}\n", parent);
    }
    
    // 注册表单提交处理器
    let form_submitted = Arc::new(AtomicBool::new(false));
    browser.on_form_submit("#login-form", {
        let form_submitted = form_submitted.clone();
        move |fields| {
            println!("✅ 表单提交事件触发!");
            println!("字段数量: {}", fields.len());
            for (key, value) in &fields {
                println!("  {} = {}", key, value);
            }
            form_submitted.store(true, Ordering::SeqCst);
        }
    });
    
    // 触发表单提交
    browser.handle_form_submit("#login-form");
    
    println!("\n表单是否提交: {}\n", form_submitted.load(Ordering::SeqCst));
    
    // 测试 hit_test
    if let Some((node_id, tag, x, y, w, h)) = browser.hit_test(50.0, 50.0) {
        println!("Hit test 结果:");
        println!("  节点 ID: {}", node_id);
        println!("  标签名: {}", tag);
        println!("  位置: ({}, {}) 大小: {}x{}\n", x, y, w, h);
    } else {
        println!("Hit test: 未命中元素\n");
    }
    
    // 测试 clear_css
    browser.set_css("button { color: red; }");
    println!("设置 CSS: button {{ color: red; }}");
    browser.clear_css();
    println!("清除所有自定义 CSS\n");
    
    println!("=== 测试完成 ===");
}
