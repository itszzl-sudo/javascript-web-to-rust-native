use jrust_browser::{BrowserConfig, BrowserInstance};
use std::fs;

fn main() {
    println!("=== 检查所有渲染结果 ===\n");
    
    // 测试 1: css-render-output (成功的)
    println!("1. css-render-output (已知成功):");
    let html1 = fs::read_to_string("examples/css-test.html").unwrap();
    let mut b1 = BrowserInstance::new(BrowserConfig {
        width: 600, height: 600, title: "T1".into(), enable_js: false, enable_gui: false
    }).unwrap();
    b1.set_html(&html1).unwrap();
    let r1 = b1.all_rects();
    let mut colored1 = 0;
    for (_, tag, _, _, w, h) in &r1 {
        if *w > 0.0 && *h > 0.0 { colored1 += 1; }
    }
    println!("  有尺寸节点: {}/{}\n", colored1, r1.len());
    
    // 测试 2: simple-render (白屏?)
    println!("2. simple-render (白屏?):");
    let html2 = fs::read_to_string("examples/simple-demo.html").unwrap();
    println!("  HTML 内容预览:");
    for (i, line) in html2.lines().take(15).enumerate() {
        println!("    {}: {}", i, line);
    }
    
    let mut b2 = BrowserInstance::new(BrowserConfig {
        width: 800, height: 700, title: "T2".into(), enable_js: false, enable_gui: false
    }).unwrap();
    b2.set_html(&html2).unwrap();
    let r2 = b2.all_rects();
    let mut colored2 = 0;
    for (_, tag, _, _, w, h) in &r2 {
        if *w > 0.0 && *h > 0.0 { 
            colored2 += 1;
            println!("  ✅ {} - {:.1}x{:.1}", tag, w, h);
        }
    }
    println!("  有尺寸节点: {}/{}\n", colored2, r2.len());
    
    // 测试 3: final-render (白屏?)
    println!("3. final-render (白屏?):");
    let html3 = fs::read_to_string("examples/final-demo.html").unwrap();
    let mut b3 = BrowserInstance::new(BrowserConfig {
        width: 1024, height: 768, title: "T3".into(), enable_js: false, enable_gui: false
    }).unwrap();
    b3.set_html(&html3).unwrap();
    let r3 = b3.all_rects();
    let mut colored3 = 0;
    for (_, tag, _, _, w, h) in &r3 {
        if *w > 0.0 && *h > 0.0 { 
            colored3 += 1;
            println!("  ✅ {} - {:.1}x{:.1}", tag, w, h);
        }
    }
    println!("  有尺寸节点: {}/{}\n", colored3, r3.len());
    
    // 渲染并检查
    let p1 = b1.render();
    let p2 = b2.render();
    let p3 = b3.render();
    
    // 检查PNG中的非白色像素
    fn count_colored(png: &[u8]) -> usize {
        // PNG 是压缩的，无法直接检查像素
        // 但可以通过文件大小判断
        png.len()
    }
    
    println!("PNG 大小:");
    println!("  css-render: {} bytes", count_colored(&p1));
    println!("  simple: {} bytes", count_colored(&p2));
    println!("  final: {} bytes", count_colored(&p3));
}
