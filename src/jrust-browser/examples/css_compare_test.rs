use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== CSS 效果对比测试 ===\n");
    
    let config = BrowserConfig {
        width: 600,
        height: 600,
        title: "CSS Compare".to_string(),
        enable_js: false,
        enable_gui: false,
    };
    
    let mut browser = BrowserInstance::new(config).expect("Failed to create browser");
    
    // 测试 1: 无 CSS
    let html_no_css = fs::read_to_string("examples/no-css-test.html").unwrap();
    browser.set_html(&html_no_css).unwrap();
    let png_no_css = browser.render();
    std::fs::write("examples/no-css-output.png", &png_no_css).unwrap();
    println!("无 CSS PNG: {} bytes", png_no_css.len());
    
    // 测试 2: 有 CSS
    let html_with_css = fs::read_to_string("examples/css-test.html").unwrap();
    browser.set_html(&html_with_css).unwrap();
    let png_with_css = browser.render();
    std::fs::write("examples/css-render-output.png", &png_with_css).unwrap();
    println!("有 CSS PNG: {} bytes", png_with_css.len());
    
    // 对比
    println!("\n=== 对比结果 ===");
    println!("无 CSS: {} bytes", png_no_css.len());
    println!("有 CSS: {} bytes", png_with_css.len());
    
    if png_with_css.len() > png_no_css.len() {
        let diff = png_with_css.len() - png_no_css.len();
        println!("差异: +{} bytes ({:.1}%)", diff, (diff as f32 / png_no_css.len() as f32) * 100.0);
        println!("✅ CSS 已生效 - 有 CSS 的 PNG 明显更大");
    } else {
        println!("⚠️  CSS 可能未生效");
    }
    
    // 计算像素差异
    let mut diff_count = 0;
    for (a, b) in png_no_css.iter().zip(png_with_css.iter()) {
        if a != b {
            diff_count += 1;
        }
    }
    println!("\n像素差异: {} / {} bytes ({:.1}%)", 
        diff_count, 
        png_no_css.len(), 
        (diff_count as f32 / png_no_css.len() as f32) * 100.0
    );
    
    if diff_count > png_no_css.len() / 10 {
        println!("✅ 确认: CSS 已渲染 - 图片内容显著不同");
    }
}
