use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== window.open 监听测试 ===\n");

    let config = BrowserConfig {
        width: 800,
        height: 600,
        title: "Window Open Test".to_string(),
        headless: true,
    };

    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser instance created\n");

    println!("1. 注册 window.open 处理器（自动下载附件）");
    browser.on_window_open(|url| {
        println!("window.open 调用: {}", url);

        if url.contains("attachment") || url.contains("/download/") {
            println!("  → 检测到附件下载，开始下载...");
            true
        } else {
            println!("  → 非附件链接，忽略");
            false
        }
    });

    println!("\n2. 测试 window.open 处理");

    let test_urls = vec![
        "https://example.com/attachment/file.pdf",
        "https://example.com/download/document.docx",
        "https://example.com/page.html",
        "https://google.com",
    ];

    for url in test_urls {
        println!("\n处理: {}", url);
        let handled = browser.handle_window_open(url);
        println!("结果: {}", if handled { "已处理" } else { "已忽略" });
    }

    println!("\n=== 测试完成 ===");
}
