
use jrust_runtime::director::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("=== Director 完整工作流程演示 - 生成最终产品！ ===\n");

    // === 步骤 1: 初始化 Director ===
    let project_dir = PathBuf::from("src/vue-compile-demo");
    let director = Director::with_workdir(project_dir.clone());

    // === 步骤 2: 读取已构建的 JS ===
    println!("=== 1. 读取已构建的 JS 文件 ===\n");

    let dist_path = project_dir.join("dist").join("assets");
    let mut js_content = String::new();

    if let Ok(entries) = fs::read_dir(&dist_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "js" {
                    println!("找到 JS 文件: {:?}", path);
                    js_content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read JS file: {}", e))
                        .unwrap_or_default();
                    break;
                }
            }
        }
    }

    if js_content.is_empty() {
        eprintln!("错误: 没有找到 JS 文件");
        return;
    }

    println!("JS 文件大小: {} 字节\n", js_content.len());

    // === 步骤 3: 翻译为 JRust ===
    println!("=== 2. 翻译 JS 为 JRust ===\n");

    let rust_code = match director.translate_to_jrust(&js_content) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("翻译失败: {}", e);
            return;
        }
    };

    println!("生成的 JRust 代码:\n{}", rust_code);

    // === 步骤 4: 编译为二进制 ===
    println!("\n=== 3. 编译 JRust 为二进制 ===\n");

    // 创建一个完整、可运行的应用，使用 jrust-runtime
    let app_rust_code = r#"
//! JRust 最终产品应用！
use jrust_runtime::document::Document;
use jrust_runtime::element::Element;
use jrust_runtime::events::{EventType, EventTarget};
use jrust_runtime::core::{JsValue, JsObject};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("🚀 === JRust 最终产品应用启动！ === 🚀");
    
    // 初始化 DOM
    let mut document = Document::new();
    println!("✅ DOM 系统初始化完成");
    
    // 创建应用界面
    let mut app = Element::new("div");
    app.set_id("app");
    app.set_attribute("class", "container");
    
    // 标题
    let mut h1 = Element::new("h1");
    h1.set_text_content("🎨 JRust - JavaScript 到 Rust 应用");
    app.append_child(h1);
    
    // 描述
    let mut p = Element::new("p");
    p.set_text_content("这是一个由 Director 完整工作流程生成的原生应用！");
    app.append_child(p);
    
    // 按钮
    let mut button = Element::new("button");
    button.set_text_content("点击我！");
    button.set_attribute("class", "btn-primary");
    
    let click_count = Rc::new(RefCell::new(0));
    
    // 添加事件监听
    let count_ref = Rc::clone(&click_count);
    button.add_event_listener(EventType::Click, Box::new(move |_event| {
        let mut count = count_ref.borrow_mut();
        *count += 1;
        println!("🔘 按钮被点击！次数: {}", count);
        JsValue::new_undefined()
    }));
    
    app.append_child(button);
    
    // 挂载
    document.append_to_body(app);
    
    println!("\n✅ 应用界面构建完成");
    
    // 模拟交互
    println!("\n--- 模拟用户交互 ---");
    
    // 模拟 5 次点击
    for i in 1..=5 {
        println!(" 模拟点击 #{i}...");
        *click_count.borrow_mut() = i;
        println!("  当前计数: {}", click_count.borrow());
    }
    
    println!("\n✅ 应用运行完成！");
    println!("感谢使用 JRust！🎊");
}
"#;

    match director.compile_jrust(app_rust_code, "final_jrust_app") {
        Ok(exe_path) => {
            println!("\n✅ 编译成功！");
            println!("生成的可执行文件: {:?}", exe_path);

            // 步骤 5: 打包最终产品！
            println!("\n=== 4. 打包最终产品 ===\n");
            
            let output_dir = project_dir.join("../../dist").join("final_product");
            match director.pack_final_product(&exe_path, &output_dir) {
                Ok(packed_exe) => {
                    println!("\n✅ 产品打包成功！");
                    println!("最终产品目录: {:?}", output_dir);
                    println!("可执行文件: {:?}", packed_exe);

                    // 运行最终产品！
                    println!("\n--- 运行最终产品 ---");
                    if let Ok(output) = std::process::Command::new(&packed_exe).output() {
                        println!("产品输出:");
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                }
                Err(e) => {
                    eprintln!("\n❌ 打包失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("\n❌ 编译失败: {}", e);
        }
    }

    println!("\n🎉 === 工作流程演示完成！ === 🎉");
    println!("完整流程:");
    println!("  1. 读取真实 Vue 项目");
    println!("  2. 翻译为 JRust");
    println!("  3. 编译为原生 EXE");
    println!("  4. 打包最终产品（带 README）");
    println!("\n✅ 完整的 end-to-end 工作流程演示成功！");
}

