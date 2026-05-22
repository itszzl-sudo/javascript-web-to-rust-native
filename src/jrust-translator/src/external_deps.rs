use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDepType {
    JavaScript,
    Font,
    Image,
    Other,
}

#[derive(Debug, Clone)]
pub struct ExternalDependency {
    pub url: String,
    pub dep_type: ExternalDepType,
    pub source: String,
}

impl ExternalDependency {
    pub fn new(url: String, dep_type: ExternalDepType, source: String) -> Self {
        Self {
            url,
            dep_type,
            source,
        }
    }
}

pub struct ExternalDepDetector {
    detected: Vec<ExternalDependency>,
    processed_urls: HashSet<String>,
}

impl ExternalDepDetector {
    pub fn new() -> Self {
        Self {
            detected: Vec::new(),
            processed_urls: HashSet::new(),
        }
    }
    
    pub fn detect_from_source(&mut self, source: &str) {
        self.detect_imports(source);
        self.detect_dynamic_imports(source);
        self.detect_font_urls(source);
        self.detect_image_urls(source);
        self.detect_fetch_calls(source);
    }
    
    fn detect_imports(&mut self, source: &str) {
        let import_patterns = [
            r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#,
            r#"import\s+['"]([^'"]+)['"]"#,
            r#"export\s+.*?\s+from\s+['"]([^'"]+)['"]"#,
        ];
        
        for pattern in import_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(source) {
                    if let Some(url) = cap.get(1) {
                        let url_str = url.as_str();
                        if self.is_external_url(url_str) && !self.processed_urls.contains(url_str) {
                            self.processed_urls.insert(url_str.to_string());
                            let dep_type = self.classify_url(url_str);
                            self.detected.push(ExternalDependency::new(
                                url_str.to_string(),
                                dep_type,
                                "import".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    
    fn detect_dynamic_imports(&mut self, source: &str) {
        let patterns = [
            r#"import\(['"]([^'"]+)['"]\)"#,
            r#"require\(['"]([^'"]+)['"]\)"#,
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(source) {
                    if let Some(url) = cap.get(1) {
                        let url_str = url.as_str();
                        if self.is_external_url(url_str) && !self.processed_urls.contains(url_str) {
                            self.processed_urls.insert(url_str.to_string());
                            let dep_type = self.classify_url(url_str);
                            self.detected.push(ExternalDependency::new(
                                url_str.to_string(),
                                dep_type,
                                "dynamic-import".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    
    fn detect_font_urls(&mut self, source: &str) {
        let patterns = [
            r#"url\(['"]?(https?://fonts\.googleapis\.com[^'"\)]*)['"]?\)"#,
            r#"url\(['"]?(https?://fonts\.gstatic\.com[^'"\)]*)['"]?\)"#,
            r#"@import\s+url\(['"]?(https?://fonts\.googleapis\.com[^'"\)]*)['"]?\)"#,
            r#"font-family:\s*['"]([^'"]+)['"]"#,
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(source) {
                    if let Some(url) = cap.get(1) {
                        let url_str = url.as_str();
                        if !self.processed_urls.contains(url_str) {
                            self.processed_urls.insert(url_str.to_string());
                            self.detected.push(ExternalDependency::new(
                                url_str.to_string(),
                                ExternalDepType::Font,
                                "font-url".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    
    fn detect_image_urls(&mut self, source: &str) {
        let patterns = [
            r#"\.src\s*=\s*['"]([^'"]+\.(png|jpg|jpeg|gif|svg|webp))['"]"#,
            r#"new\s+Image\(\)"#,
            r#"url\(['"]?(https?://[^'"\)]+\.(png|jpg|jpeg|gif|svg|webp))['"]?\)"#,
        ];
        
        if let Ok(re) = regex::Regex::new(&patterns[2]) {
            for cap in re.captures_iter(source) {
                if let Some(url) = cap.get(1) {
                    let url_str = url.as_str();
                    if self.is_external_url(url_str) && !self.processed_urls.contains(url_str) {
                        self.processed_urls.insert(url_str.to_string());
                        self.detected.push(ExternalDependency::new(
                            url_str.to_string(),
                            ExternalDepType::Image,
                            "image-url".to_string(),
                        ));
                    }
                }
            }
        }
    }
    
    fn detect_fetch_calls(&mut self, source: &str) {
        let patterns = [
            r#"fetch\(['"]([^'"]+)['"]"#,
            r#"axios\.[get|post]+\(['"]([^'"]+)['"]"#,
            r#"\.get\(['"]([^'"]+)['"]"#,
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(source) {
                    if let Some(url) = cap.get(1) {
                        let url_str = url.as_str();
                        if self.is_external_url(url_str) && !self.processed_urls.contains(url_str) {
                            self.processed_urls.insert(url_str.to_string());
                            self.detected.push(ExternalDependency::new(
                                url_str.to_string(),
                                ExternalDepType::Other,
                                "fetch".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    
    fn is_external_url(&self, url: &str) -> bool {
        url.starts_with("http://") || 
        url.starts_with("https://") ||
        url.starts_with("//")
    }
    
    fn classify_url(&self, url: &str) -> ExternalDepType {
        let url_lower = url.to_lowercase();
        
        if url_lower.contains("fonts.googleapis.com") || 
           url_lower.contains("fonts.gstatic.com") {
            return ExternalDepType::Font;
        }
        
        if url_lower.ends_with(".png") || 
           url_lower.ends_with(".jpg") || 
           url_lower.ends_with(".jpeg") ||
           url_lower.ends_with(".gif") ||
           url_lower.ends_with(".svg") ||
           url_lower.ends_with(".webp") {
            return ExternalDepType::Image;
        }
        
        if url_lower.ends_with(".js") || 
           url_lower.ends_with(".mjs") ||
           Self::is_cdn_url(url) {
            return ExternalDepType::JavaScript;
        }
        
        ExternalDepType::Other
    }
    
    fn is_cdn_url(url: &str) -> bool {
        let url_lower = url.to_lowercase();
        url_lower.contains("unpkg.com") ||
        url_lower.contains("cdn.jsdelivr.net") ||
        url_lower.contains("cdnjs.cloudflare.com") ||
        url_lower.contains("code.jquery.com") ||
        url_lower.contains("ajax.googleapis.com") ||
        url_lower.contains("stackpath.bootstrapcdn.com")
    }
    
    pub fn get_dependencies(&self) -> &[ExternalDependency] {
        &self.detected
    }
    
    pub fn get_by_type(&self, dep_type: ExternalDepType) -> Vec<&ExternalDependency> {
        self.detected.iter()
            .filter(|d| d.dep_type == dep_type)
            .collect()
    }
    
    pub fn has_unsupported_deps(&self) -> bool {
        self.detected.iter().any(|d| d.dep_type == ExternalDepType::Other)
    }
    
    pub fn get_unsupported_deps(&self) -> Vec<&ExternalDependency> {
        self.get_by_type(ExternalDepType::Other)
    }
    
    pub fn clear(&mut self) {
        self.detected.clear();
        self.processed_urls.clear();
    }
}

impl Default for ExternalDepDetector {
    fn default() -> Self {
        Self::new()
    }
}

pub fn detect_external_deps(source: &str) -> Vec<ExternalDependency> {
    let mut detector = ExternalDepDetector::new();
    detector.detect_from_source(source);
    detector.detected
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_cdn_import() {
        let source = r#"import Vue from 'https://unpkg.com/vue@3';"#;
        let deps = detect_external_deps(source);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, ExternalDepType::JavaScript);
        assert_eq!(deps[0].url, "https://unpkg.com/vue@3");
    }
    
    #[test]
    fn test_detect_google_fonts() {
        let source = r#"@import url('https://fonts.googleapis.com/css2?family=Roboto');"#;
        let deps = detect_external_deps(source);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, ExternalDepType::Font);
    }
    
    #[test]
    fn test_detect_image_url() {
        let source = r#"url('https://example.com/logo.png')"#;
        let deps = detect_external_deps(source);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, ExternalDepType::Image);
    }
    
    #[test]
    fn test_detect_other_deps() {
        let source = r#"fetch('https://api.example.com/data')"#;
        let deps = detect_external_deps(source);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, ExternalDepType::Other);
    }
    
    #[test]
    fn test_no_local_deps() {
        let source = r#"import './local.js';"#;
        let deps = detect_external_deps(source);
        assert_eq!(deps.len(), 0);
    }
}
