use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::external_deps::{ExternalDependency, ExternalDepType, ExternalDepDetector};
use crate::Compiler;

pub struct ExternalDepProcessor {
    output_dir: PathBuf,
    downloader: Option<crate::resource::Downloader>,
}

#[derive(Debug)]
pub enum ProcessError {
    Unsupported(String),
    DownloadFailed(String),
    UserCancelled,
    Io(io::Error),
}

impl From<io::Error> for ProcessError {
    fn from(e: io::Error) -> Self {
        ProcessError::Io(e)
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::Unsupported(msg) => write!(f, "不支持的外部依赖: {}", msg),
            ProcessError::DownloadFailed(msg) => write!(f, "下载失败: {}", msg),
            ProcessError::UserCancelled => write!(f, "用户取消"),
            ProcessError::Io(e) => write!(f, "IO 错误: {}", e),
        }
    }
}

impl ExternalDepProcessor {
    pub fn new(output_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
            downloader: None,
        }
    }
    
    pub fn process_source(&mut self, source: &str) -> Result<Vec<PathBuf>, ProcessError> {
        let mut detector = ExternalDepDetector::new();
        detector.detect_from_source(source);
        
        let deps = detector.get_dependencies();
        
        if deps.is_empty() {
            return Ok(Vec::new());
        }
        
        if detector.has_unsupported_deps() {
            self.handle_unsupported_deps(detector.get_unsupported_deps())?;
        }
        
        let mut results = Vec::new();
        
        for dep in deps {
            match dep.dep_type {
                ExternalDepType::JavaScript => {
                    let processed = self.process_js_dep(&dep)?;
                    results.push(processed);
                }
                ExternalDepType::Font => {
                    let processed = self.process_font_dep(&dep)?;
                    results.push(processed);
                }
                ExternalDepType::Image => {
                    let processed = self.process_image_dep(&dep)?;
                    results.push(processed);
                }
                ExternalDepType::Other => {
                }
            }
        }
        
        Ok(results)
    }
    
    fn process_js_dep(&mut self, dep: &ExternalDependency) -> Result<PathBuf, ProcessError> {
        println!("\n{}{}{}", "=".repeat(60), "=", "=".repeat(60));
        println!("🔧 发现外部 JavaScript");
        println!("   URL: {}", dep.url);
        println!("   来源: {}", dep.source);
        println!("{}{}{}\n", "=".repeat(60), "=", "=".repeat(60));
        
        let downloaded = self.download_with_retry(&dep.url, "JavaScript")?;
        
        println!("📦 开始转译...");
        let compiled = self.translate_js(&downloaded)?;
        
        println!("✅ 转译完成: {}\n", compiled.display());
        Ok(compiled)
    }
    
    fn process_font_dep(&mut self, dep: &ExternalDependency) -> Result<PathBuf, ProcessError> {
        println!("\n{}{}{}", "=".repeat(60), "=", "=".repeat(60));
        println!("🔤 发现外部字体");
        println!("   URL: {}", dep.url);
        println!("{}{}{}\n", "=".repeat(60), "=", "=".repeat(60));
        
        let font_dir = self.output_dir.join("fonts");
        std::fs::create_dir_all(&font_dir)?;
        
        let downloaded = self.download_with_retry(&dep.url, "字体")?;
        
        println!("✅ 字体已下载: {}\n", downloaded.display());
        Ok(downloaded)
    }
    
    fn process_image_dep(&mut self, dep: &ExternalDependency) -> Result<PathBuf, ProcessError> {
        println!("\n{}{}{}", "=".repeat(60), "=", "=".repeat(60));
        println!("🖼️  发现外部图片");
        println!("   URL: {}", dep.url);
        println!("{}{}{}\n", "=".repeat(60), "=", "=".repeat(60));
        
        let image_dir = self.output_dir.join("images");
        std::fs::create_dir_all(&image_dir)?;
        
        let downloaded = self.download_with_retry(&dep.url, "图片")?;
        
        println!("✅ 图片已下载: {}\n", downloaded.display());
        Ok(downloaded)
    }
    
    fn handle_unsupported_deps(&self, deps: Vec<&ExternalDependency>) -> Result<(), ProcessError> {
        println!("\n{}{}{}", "=".repeat(60), "=", "=".repeat(60));
        println!("⚠️  发现不支持的外部依赖");
        println!("{}{}{}\n", "=".repeat(60), "=", "=".repeat(60));
        
        for dep in deps {
            println!("❌ 类型: 其他");
            println!("   URL: {}", dep.url);
            println!("   来源: {}", dep.source);
            println!();
        }
        
        println!("Jade 不支持以下类型的外部依赖：");
        println!("  - API 请求 (fetch/axios)");
        println!("  - WebSocket");
        println!("  - 其他网络资源");
        println!("\n按 Enter 确认后退出 Jade，或输入 'c' 继续处理其他依赖:");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(ProcessError::Io)?;
        
        if input.trim() == "c" {
            println!("⚠️  跳过不支持的外部依赖，继续处理...\n");
            return Ok(());
        }
        
        println!("\n❌ 用户确认退出 Jade");
        std::process::exit(1);
    }
    
    fn download_with_retry(&mut self, url: &str, resource_type: &str) -> Result<PathBuf, ProcessError> {
        const MAX_RETRIES: u32 = 3;
        
        for attempt in 1..=MAX_RETRIES {
            println!("📥 下载 {} (尝试 {}/{})", resource_type, attempt, MAX_RETRIES);
            println!("   URL: {}", url);
            println!("\n按 Enter 继续下载，输入 'q' 取消:");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(ProcessError::Io)?;
            
            if input.trim() == "q" {
                println!("❌ 用户取消下载，退出 Jade");
                std::process::exit(1);
            }
            
            match self.try_download(url) {
                Ok(path) => return Ok(path),
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        println!("\n⚠️  下载失败: {}", e);
                        println!("   2 秒后重试...\n");
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                }
            }
        }
        
        println!("\n{}{}{}", "=".repeat(60), "=", "=".repeat(60));
        println!("❌ 下载失败");
        println!("   URL: {}", url);
        println!("   重试 {} 次后仍失败，退出 Jade", MAX_RETRIES);
        println!("{}{}{}\n", "=".repeat(60), "=", "=".repeat(60));
        std::process::exit(1);
    }
    
    #[cfg(feature = "network")]
    fn try_download(&self, url: &str) -> Result<PathBuf, ProcessError> {
        use std::fs;
        
        let filename = self.extract_filename(url);
        let dest_path = self.output_dir.join(&filename);
        
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(30))
            .call()
            .map_err(|e| ProcessError::DownloadFailed(e.to_string()))?;
        
        let mut reader = response.into_reader();
        let mut buffer = Vec::new();
        io::copy(&mut reader, &mut buffer)?;
        
        fs::write(&dest_path, &buffer)?;
        Ok(dest_path)
    }
    
    #[cfg(not(feature = "network"))]
    fn try_download(&self, _url: &str) -> Result<PathBuf, ProcessError> {
        Err(ProcessError::DownloadFailed(
            "网络功能未启用。请在编译时添加 --features network".to_string()
        ))
    }
    
    fn translate_js(&self, js_path: &Path) -> Result<PathBuf, ProcessError> {
        let js_content = std::fs::read_to_string(js_path)?;
        
        let mut compiler = Compiler::new();
        let result = compiler.compile(&js_content)
            .map_err(|e| ProcessError::DownloadFailed(format!("转译失败: {}", e)))?;
        
        let rs_filename = js_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.replace(".js", ".rs"))
            .unwrap_or_else(|| "downloaded.rs".to_string());
        
        let rs_path = self.output_dir.join("translated").join(&rs_filename);
        std::fs::create_dir_all(rs_path.parent().unwrap())?;
        std::fs::write(&rs_path, &result.code)?;
        
        Ok(rs_path)
    }
    
    fn extract_filename(&self, url: &str) -> String {
        if let Some(filename) = url.split('/').last() {
            if !filename.is_empty() && filename.contains('.') {
                return filename.split('?').next().unwrap_or(filename).to_string();
            }
        }
        "downloaded".to_string()
    }
}

pub fn process_external_deps(
    source: &str,
    output_dir: impl Into<PathBuf>,
) -> Result<Vec<PathBuf>, ProcessError> {
    let mut processor = ExternalDepProcessor::new(output_dir);
    processor.process_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_external_deps() {
        let source = "let x = 1;";
        let result = process_external_deps(source, "./output");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
