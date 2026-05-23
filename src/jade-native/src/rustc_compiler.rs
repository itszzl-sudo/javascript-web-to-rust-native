use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileTarget {
    Native,    // exe: 完整应用 + GUI 框架
    NativeLib, // lib: 组件库，可渲染但无应用入口
    Wasm,
}

#[derive(Debug)]
pub enum RustcError {
    IoError(String),
    CompileError(String),
}

impl std::fmt::Display for RustcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustcError::IoError(msg) => write!(f, "IO错误: {}", msg),
            RustcError::CompileError(msg) => write!(f, "编译错误: {}", msg),
        }
    }
}

impl std::error::Error for RustcError {}

pub struct RustcCompiler {
    temp_dir: PathBuf,
    opt_level: String,
}

impl RustcCompiler {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir();
        Self {
            temp_dir,
            opt_level: "2".to_string(),
        }
    }
    
    pub fn with_opt_level(mut self, level: &str) -> Self {
        self.opt_level = level.to_string();
        self
    }
    
    pub fn compile(&self, rust_code: &str, target: CompileTarget) -> Result<Vec<u8>, RustcError> {
        let temp_rs = self.temp_dir.join("jrust_temp.rs");
        
        fs::write(&temp_rs, rust_code)
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        match target {
            CompileTarget::Native => self.compile_native(&temp_rs, true),
            CompileTarget::NativeLib => self.compile_native_lib(&temp_rs),
            CompileTarget::Wasm => self.compile_wasm(&temp_rs),
        }
    }
    
    fn compile_native(&self, rs_path: &PathBuf, has_gui: bool) -> Result<Vec<u8>, RustcError> {
        let output_exe = self.temp_dir.join("jrust_temp.exe");
        
        let cargo_toml = self.temp_dir.join("Cargo.toml");
        let cargo_content = if has_gui {
            r#"
[package]
name = "jrust_temp"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.27"
egui = "0.27"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[[bin]]
name = "jrust_temp"
path = "jrust_temp.rs"
"#
        } else {
            r#"
[package]
name = "jrust_temp"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[[bin]]
name = "jrust_temp"
path = "jrust_temp.rs"
"#
        };
        fs::write(&cargo_toml, cargo_content)
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(&self.temp_dir)
            .output()
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let _ = fs::remove_file(rs_path);
        let _ = fs::remove_file(&cargo_toml);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RustcError::CompileError(stderr.to_string()));
        }
        
        let release_exe = self.temp_dir.join("target").join("release").join("jrust_temp.exe");
        
        println!("✅ cargo 编译成功：Native 可执行文件");
        
        fs::read(&release_exe)
            .map_err(|e| RustcError::IoError(e.to_string()))
    }
    
    fn compile_native_lib(&self, rs_path: &PathBuf) -> Result<Vec<u8>, RustcError> {
        let cargo_toml = self.temp_dir.join("Cargo.toml");
        let cargo_content = r#"
[package]
name = "jrust_temp"
version = "0.1.0"
edition = "2021"

[lib]
name = "jrust_temp"
path = "jrust_temp.rs"
crate-type = ["cdylib"]

[dependencies]
egui = "0.27"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
"#;
        fs::write(&cargo_toml, cargo_content)
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(&self.temp_dir)
            .output()
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let _ = fs::remove_file(rs_path);
        let _ = fs::remove_file(&cargo_toml);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RustcError::CompileError(stderr.to_string()));
        }
        
        let release_lib = self.temp_dir.join("target").join("release").join("jrust_temp.dll");
        
        println!("✅ cargo 编译成功：Native 库 (可渲染)");
        
        fs::read(&release_lib)
            .map_err(|e| RustcError::IoError(e.to_string()))
    }
    
    fn compile_wasm(&self, rs_path: &PathBuf) -> Result<Vec<u8>, RustcError> {
        let output_wasm = self.temp_dir.join("jrust_temp.wasm");
        
        let output = Command::new("rustc")
            .args(&[
                "--edition", "2021",
                "--target", "wasm32-unknown-unknown",
                "-O",
                "--crate-type=cdylib",
                "-o", output_wasm.to_str().unwrap(),
                rs_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let _ = fs::remove_file(rs_path);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RustcError::CompileError(stderr.to_string()));
        }
        
        println!("✅ rustc 编译成功：WASM 模块");
        
        fs::read(&output_wasm)
            .map_err(|e| RustcError::IoError(e.to_string()))
    }
    
    pub fn check(rust_code: &str) -> Result<(), RustcError> {
        let temp_dir = std::env::temp_dir();
        let temp_rs = temp_dir.join("jrust_temp.rs");
        
        let code_with_main = if !rust_code.contains("fn main()") {
            format!("{}\n\nfn main() {{}}", rust_code)
        } else {
            rust_code.to_string()
        };
        
        fs::write(&temp_rs, &code_with_main)
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let cargo_toml = temp_dir.join("Cargo.toml");
        let cargo_content = r#"
[package]
name = "jrust_check"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.27"
egui = "0.27"

[[bin]]
name = "jrust_check"
path = "jrust_temp.rs"
"#;
        fs::write(&cargo_toml, cargo_content)
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let output = Command::new("cargo")
            .args(&["check", "--release"])
            .current_dir(&temp_dir)
            .output()
            .map_err(|e| RustcError::IoError(e.to_string()))?;
        
        let _ = fs::remove_file(&temp_rs);
        let _ = fs::remove_file(&cargo_toml);
        
        if output.status.success() {
            println!("✅ cargo 验证通过：类型正确");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(RustcError::CompileError(stderr.to_string()))
        }
    }
}

impl Default for RustcCompiler {
    fn default() -> Self {
        Self::new()
    }
}

pub fn compile_with_rustc(rust_code: &str, target: CompileTarget) -> Result<Vec<u8>, RustcError> {
    let compiler = RustcCompiler::new();
    compiler.compile(rust_code, target)
}
