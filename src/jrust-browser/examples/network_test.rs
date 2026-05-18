//! 测试网络请求功能

use jrust_browser::{BrowserConfig, BrowserInstance, HttpResponse};

fn main() {
    println!("=== 网络请求测试 ===\n");
    
    let config = BrowserConfig::new(800, 600)
        .with_title("Network Test".to_string())
        .with_js(false)
        .with_gui(false);
    
    let mut browser = BrowserInstance::new(config).unwrap();
    
    // 测试 HTTP GET
    println!("1. 测试 HTTP GET 请求\n");
    match browser.http_get("https://httpbin.org/get") {
        Ok(response) => {
            println!("✅ GET 请求成功!");
            println!("状态码: {}", response.status);
            println!("最终 URL: {}", response.final_url);
            println!("响应体大小: {} bytes", response.body.len());
            
            let body_str = String::from_utf8_lossy(&response.body);
            println!("响应体前 200 字符:\n{}\n", &body_str[..200.min(body_str.len())]);
        }
        Err(e) => {
            println!("❌ GET 请求失败: {}\n", e);
        }
    }
    
    // 测试导航
    println!("2. 测试导航\n");
    match browser.navigate("https://example.org") {
        Ok(()) => {
            println!("✅ 导航成功!");
            println!("当前 URL: {}\n", browser.current_url());
            
            // 渲染
            let png = browser.render();
            println!("渲染结果: {} bytes\n", png.len());
            
            // 查询标题
            if let Some(title_id) = browser.query("title") {
                if let Some(title) = browser.text(title_id) {
                    println!("页面标题: {}\n", title);
                }
            }
        }
        Err(e) => {
            println!("❌ 导航失败: {}\n", e);
        }
    }
    
    // 测试 HTTP POST
    println!("3. 测试 HTTP POST 请求\n");
    let post_body = r#"{"name":"test","value":123}"#;
    match browser.http_post(
        "https://httpbin.org/post",
        post_body.as_bytes(),
        "application/json"
    ) {
        Ok(response) => {
            println!("✅ POST 请求成功!");
            println!("状态码: {}", response.status);
            println!("响应体大小: {} bytes", response.body.len());
            
            let body_str = String::from_utf8_lossy(&response.body);
            println!("响应体前 200 字符:\n{}\n", &body_str[..200.min(body_str.len())]);
        }
        Err(e) => {
            println!("❌ POST 请求失败: {}\n", e);
        }
    }
    
    println!("=== 测试完成 ===");
}
