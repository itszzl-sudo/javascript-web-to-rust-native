use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct Linker {
    target: String,
    lld_path: Option<PathBuf>,
}

impl Linker {
    pub fn new(target: &str) -> Result<Self, String> {
        let lld_path = Self::find_lld()?;
        Ok(Self {
            target: target.to_string(),
            lld_path,
        })
    }
    
    fn find_lld() -> Result<Option<PathBuf>, String> {
        if let Ok(path) = which::which("lld") {
            return Ok(Some(path));
        }
        
        if let Ok(path) = which::which("ld.lld") {
            return Ok(Some(path));
        }
        
        if let Ok(rustup_home) = std::env::var("RUSTUP_HOME") {
            let lld = PathBuf::from(rustup_home)
                .join("toolchains")
                .join("stable-x86_64-pc-windows-msvc")
                .join("bin")
                .join("lld.exe");
            if lld.exists() {
                return Ok(Some(lld));
            }
        }
        
        Ok(None)
    }
    
    pub fn link_exe(
        &self,
        obj_bytes: &[u8],
        lib_path: &str,
        output_path: &str,
    ) -> Result<(), String> {
        println!("=== LLD 链接开始 ===");
        
        let temp_obj = PathBuf::from("__temp__.obj");
        fs::write(&temp_obj, obj_bytes)
            .map_err(|e| format!("写入临时 obj 失败: {}", e))?;
        
        let output = if cfg!(windows) {
            format!("{}.exe", output_path.trim_end_matches(".exe"))
        } else {
            output_path.to_string()
        };
        
        let result = if let Some(lld) = &self.lld_path {
            self.link_with_lld(lld, &temp_obj, lib_path, &output)
        } else {
            self.link_with_system_linker(&temp_obj, lib_path, &output)
        };
        
        let _ = fs::remove_file(&temp_obj);
        
        result
    }
    
    fn link_with_lld(
        &self,
        lld_path: &PathBuf,
        obj_path: &PathBuf,
        lib_path: &str,
        output_path: &str,
    ) -> Result<(), String> {
        let mut args = vec![
            "link".to_string(),
            obj_path.to_string_lossy().to_string(),
            format!("-out:{}", output_path),
        ];
        
        if cfg!(windows) {
            args.push("-machine:x64".to_string());
            args.push("-subsystem:console".to_string());
            args.push("libcmt.lib".to_string());
            args.push("kernel32.lib".to_string());
            args.push("user32.lib".to_string());
        }
        
        if fs::metadata(lib_path).is_ok() {
            args.push(lib_path.to_string());
        }
        
        println!("执行: {:?} {:?}", lld_path, args);
        
        let output = Command::new(lld_path)
            .args(&args)
            .output()
            .map_err(|e| format!("执行 LLD 失败: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "LLD 链接失败:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        println!("✅ 链接完成: {}", output_path);
        Ok(())
    }
    
    fn link_with_system_linker(
        &self,
        obj_path: &PathBuf,
        lib_path: &str,
        output_path: &str,
    ) -> Result<(), String> {
        println!("未找到 LLD，使用系统链接器");
        
        if cfg!(windows) {
            let args = vec![
                obj_path.to_string_lossy().to_string(),
                lib_path.to_string(),
                format!("/OUT:{}", output_path),
            ];
            
            let output = Command::new("link.exe")
                .args(&args)
                .output()
                .map_err(|e| format!("执行 link.exe 失败: {}", e))?;
            
            if !output.status.success() {
                return Err(format!(
                    "link.exe 失败:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        } else {
            let args = vec![
                obj_path.to_string_lossy().to_string(),
                lib_path.to_string(),
                "-o".to_string(),
                output_path.to_string(),
            ];
            
            let output = Command::new("ld")
                .args(&args)
                .output()
                .map_err(|e| format!("执行 ld 失败: {}", e))?;
            
            if !output.status.success() {
                return Err(format!(
                    "ld 失败:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        
        println!("✅ 链接完成: {}", output_path);
        Ok(())
    }
    
    pub fn link_lib(
        &self,
        obj_bytes: &[u8],
        lib_name: &str,
        output_path: &str,
    ) -> Result<(), String> {
        println!("=== 打包为 lib ===");
        
        let temp_obj = PathBuf::from("__temp__.obj");
        fs::write(&temp_obj, obj_bytes)
            .map_err(|e| format!("写入临时 obj 失败: {}", e))?;
        
        let output = if cfg!(windows) {
            format!("{}.lib", lib_name)
        } else {
            format!("lib{}.a", lib_name)
        };
        
        let result = if cfg!(windows) {
            let lib_output = Command::new("lib.exe")
                .arg(&temp_obj)
                .arg(format!("/OUT:{}", output))
                .output()
                .map_err(|e| format!("执行 lib.exe 失败: {}", e))?;
            
            if !lib_output.status.success() {
                Err(format!("lib.exe 失败: {}", String::from_utf8_lossy(&lib_output.stderr)))
            } else {
                Ok(())
            }
        } else {
            let ar_output = Command::new("ar")
                .arg("rcs")
                .arg(&output)
                .arg(&temp_obj)
                .output()
                .map_err(|e| format!("执行 ar 失败: {}", e))?;
            
            if !ar_output.status.success() {
                Err(format!("ar 失败: {}", String::from_utf8_lossy(&ar_output.stderr)))
            } else {
                Ok(())
            }
        };
        
        let _ = fs::remove_file(&temp_obj);
        result
    }
}
