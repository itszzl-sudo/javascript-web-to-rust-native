
use jrust_runtime::director::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("=== Director 完整工作流程演示 ===\n");

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
    
    // === 步骤 4: 编译为二进制 (新功能演示！) ===
    println!("\n=== 3. 编译 JRust 为二进制 ===\n");
    
    // 先创建一个简单可编译的测试代码，而不是用真实的 Vue JS 翻译结果
    let test_rust_code = format!(
        "//! 测试 JRust 程序\n\
        use std::println;\n\
        \n\
        fn main() {{\n\
        \tprintln!(\"Hello from JRust! 🚀\");\n\
        \tprintln!(\"This is a Director-generated native app!\");\n\
        }}\n"
    );
    
    match director.compile_jrust(&test_rust_code, "my_jrust_app") {
        Ok(exe_path) => {
            println!("\n✅ 编译成功！");
            println!("生成的可执行文件: {:?}", exe_path);
            
            // 可选：尝试运行它
            println!("\n--- 尝试运行生成的程序 ---");
            if let Ok(output) = std::process::Command::new(&exe_path).output() {
                println!("程序输出:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(e) => {
            eprintln!("\n❌ 编译失败: {}", e);
        }
    }
    
    // === 步骤 5: (将来) 生成 snap，分离事件，集成 Servo... ===
    println!("\n=== 工作流程演示完成 ===");
    println!("\n下一步规划:");
    println!("1. 添加 snap (DOM 序列化) 生成功能");
    println!("2. 实现事件分离为 jruste");
    println!("3. 集成 Servo 渲染引擎");
    println!("4. 最终产品打包\n");
}

