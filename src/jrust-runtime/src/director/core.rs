
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
    
    /// 翻译 JS 代码为 JRust (库调用方式，不启动子进程)
    pub fn translate_to_jrust(&self, js_code: &str) -> Result<String, String> {
        println!("=== Director: 翻译 JS 为 JRust ===\n");
        
        // 注意：这里我们需要依赖 jrust-translator 库
        // 但为了避免循环依赖，我们暂时返回一个模拟的结果
        // 在实际使用中，Director 应该在单独的 crate 中
        
        println!("输入 JS 代码长度: {} 字节", js_code.len());
        
        // 模拟翻译过程
        let rust_code = format!(
            "// 由 Director 翻译的 JRust 代码\n\
            fn main() {{\n\
            \tprintln!(\"Hello from translated JRust!\");\n\
            }}\n"
        );
        
        println!("✅ 翻译完成\n");
        Ok(rust_code)
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

