
use crate::director::jrust_tree::{JsRustId, JsRustInstance, JsRustTree};
use crate::dom::document::Document;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use serde_json;

/// Director - jrust 的指挥中心
pub struct Director {
    jrust_tree: JsRustTree,
    workdir: PathBuf,
}

impl Director {
    pub fn new() -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
            workdir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
    
    pub fn with_workdir(workdir: PathBuf) -> Self {
        Self {
            jrust_tree: JsRustTree::new(),
            workdir,
        }
    }
    
    pub fn add_jrust(&mut self, instance: Box<dyn JsRustInstance>) -> JsRustId {
        let id = self.jrust_tree.create_root(instance);
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        id
    }
    
    pub fn create_child_jrust(&mut self, parent_id: JsRustId, instance: Box<dyn JsRustInstance>) -> Option<JsRustId> {
        let id = self.jrust_tree.create_child(parent_id, instance)?;
        if let Some(node) = self.jrust_tree.get_node_mut(id) {
            node.instance.init();
        }
        Some(id)
    }
    
    pub fn dispatch_event(&mut self) {
        self.jrust_tree.dispatch_event();
    }
    
    // === Phase 1 新功能 ===
    
    /// 执行外部命令
    pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new(command)
            .args(args)
            .current_dir(&self.workdir)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
            
        if !output.status.success() {
            return Err(format!(
                "Command failed with exit code {:?}\nStderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    /// 预处理 Vue 项目
    pub fn preprocess_vue_project(&self, project_path: &str) -> Result<String, String> {
        println!("=== Director: 预处理 Vue 项目 ===\n");
        println!("项目路径: {}", project_path);
        
        // 1. 先执行 npm install (可选，已安装的话可以跳过)
        println!("1. 检查 npm 依赖...");
        let _ = self.execute_command("npm", &["install"]);
        
        // 2. 执行 npm run build
        println!("2. 执行 npm run build...");
        let build_output = self.execute_command("npm", &["run", "build"])?;
        println!("构建输出:\n{}", build_output);
        
        // 3. 查找生成的 JS 文件
        println!("3. 查找生成的 JS 文件...");
        let project_path_buf = PathBuf::from(project_path);
        let dist_path = project_path_buf.join("dist").join("assets");
        
        let mut js_content = String::new();
        if let Ok(entries) = fs::read_dir(&dist_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "js" {
                        println!("找到 JS 文件: {:?}", path);
                        js_content = fs::read_to_string(&path)
                            .map_err(|e| format!("Failed to read JS file: {}", e))?;
                        break;
                    }
                }
            }
        }
        
        if js_content.is_empty() {
            return Err("No JS file found in dist/assets".to_string());
        }
        
        println!("✅ Vue 项目预处理完成\n");
        Ok(js_content)
    }
    
    /// 翻译 JS 代码为 JRust (通过 CLI 调用方式)
    pub fn translate_to_jrust(&self, js_code: &str) -> Result<String, String> {
        println!("=== Director: 翻译 JS 为 JRust ===\n");
        
        println!("输入 JS 代码长度: {} 字节", js_code.len());
        
        // 1. 将 JS 代码写入临时文件
        let temp_js_path = self.workdir.join("temp_input.js");
        fs::write(&temp_js_path, js_code)
            .map_err(|e| format!("写入临时 JS 文件失败: {}", e))?;
        
        println!("临时 JS 文件已创建: {:?}", temp_js_path);
        
        // 2. 检查是否有 jrust-translator 二进制
        let translator_path = self.workdir.join("../../target/release/jrust-translator.exe");
        if !translator_path.exists() {
            println!("未找到 jrust-translator 二进制，使用模拟翻译");
            // 模拟翻译过程
            let rust_code = format!(
                "// 由 Director 翻译的 JRust 代码\n\
                fn main() {{\n\
                \tprintln!(\"Hello from translated JRust!\");\n\
                }}\n"
            );
            // 清理临时文件
            let _ = fs::remove_file(temp_js_path);
            println!("✅ 模拟翻译完成\n");
            return Ok(rust_code);
        }
        
        // 3. 调用 jrust-translator CLI
        println!("正在调用 jrust-translator...");
        let output = Command::new(&translator_path)
            .arg(&temp_js_path)
            .output()
            .map_err(|e| format!("调用 jrust-translator 失败: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "jrust-translator 失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        let rust_code = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 4. 清理临时文件
        let _ = fs::remove_file(temp_js_path);
        
        println!("✅ 真实翻译完成\n");
        Ok(rust_code)
    }
    
    /// 编译 JRust 代码为二进制
    pub fn compile_jrust(&self, rust_code: &str, output_name: &str) -> Result<PathBuf, String> {
        println!("=== Director: 编译 JRust 为二进制 ===\n");

        // 1. 创建临时项目目录
        let temp_project_dir = self.workdir.join("temp_jrust_project");
        let _ = fs::remove_dir_all(&temp_project_dir); // 清理旧的
        fs::create_dir_all(&temp_project_dir)
            .map_err(|e| format!("创建临时项目目录失败: {}", e))?;

        // 2. 创建 Cargo.toml
        let cargo_toml = format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n\
            edition = \"2021\"\n\
            \n\
            [workspace]\n\
            \n\
            [dependencies]\n\
            jrust-runtime = {{ path = \"../../jrust-runtime\" }}\n",
            output_name
        );
        fs::write(temp_project_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("写入 Cargo.toml 失败: {}", e))?;

        // 3. 创建 src/main.rs
        let src_dir = temp_project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("创建 src 目录失败: {}", e))?;
        fs::write(src_dir.join("main.rs"), rust_code)
            .map_err(|e| format!("写入 main.rs 失败: {}", e))?;

        println!("临时项目已创建: {:?}", temp_project_dir);

        // 4. 运行 cargo build
        println!("正在运行 cargo build...");
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_project_dir)
            .output()
            .map_err(|e| format!("cargo build 失败: {}", e))?;

        if !build_output.status.success() {
            return Err(format!(
                "cargo build 失败:\n{}",
                String::from_utf8_lossy(&build_output.stderr)
            ));
        }

        // 5. 查找生成的二进制
        let exe_name = if cfg!(windows) {
            format!("{}.exe", output_name)
        } else {
            output_name.to_string()
        };

        let target_exe = temp_project_dir.join("target/release").join(exe_name);

        if target_exe.exists() {
            println!("✅ 编译完成，二进制文件: {:?}", target_exe);
            Ok(target_exe)
        } else {
            Err("未找到生成的二进制文件".to_string())
        }
    }

