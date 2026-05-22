use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::collections::HashMap;
use std::fs;

static RESOURCE_LOADER: OnceLock<ResourceLoader> = OnceLock::new();

pub struct ResourceLoader {
    resource_dir: Option<PathBuf>,
    font_dir: Option<PathBuf>,
    image_dir: Option<PathBuf>,
    embedded_resources: HashMap<String, Vec<u8>>,
    use_embedded: bool,
}

impl ResourceLoader {
    fn new() -> Self {
        Self {
            resource_dir: None,
            font_dir: None,
            image_dir: None,
            embedded_resources: HashMap::new(),
            use_embedded: cfg!(not(debug_assertions)),
        }
    }
    
    pub fn global() -> &'static ResourceLoader {
        RESOURCE_LOADER.get_or_init(|| {
            let mut loader = ResourceLoader::new();
            loader.init_default_paths();
            loader
        })
    }
    
    pub fn global_mut() -> &'static mut ResourceLoader {
        RESOURCE_LOADER.get_or_init(ResourceLoader::new);
        unsafe {
            RESOURCE_LOADER.get().unwrap() as *const _ as *mut _ as &'static mut ResourceLoader
        }
    }
    
    fn init_default_paths(&mut self) {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                self.resource_dir = Some(exe_dir.to_path_buf());
                self.font_dir = Some(exe_dir.join("fonts"));
                self.image_dir = Some(exe_dir.join("images"));
            }
        }
        
        #[cfg(debug_assertions)]
        {
            if let Ok(cwd) = std::env::current_dir() {
                let output_dir = cwd.join("output");
                if output_dir.exists() {
                    self.resource_dir = Some(output_dir.clone());
                    self.font_dir = Some(output_dir.join("fonts"));
                    self.image_dir = Some(output_dir.join("images"));
                }
            }
        }
    }
    
    pub fn set_resource_dir(path: impl Into<PathBuf>) {
        let path = path.into();
        let loader = Self::global_mut();
        loader.resource_dir = Some(path.clone());
        loader.font_dir = Some(path.join("fonts"));
        loader.image_dir = Some(path.join("images"));
    }
    
    pub fn set_font_dir(path: impl Into<PathBuf>) {
        Self::global_mut().font_dir = Some(path.into());
    }
    
    pub fn set_image_dir(path: impl Into<PathBuf>) {
        Self::global_mut().image_dir = Some(path.into());
    }
    
    pub fn use_embedded() {
        Self::global_mut().use_embedded = true;
    }
    
    pub fn use_filesystem() {
        Self::global_mut().use_embedded = false;
    }
    
    pub fn embed_resource(name: &str, data: Vec<u8>) {
        Self::global_mut().embedded_resources.insert(name.to_string(), data);
    }
    
    pub fn load_font(name: &str) -> Option<Vec<u8>> {
        let loader = Self::global();
        
        if loader.use_embedded {
            if let Some(data) = loader.embedded_resources.get(name) {
                return Some(data.clone());
            }
            if let Some(data) = loader.embedded_resources.get(&format!("fonts/{}", name)) {
                return Some(data.clone());
            }
        }
        
        if let Some(ref font_dir) = loader.font_dir {
            let font_path = font_dir.join(name);
            if let Ok(data) = fs::read(&font_path) {
                return Some(data);
            }
            
            for ext in &["ttf", "otf", "woff", "woff2"] {
                let path = font_path.with_extension(ext);
                if let Ok(data) = fs::read(&path) {
                    return Some(data);
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            let system_font = PathBuf::from("C:/Windows/Fonts").join(name);
            if let Ok(data) = fs::read(&system_font) {
                return Some(data);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            let system_fonts = ["/Library/Fonts", "/System/Library/Fonts"];
            for dir in system_fonts {
                let path = PathBuf::from(dir).join(name);
                if let Ok(data) = fs::read(&path) {
                    return Some(data);
                }
            }
        }
        
        None
    }
    
    pub fn load_image(path: &str) -> Option<Vec<u8>> {
        let loader = Self::global();
        
        if loader.use_embedded {
            if let Some(data) = loader.embedded_resources.get(path) {
                return Some(data.clone());
            }
            if let Some(data) = loader.embedded_resources.get(&format!("images/{}", path)) {
                return Some(data.clone());
            }
        }
        
        if path.starts_with("data:") {
            return Self::parse_data_url(path);
        }
        
        if path.starts_with("http://") || path.starts_with("https://") {
            log::warn!("Network image loading not supported: {}", path);
            return None;
        }
        
        if let Some(ref image_dir) = loader.image_dir {
            let img_path = image_dir.join(path);
            if let Ok(data) = fs::read(&img_path) {
                return Some(data);
            }
        }
        
        if let Some(ref resource_dir) = loader.resource_dir {
            let img_path = resource_dir.join(path);
            if let Ok(data) = fs::read(&img_path) {
                return Some(data);
            }
        }
        
        fs::read(path).ok()
    }
    
    fn parse_data_url(url: &str) -> Option<Vec<u8>> {
        let data_prefix = "data:";
        if !url.starts_with(data_prefix) {
            return None;
        }
        
        let rest = &url[data_prefix.len()..];
        let comma_pos = rest.find(',')?;
        let mime_part = &rest[..comma_pos];
        let data_part = &rest[comma_pos + 1..];
        
        if mime_part.contains(";base64") {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.decode(data_part).ok()
        } else {
            Some(urlencoding_decode(data_part))
        }
    }
    
    pub fn resolve_font_path(name: &str) -> Option<PathBuf> {
        let loader = Self::global();
        
        if let Some(ref font_dir) = loader.font_dir {
            let path = font_dir.join(name);
            if path.exists() {
                return Some(path);
            }
        }
        
        None
    }
    
    pub fn resolve_image_path(path: &str) -> Option<PathBuf> {
        let loader = Self::global();
        
        if let Some(ref image_dir) = loader.image_dir {
            let img_path = image_dir.join(path);
            if img_path.exists() {
                return Some(img_path);
            }
        }
        
        if let Some(ref resource_dir) = loader.resource_dir {
            let img_path = resource_dir.join(path);
            if img_path.exists() {
                return Some(img_path);
            }
        }
        
        let abs_path = PathBuf::from(path);
        if abs_path.exists() {
            Some(abs_path)
        } else {
            None
        }
    }
}

fn urlencoding_decode(s: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(s.len());
    let mut chars = s.chars();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte);
            }
        } else if c == '+' {
            result.push(b' ');
        } else {
            result.push(c as u8);
        }
    }
    
    result
}

