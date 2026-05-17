
use jrust_runtime::director::*;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("=== Director 完整工作流程演示 ===\n");

    // === 步骤 1: 初始化 Director ===
    let project_dir = PathBuf::from("src/vue-compile-demo");
    let director = Director::with_workdir(project_dir.clone());
    
    // === 步骤 2: 读取已构建的 JS (因为 npm 路径问题，直接读取现有文件) ===
    println!("=== 1. 读取已构建的 JS 文件 ===\n");
    
    let dist_path = project_dir.join("dist").join("assets");
    let mut js_content = String::new();
    let mut js_path = None;
    
    if let Ok(entries) = fs::read_dir(&dist_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "js" {
                    println!("找到 JS 文件: {:?}", path);
                    js_path = Some(path.clone());
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
    
    // === 步骤 4: (将来) 编译为二进制，生成 snap，分离事件... ===
    println!("\n=== 工作流程演示完成 ===");
    println!("\n下一步规划:");
    println!("1. 完整集成 jrust-translator (当前是模拟)");
    println!("2. 添加 snap (DOM 序列化) 生成功能");
    println!("3. 实现事件分离为 jruste");
    println!("4. 集成 Servo 渲染引擎");
    println!("5. 最终产品打包\n");
}

