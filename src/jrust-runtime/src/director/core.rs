
use crate::director::jrust_tree::{JsRustId, JsRustInstance, JsRustTree};
use std::process::Command;
use std::fs;
use std::path::PathBuf;

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

