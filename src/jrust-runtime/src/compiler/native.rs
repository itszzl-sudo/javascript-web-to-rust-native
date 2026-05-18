use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct NativeCompiler {
    rust_browser_path: PathBuf,
    workdir: PathBuf,
}

impl NativeCompiler {
    pub fn new() -> Self {
        Self {
            rust_browser_path: PathBuf::from("C:/Users/a/Documents/codebuddy-projects/rust-browser/rust-browser"),
            workdir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
    
    pub fn with_rust_browser_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.rust_browser_path = path.into();
        self
    }
    
    pub fn with_workdir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.workdir = path.into();
        self
    }
    
    pub fn compile_to_lib(&self, rust_code: &str, lib_name: &str) -> Result<PathBuf, String> {
        println!("=== NativeCompiler: 编译为 lib ===\n");
        println!("库名: {}", lib_name);
        println!("rust-browser 路径: {:?}", self.rust_browser_path);
        
        let project_dir = self.workdir.join(lib_name);
        let _ = fs::remove_dir_all(&project_dir);
        fs::create_dir_all(&project_dir)
            .map_err(|e| format!("创建项目目录失败: {}", e))?;
        
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
name = "{}"
path = "src/lib.rs"

[dependencies]
rust-browser = {{ path = {:?} }}
"#,
            lib_name, lib_name, self.rust_browser_path
        );
        
        fs::write(project_dir.join("Cargo.toml"), &cargo_toml)
            .map_err(|e| format!("写入 Cargo.toml 失败: {}", e))?;
        
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("创建 src 目录失败: {}", e))?;
        
        let lib_rs = self.wrap_with_bridge_api(rust_code);
        fs::write(src_dir.join("lib.rs"), &lib_rs)
            .map_err(|e| format!("写入 lib.rs 失败: {}", e))?;
        
        println!("临时项目已创建: {:?}", project_dir);
        println!("正在编译 lib...");
        
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&project_dir)
            .output()
            .map_err(|e| format!("cargo build 失败: {}", e))?;
        
        if !build_output.status.success() {
            return Err(format!(
                "cargo build 失败:\n{}",
                String::from_utf8_lossy(&build_output.stderr)
            ));
        }
        
        let lib_name_file = if cfg!(windows) {
            format!("{}.lib", lib_name.replace("-", "_"))
        } else {
            format!("lib{}.a", lib_name.replace("-", "_"))
        };
        
        let target_lib = project_dir.join("target/release").join(&lib_name_file);
        
        if target_lib.exists() {
            println!("✅ 编译完成，lib 文件: {:?}", target_lib);
            Ok(target_lib)
        } else {
            let deps_dir = project_dir.join("target/release/deps");
            if let Ok(entries) = fs::read_dir(&deps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy();
                        if name_str.starts_with(&lib_name.replace("-", "_")) && 
                           (name_str.ends_with(".lib") || name_str.ends_with(".a")) {
                            println!("✅ 找到 lib 文件: {:?}", path);
                            return Ok(path);
                        }
                    }
                }
            }
            Err("未找到生成的 lib 文件".to_string())
        }
    }
    
    fn wrap_with_bridge_api(&self, rust_code: &str) -> String {
        format!(
            r#"use rust_browser::WebNativeBridge;

{}

pub fn init(bridge: &mut WebNativeBridge) {{
    if let Some(init_fn) = __jrust_init.as_ref() {{
        init_fn(bridge);
    }}
}}

pub fn render(bridge: &mut WebNativeBridge) -> Vec<u8> {{
    bridge.render()
}}

mod __jrust_runtime {{
    use super::*;
    
    pub fn create_bridge(width: u32, height: u32) -> WebNativeBridge {{
        WebNativeBridge::new(width, height)
    }}
    
    pub fn set_html(bridge: &mut WebNativeBridge, html: &str) {{
        bridge.set_html(html);
    }}
    
    pub fn set_css(bridge: &mut WebNativeBridge, css: &str) {{
        bridge.set_css(css);
    }}
    
    pub fn on_click<F>(bridge: &mut WebNativeBridge, selector: &str, handler: F)
    where
        F: FnMut(f32, f32) + Send + 'static,
    {{
        bridge.on_click(selector, Box::new(handler));
    }}
    
    pub fn eval_js(bridge: &mut WebNativeBridge, code: &str) -> String {{
        bridge.eval_js(code)
    }}
}}
"#,
            rust_code
        )
    }
    
    pub fn create_exe_project(&self, lib_path: &PathBuf, exe_name: &str) -> Result<PathBuf, String> {
        println!("=== NativeCompiler: 创建 exe 项目 ===\n");
        
        let project_dir = self.workdir.join(format!("{}-exe", exe_name));
        let _ = fs::remove_dir_all(&project_dir);
        fs::create_dir_all(&project_dir)
            .map_err(|e| format!("创建项目目录失败: {}", e))?;
        
        let lib_name = lib_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("generated_app");
        
        let lib_parent = lib_path.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .ok_or_else(|| "无法确定 lib 项目路径".to_string())?;
        
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
rust-browser = {{ path = {:?} }}
{} = {{ path = {:?} }}
"#,
            exe_name, exe_name, self.rust_browser_path, lib_name, lib_parent
        );
        
        fs::write(project_dir.join("Cargo.toml"), &cargo_toml)
            .map_err(|e| format!("写入 Cargo.toml 失败: {}", e))?;
        
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("创建 src 目录失败: {}", e))?;
        
        let main_rs = format!(
            r#"use rust_browser::WebNativeBridge;
use std::fs;

fn main() {{
    let mut bridge = WebNativeBridge::new(1280, 720);
    
    {}::init(&mut bridge);
    
    let png = {}::render(&mut bridge);
    fs::write("output.png", &png).expect("Failed to write output.png");
    
    println!("Rendered to output.png");
}}
"#,
            lib_name, lib_name
        );
        
        fs::write(src_dir.join("main.rs"), &main_rs)
            .map_err(|e| format!("写入 main.rs 失败: {}", e))?;
        
        println!("正在编译 exe...");
        let build_output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&project_dir)
            .output()
            .map_err(|e| format!("cargo build 失败: {}", e))?;
        
        if !build_output.status.success() {
            return Err(format!(
                "cargo build 失败:\n{}",
                String::from_utf8_lossy(&build_output.stderr)
            ));
        }
        
        let exe_name_file = if cfg!(windows) {
            format!("{}.exe", exe_name)
        } else {
            exe_name.to_string()
        };
        
        let target_exe = project_dir.join("target/release").join(&exe_name_file);
        
        if target_exe.exists() {
            println!("✅ exe 编译完成: {:?}", target_exe);
            Ok(target_exe)
        } else {
            Err("未找到生成的 exe 文件".to_string())
        }
    }
}

impl Default for NativeCompiler {
    fn default() -> Self {
        Self::new()
    }
}