pub fn load_font(name: &str) -> Option<Vec<u8>> {
    ResourceLoader::global().load_font(name)
}

pub fn load_image(path: &str) -> Option<Vec<u8>> {
    ResourceLoader::global().load_image(path)
}

pub fn set_resource_dir(path: impl Into<PathBuf>) {
    ResourceLoader::set_resource_dir(path);
}

pub fn set_font_dir(path: impl Into<PathBuf>) {
    ResourceLoader::set_font_dir(path);
}

pub fn set_image_dir(path: impl Into<PathBuf>) {
    ResourceLoader::set_image_dir(path);
}

pub fn embed_resource(name: &str, data: Vec<u8>) {
    ResourceLoader::embed_resource(name, data);
}

#[macro_export]
macro_rules! embed_font {
    ($name:expr, $path:expr) => {
        $crate::resource::embed_resource(
            &format!("fonts/{}", $name),
            include_bytes!($path).to_vec()
        )
    };
}

#[macro_export]
macro_rules! embed_image {
    ($name:expr, $path:expr) => {
        $crate::resource::embed_resource(
            &format!("images/{}", $name),
            include_bytes!($path).to_vec()
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_url_base64() {
        let url = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let result = ResourceLoader::parse_data_url(url);
        assert!(result.is_some());
        let data = result.unwrap();
        assert_eq!(&data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
    
    #[test]
    fn test_urlencoding_decode() {
        assert_eq!(urlencoding_decode("hello%20world"), b"hello world");
        assert_eq!(urlencoding_decode("a+b"), b"a b");
    }
}
