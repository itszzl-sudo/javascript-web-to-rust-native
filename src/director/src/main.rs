use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let mut input_file: Option<String> = None;
    let mut output_name = "app".to_string();
    let mut output_dir = "dist".to_string();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                i += 1;
                if i < args.len() {
                    input_file = Some(args[i].clone());
                }
            }
            "--name" | "-n" => {
                i += 1;
                if i < args.len() {
                    output_name = args[i].clone();
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_dir = args[i].clone();
                }
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            _ => {
                if input_file.is_none() {
                    input_file = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }
    
    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("❌ 错误: 未指定输入文件");
            eprintln!("使用 --input <file> 指定");
            std::process::exit(1);
        }
    };
    
    println!("=========================================");
    println!("  Director CLI - JS → Native");
    println!("=========================================");
    println!("输入文件: {}", input_file);
    println!("输出名称: {}", output_name);
    println!("输出目录: {}", output_dir);
    println!();
    
    let js_content = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ 读取文件失败: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("JS 代码大小: {} 字节", js_content.len());
    println!();
    
    let director = jrust_runtime::director::Director::new();
    
    match director.compile_to_native(&js_content, &output_name) {
        Ok(obj_path) => {
            println!();
            println!("✅ 编译成功！");
            println!("目标文件: {:?}", obj_path);
            
            let output_path = PathBuf::from(&output_dir).join(&output_name);
            if let Err(e) = fs::create_dir_all(&output_path) {
                eprintln!("⚠️  创建输出目录失败: {}", e);
            }
            
            let final_obj = output_path.join(format!("{}.obj", output_name));
            if let Err(e) = fs::copy(&obj_path, &final_obj) {
                eprintln!("⚠️  复制目标文件失败: {}", e);
            } else {
                println!("输出位置: {:?}", final_obj);
            }
            
            println!();
            println!("下一步:");
            println!("  compiler.link_with_lib(&obj, \"rust-browser.lib\", \"{}.exe\")", output_name);
        }
        Err(e) => {
            eprintln!();
            eprintln!("❌ 编译失败: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Director CLI - Vue 项目 → Native 可执行文件编译器");
    println!();
    println!("用法:");
    println!("  director --input <js-file> [options]");
    println!("  director <js-file> [options]");
    println!();
    println!("选项:");
    println!("  -i, --input <file>   输入 JS 文件路径");
    println!("  -n, --name <name>    输出名称 (默认: app)");
    println!("  -o, --output <dir>   输出目录 (默认: dist)");
    println!("  -h, --help           显示帮助信息");
    println!();
    println!("示例:");
    println!("  director --input ./dist/assets/index.js --name my-app");
    println!("  director ./app.js -n my-app -o ./output");
    println!();
    println!("流程:");
    println!("  1. 读取 JS 文件 (已优化的 Vue 项目输出)");
    println!("  2. JS → Cranelift IR (jrust-translator)");
    println!("  3. IR → 目标文件 (cranelift-compiler)");
    println!("  4. 输出 .obj 文件 (待链接 rust-browser.lib)");
}
