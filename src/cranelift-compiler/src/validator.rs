use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidateLevel {
    Syntax,
    Full,
}

#[derive(Debug)]
pub enum ValidationError {
    SyntaxError(String),
    TypeError(String),
    IoError(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::SyntaxError(msg) => write!(f, "语法错误: {}", msg),
            ValidationError::TypeError(msg) => write!(f, "类型错误: {}", msg),
            ValidationError::IoError(msg) => write!(f, "IO错误: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

pub struct Validator {
    temp_dir: PathBuf,
}

impl Validator {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir();
        Self { temp_dir }
    }
    
    pub fn validate(&self, code: &str, level: ValidateLevel) -> Result<(), ValidationError> {
        self.validate_syntax(code)?;
        
        if level == ValidateLevel::Full {
            self.validate_full(code)?;
        }
        
        Ok(())
    }
    
    pub fn validate_syntax(&self, code: &str) -> Result<(), ValidationError> {
        let _parsed: syn::File = syn::parse_str(code)
            .map_err(|e| ValidationError::SyntaxError(e.to_string()))?;
        
        println!("✅ syn 验证通过：语法正确");
        Ok(())
    }
    
    pub fn validate_full(&self, code: &str) -> Result<(), ValidationError> {
        let temp_file = self.temp_dir.join("jrust_validate.rs");
        
        fs::write(&temp_file, code)
            .map_err(|e| ValidationError::IoError(e.to_string()))?;
        
        let output = Command::new("rustc")
            .args(&[
                "--edition", "2021",
                "--emit=metadata",
                "-o", self.temp_dir.join("jrust_validate.rmeta").to_str().unwrap(),
                temp_file.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| ValidationError::IoError(e.to_string()))?;
        
        let _ = fs::remove_file(&temp_file);
        let _ = fs::remove_file(self.temp_dir.join("jrust_validate.rmeta"));
        
        if output.status.success() {
            println!("✅ rustc 验证通过：类型正确");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ValidationError::TypeError(stderr.to_string()))
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_jrust(code: &str, level: ValidateLevel) -> Result<(), ValidationError> {
    let validator = Validator::new();
    validator.validate(code, level)
}
