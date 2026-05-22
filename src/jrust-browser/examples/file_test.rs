use jrust_browser::{BrowserConfig, BrowserInstance};

fn main() {
    println!("=== 文件操作测试 ===\n");

    let config = BrowserConfig {
        width: 800,
        height: 600,
        title: "File Test".to_string(),
        enable_js: false,
        enable_gui: false,
    };

    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    println!("✅ Browser instance created\n");

    println!("1. 测试文件下载");
    let url = "https://httpbin.org/robots.txt";
    let path = "test_robots.txt";
    
    match browser.download_file(url, path) {
        Ok(size) => {
            println!("✅ 下载成功! 大小: {} bytes", size);
            
            println!("\n2. 测试文件读取");
            match browser.read_file(path) {
                Ok(content) => {
                    let text = String::from_utf8_lossy(&content);
                    println!("✅ 读取成功! 内容:\n{}", text);
                }
                Err(e) => println!("❌ 读取失败: {}", e),
            }
        }
        Err(e) => println!("❌ 下载失败: {}", e),
    }

    println!("\n3. 测试文件写入");
    let test_data = b"Hello from jrust-browser!\nThis is a test file.";
    match browser.write_file("test_output.txt", test_data) {
        Ok(()) => {
            println!("✅ 写入成功!");
            
            match browser.read_file("test_output.txt") {
                Ok(content) => {
                    let text = String::from_utf8_lossy(&content);
                    println!("验证内容: {}", text);
                }
                Err(e) => println!("❌ 验证失败: {}", e),
            }
        }
        Err(e) => println!("❌ 写入失败: {}", e),
    }

    println!("\n=== 测试完成 ===");
}