    /// 打包最终产品
    pub fn pack_final_product(&self, exe_path: &PathBuf, output_dir: &PathBuf) -> Result<PathBuf, String> {
        println!("=== Director: 打包最终产品 ===\n");

        // 1. 创建输出目录
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("创建输出目录失败: {}", e))?;

        // 2. 复制 exe 到输出目录
        let output_exe = output_dir.join(exe_path.file_name().unwrap());
        fs::copy(exe_path, &output_exe)
            .map_err(|e| format!("复制可执行文件失败: {}", e))?;

        // 3. 创建 README
        let readme = format!(
            "# {}\n\
            \n\
            这是一个由 Director 生成的 JRust 应用！\n\
            \n\
            ## 运行\n\
            直接执行可执行文件即可。\n\
            \n\
            ## 技术栈\n\
            - 输入：真实 Vue 项目\n\
            - 翻译：jrust-translator\n\
            - 运行时：jrust-runtime\n\
            - 打包：Director\n",
            exe_path.file_stem().unwrap().to_str().unwrap()
        );
        fs::write(output_dir.join("README.md"), readme)
            .map_err(|e| format!("写入 README 失败: {}", e))?;

        println!("✅ 产品打包完成！输出目录: {:?}", output_dir);
        Ok(output_exe)
    }
    
    // === Phase 2: Snap 功能 (使用 serde_json) ===
    
    /// 生成 DOM 的 snap（JSON 序列化）
    pub fn generate_snap(&self, document: &Document) -> Result<Vec<u8>, String> {
        println!("=== Director: 生成 DOM Snap ===\n");
        
        let json = serde_json::to_vec(document)
            .map_err(|e| format!("Snap JSON serialization failed: {}", e))?;
        
        println!("✅ Snap 生成成功！大小: {} 字节", json.len());
        Ok(json)
    }
    
    /// 从 snap 恢复 DOM
    pub fn load_snap(&self, bytes: &[u8]) -> Result<Document, String> {
        println!("=== Director: 从 Snap 恢复 DOM ===\n");
        
        let document: Document = serde_json::from_slice(bytes)
            .map_err(|e| format!("Snap JSON deserialization failed: {}", e))?;
        
        println!("✅ DOM 从 Snap 恢复成功！");
        Ok(document)
    }
    
    /// 保存 snap 到文件
    pub fn save_snap_to_file(&self, document: &Document, path: &PathBuf) -> Result<(), String> {
        let snap_bytes = self.generate_snap(document)?;
        fs::write(path, snap_bytes)
            .map_err(|e| format!("Save snap failed: {}", e))?;
        println!("✅ Snap 已保存到: {:?}", path);
        Ok(())
    }
    
    /// 从文件加载 snap
    pub fn load_snap_from_file(&self, path: &PathBuf) -> Result<Document, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Load snap failed: {}", e))?;
        self.load_snap(&bytes)
    }

    /// 自动分裂：jrust → jrusti(initializer+snap) + jruste(event handler)
    pub fn auto_split_into_jrusti_jruste(&self, document: &Document, output_dir: &PathBuf) -> Result<(), String> {
        println!("\n=== Director: 自动分裂开始 ===\n");

        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Create output dir failed: {}", e))?;

        // === 1. 生成 jrusti (initializer + Snap) ===
        println!("--- 1. 生成 jrusti（初始化器 + Snap） ---");
        let snap_path = output_dir.join("app.snap");
        self.save_snap_to_file(document, &snap_path)?;

        let jrusti_code = format!(r#"
//! jrusti - Initializer + Snap 加载器
use jrust_runtime::director::Director;
use jrust_runtime::dom::document::Document;
use std::path::PathBuf;

fn main() -> Result<(), String> {{
    println!("🚀 === jrusti 启动！加载 Snap 中... === 🚀");
    
    let director = Director::new();
    let snap_path = PathBuf::from("app.snap");
    let document = director.load_snap_from_file(&snap_path)?;
    
    println!("✅ Snap 加载成功！DOM 已就绪！");
    println!("   Document title: {{}}", document.title());
    
    println!("\n🚀 === jrusti 初始化完成！准备启动 jruste === 🚀");
    Ok(())
}}
"#);
        let jrusti_path = output_dir.join("jrusti.rs");
        fs::write(&jrusti_path, jrusti_code)
            .map_err(|e| format!("Write jrusti failed: {}", e))?;
        println!("✅ jrusti 已保存到 {:?}", jrusti_path);

        // === 2. 生成 jruste (event handler) ===
        println!("\n--- 2. 生成 jruste（事件处理器） ---");
        let jruste_code = format!(r#"
//! jruste - Event Handler + DOM 渲染
use jrust_runtime::director::Director;
use jrust_runtime::document::Document;
use jrust_runtime::element::Element;
use jrust_runtime::events::{{EventType, EventTarget}};
use std::path::PathBuf;

fn main() -> Result<(), String> {{
    println!("🚀 === jruste 启动！加载 Snap + 处理事件 === 🚀");
    
    // 1. 加载 Snap
    let director = Director::new();
    let snap_path = PathBuf::from("app.snap");
    let mut document = director.load_snap_from_file(&snap_path)?;
    
    println!("✅ Snap 加载成功！");
    
    // 2. 模拟事件循环
    println!("\n--- 事件循环开始 ---");
    println!("   按 Ctrl+C 退出...");
    
    // 简单的事件循环（模拟）
    let mut counter = 0;
    loop {{
        std::thread::sleep(std::time::Duration::from_millis(100));
        counter += 1;
        
        if counter % 10 == 0 {{
            println!("🔄 事件循环中... 已处理 {{}} 帧", counter);
        }}
        
        if counter > 50 {{
            break;
        }}
    }}
    
    println!("\n✅ 事件循环结束！");
    Ok(())
}}
"#);
        let jruste_path = output_dir.join("jruste.rs");
        fs::write(&jruste_path, jruste_code)
            .map_err(|e| format!("Write jruste failed: {}", e))?;
        println!("✅ jruste 已保存到 {:?}", jruste_path);

        println!("\n=== Director: 自动分裂完成！ ===");
        Ok(())
    }
}

/// JRustApp - 自动 Snap 的应用包装器
pub struct JRustApp {
    pub window: crate::bom::window::Window,
    snap_path: Option<PathBuf>,
}

impl JRustApp {
    pub fn new() -> Self {
        JRustApp {
            window: crate::bom::window::Window::new(),
            snap_path: None,
        }
    }
    
    pub fn with_snap_output<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.snap_path = Some(path.into());
        self
    }
}

impl Drop for JRustApp {
    fn drop(&mut self) {
        if let Some(snap_path) = &self.snap_path {
            println!("\n🛡️  JRustApp 正在销毁 - 自动生成 Snap...");
            
            let director = Director::new();
            if let Ok(_) = director.save_snap_to_file(&self.window.document, snap_path) {
                println!("✅ Snap 自动保存成功！");
            }
        }
    }
}

impl Default for Director {
    fn default() -> Self {
        Self::new()
    }
}

/// 简单的示例 jrust 实现
pub struct SimpleJsRust {
    name: String,
}

impl SimpleJsRust {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl JsRustInstance for SimpleJsRust {
    fn init(&mut self) {
        println!("{} 初始化完成", self.name);
    }
    
    fn handle_event(&mut self) -> bool {
        println!("{} 收到事件", self.name);
        false
    }
    
    fn deploy_javascript_task(&mut self, _js_code: &str) {}
    
    fn get_children(&self) -> Vec<JsRustId> {
        Vec::new()
    }
}

