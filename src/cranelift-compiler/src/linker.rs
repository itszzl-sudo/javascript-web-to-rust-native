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
        // 1. 优先使用项目内工具链
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("获取当前目录失败: {}", e))?;
        
        if cfg!(windows) {
            // 尝试多个可能的路径
            let possible_paths = vec![
                current_dir.join("toolchain").join("win32-x64-msvc").join("link.exe"),
                current_dir.join("packages").join("jade").join("toolchain").join("win32-x64-msvc").join("link.exe"),
                current_dir.parent()
                    .map(|p| p.join("toolchain").join("win32-x64-msvc").join("link.exe"))
                    .unwrap_or_default(),
            ];
            
            for project_link in possible_paths {
                if project_link.exists() {
                    println!("✅ 使用项目内工具链: {:?}", project_link);
                    return Ok(Some(project_link));
                }
            }
        }
        
        // 2. 查找系统 LLD
        if let Ok(path) = which::which("lld") {
            return Ok(Some(path));
        }
        
        if let Ok(path) = which::which("ld.lld") {
            return Ok(Some(path));
        }
        
        // 3. 查找 Rustup 中的 LLD
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
        
        // Windows: 检查 .NET Framework
        if cfg!(windows) {
            self.check_dotnet_framework()?;
        }
        
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
    
    fn check_dotnet_framework(&self) -> Result<(), String> {
        use std::process::Command;
        
        println!("检查 .NET Framework...");
        
        // 方法 1: 检查文件系统
        let dotnet_paths = vec![
            // .NET 4.8
            ("4.8", "C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319"),
            ("4.8", "C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319"),
            // .NET 4.0
            ("4.0", "C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30128"),
            ("4.0", "C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30128"),
        ];
        
        for (version, path) in dotnet_paths {
            if fs::metadata(path).is_ok() {
                let clr_path = format!("{}\\clr.dll", path);
                if fs::metadata(&clr_path).is_ok() {
                    println!("✅ 检测到 .NET Framework {}: {}", version, path);
                    return Ok(());
                }
            }
        }
        
        // 方法 2: 通过注册表检查（更准确）
        let reg_output = Command::new("reg")
            .args(&[
                "query",
                "HKLM\\SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full",
                "/v",
                "Release",
            ])
            .output();
        
        if let Ok(output) = reg_output {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                
                // 解析 Release 值
                // 4.5: 378389
                // 4.5.1: 378675
                // 4.5.2: 379893
                // 4.6: 393295
                // 4.6.1: 394254
                // 4.6.2: 394802
                // 4.7: 460798
                // 4.7.1: 461308
                // 4.7.2: 461808
                // 4.8: 528040
                
                for line in result.lines() {
                    if line.contains("Release") && line.contains("REG_DWORD") {
                        // 提取版本号
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 {
                            if let Ok(release) = u32::from_str_radix(parts.last().unwrap(), 16) {
                                let version = match release {
                                    r if r >= 528040 => "4.8",
                                    r if r >= 461808 => "4.7.2",
                                    r if r >= 461308 => "4.7.1",
                                    r if r >= 460798 => "4.7",
                                    r if r >= 394802 => "4.6.2",
                                    r if r >= 394254 => "4.6.1",
                                    r if r >= 393295 => "4.6",
                                    r if r >= 379893 => "4.5.2",
                                    r if r >= 378675 => "4.5.1",
                                    r if r >= 378389 => "4.5",
                                    _ => "4.0+",
                                };
                                println!("✅ 检测到 .NET Framework {} (Release: {})", version, release);
                                return Ok(());
                            }
                        }
                    }
                }
                
                // 如果包含 Release 键但无法解析，仍然认为已安装
                if result.contains("Release") {
                    println!("✅ 检测到 .NET Framework 4.x (注册表)");
                    return Ok(());
                }
            }
        }
        
        // 方法 3: 检查 Install 值（兼容旧版本）
        let install_check = Command::new("reg")
            .args(&[
                "query",
                "HKLM\\SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Client",
                "/v",
                "Install",
            ])
            .output();
        
        if let Ok(output) = install_check {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                if result.contains("Install") && result.contains("0x1") {
                    println!("✅ 检测到 .NET Framework 4.0 (Client)");
                    return Ok(());
                }
            }
        }
        
        // 未找到 .NET Framework
        let error_msg = r#"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ 未检测到 .NET Framework 4.0 或更高版本

MSVC 链接器需要 .NET Framework 支持。

支持的版本：
  ✅ .NET Framework 4.8   (推荐，最新)
  ✅ .NET Framework 4.7.2
  ✅ .NET Framework 4.7.1
  ✅ .NET Framework 4.7
  ✅ .NET Framework 4.6.2
  ✅ .NET Framework 4.6.1
  ✅ .NET Framework 4.6
  ✅ .NET Framework 4.5.2
  ✅ .NET Framework 4.5.1
  ✅ .NET Framework 4.5
  ✅ .NET Framework 4.0

下载地址：
  https://dotnet.microsoft.com/download/dotnet-framework

正在打开下载页面...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#;
        
        eprintln!("{}", error_msg);
        
        // 尝试打开下载页面
        #[cfg(windows)]
        {
            let _ = Command::new("cmd")
                .args(&["/C", "start", "https://dotnet.microsoft.com/download/dotnet-framework"])
                .spawn();
        }
        
        Err(".NET Framework 4.0+ 未安装。请安装后重试。".to_string())
    }
}
