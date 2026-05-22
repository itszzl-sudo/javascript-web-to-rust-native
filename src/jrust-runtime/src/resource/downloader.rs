use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

const MAX_RETRIES: u32 = 3;
const TIMEOUT_SECS: u64 = 30;

pub struct Downloader {
    user_cancelled: bool,
}

#[derive(Debug)]
pub enum DownloadError {
    Network(String),
    Io(io::Error),
    Cancelled,
    MaxRetriesExceeded,
}

impl From<io::Error> for DownloadError {
    fn from(e: io::Error) -> Self {
        DownloadError::Io(e)
    }
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::Network(msg) => write!(f, "网络错误: {}", msg),
            DownloadError::Io(e) => write!(f, "IO 错误: {}", e),
            DownloadError::Cancelled => write!(f, "用户取消下载"),
            DownloadError::MaxRetriesExceeded => write!(f, "重试 {} 次后仍失败", MAX_RETRIES),
        }
    }
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            user_cancelled: false,
        }
    }
    
    pub fn download_file(&mut self, url: &str, dest_path: &Path) -> Result<PathBuf, DownloadError> {
        self.user_cancelled = false;
        
        for attempt in 1..=MAX_RETRIES {
            if self.user_cancelled {
                return Err(DownloadError::Cancelled);
            }
            
            self.show_download_prompt(url, attempt)?;
            
            match self.try_download(url, dest_path) {
                Ok(path) => {
                    self.show_success(url, &path);
                    return Ok(path);
                }
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        self.show_retry_prompt(url, attempt, &e);
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        }
        
        self.show_failure(url);
        Err(DownloadError::MaxRetriesExceeded)
    }
    
    pub fn download_cdn_js(&mut self, url: &str, output_dir: &Path) -> Result<String, DownloadError> {
        let filename = self.extract_filename(url, "downloaded.js");
        let dest_path = output_dir.join(&filename);
        
        self.download_file(url, &dest_path)?;
        
        Ok(dest_path.to_string_lossy().to_string())
    }
    
    pub fn download_google_font(&mut self, font_family: &str, output_dir: &Path) -> Result<PathBuf, DownloadError> {
        let font_url = self.build_google_font_url(font_family);
        let filename = format!("{}.ttf", font_family.replace(' ', "_"));
        let dest_path = output_dir.join("fonts").join(&filename);
        
        fs::create_dir_all(dest_path.parent().unwrap())?;
        
        self.download_file(&font_url, &dest_path)
    }
    
    fn try_download(&self, url: &str, dest_path: &Path) -> Result<PathBuf, DownloadError> {
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        #[cfg(feature = "network")]
        {
            use ureq;
            
            let response = ureq::get(url)
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .call()
                .map_err(|e| DownloadError::Network(e.to_string()))?;
            
            let mut reader = response.into_reader();
            let mut buffer = Vec::new();
            io::copy(&mut reader, &mut buffer)?;
            
            fs::write(dest_path, &buffer)?;
            Ok(dest_path.to_path_buf())
        }
        
        #[cfg(not(feature = "network"))]
        {
            Err(DownloadError::Network("网络功能未启用。请在 Cargo.toml 中添加 'network' feature".to_string()))
        }
    }
    
    fn show_download_prompt(&self, url: &str, attempt: u32) -> Result<(), DownloadError> {
        println!("\n{'='}{}{'='}", "=".repeat(60));
        println!("📥 下载资源 (尝试 {}/{})", attempt, MAX_RETRIES);
        println!("   URL: {}", url);
        println!("{'='}{}{'='}\n", "=".repeat(60));
        println!("按 Enter 继续下载，输入 'q' 取消:");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(DownloadError::Io)?;
        
        if input.trim() == "q" {
            println!("❌ 用户取消下载，退出 Jade");
            std::process::exit(1);
        }
        
        Ok(())
    }
    
    fn show_retry_prompt(&self, url: &str, attempt: u32, error: &DownloadError) {
        println!("\n⚠️  下载失败 (尝试 {}/{}): {}", attempt, MAX_RETRIES, error);
        println!("   URL: {}", url);
        println!("   2 秒后重试...\n");
    }
    
    fn show_success(&self, url: &str, path: &Path) {
        println!("\n✅ 下载成功: {}", path.display());
        println!("   URL: {}\n", url);
    }
    
    fn show_failure(&self, url: &str) {
        println!("\n{'='}{}{'='}", "=".repeat(60));
        println!("❌ 下载失败");
        println!("   URL: {}", url);
        println!("   重试 {} 次后仍失败，退出 Jade", MAX_RETRIES);
        println!("{'='}{}{'='}\n", "=".repeat(60));
        std::process::exit(1);
    }
    
    fn extract_filename(&self, url: &str, default: &str) -> String {
        if let Some(filename) = url.split('/').last() {
            if !filename.is_empty() && filename.contains('.') {
                return filename.split('?').next().unwrap_or(filename).to_string();
            }
        }
        default.to_string()
    }
    
    fn build_google_font_url(&self, font_family: &str) -> String {
        let family_encoded = urlencoding::encode(font_family);
        format!(
            "https://fonts.gstatic.com/s/{}/v1/{}.ttf",
            family_encoded.to_lowercase(),
            family_encoded.replace(' ', "")
        )
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

pub fn download_cdn_js(url: &str, output_dir: &Path) -> Result<String, DownloadError> {
    let mut downloader = Downloader::new();
    downloader.download_cdn_js(url, output_dir)
}

pub fn download_google_font(font_family: &str, output_dir: &Path) -> Result<PathBuf, DownloadError> {
    let mut downloader = Downloader::new();
    downloader.download_google_font(font_family, output_dir)
}

#[cfg(feature = "network")]
pub fn is_cdn_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    url_lower.contains("unpkg.com") ||
    url_lower.contains("cdn.jsdelivr.net") ||
    url_lower.contains("cdnjs.cloudflare.com") ||
    url_lower.contains("code.jquery.com") ||
    url_lower.contains("ajax.googleapis.com") ||
    url_lower.contains("stackpath.bootstrapcdn.com")
}

#[cfg(feature = "network")]
pub fn is_google_fonts_url(url: &str) -> bool {
    url.to_lowercase().contains("fonts.googleapis.com") ||
    url.to_lowercase().contains("fonts.gstatic.com")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_filename() {
        let downloader = Downloader::new();
        assert_eq!(downloader.extract_filename("https://cdn.com/lib.js?v=1", "default.js"), "lib.js");
        assert_eq!(downloader.extract_filename("https://cdn.com/path/", "default.js"), "default.js");
    }
    
    #[test]
    fn test_is_cdn_url() {
        #[cfg(feature = "network")]
        {
            assert!(is_cdn_url("https://unpkg.com/vue@3"));
            assert!(is_cdn_url("https://cdn.jsdelivr.net/npm/vue"));
            assert!(!is_cdn_url("https://example.com/script.js"));
        }
    }
}
