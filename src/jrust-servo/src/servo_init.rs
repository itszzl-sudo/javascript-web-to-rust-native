
//! Servo 初始化配置

/// Servo 初始化配置
#[derive(Debug, Clone)]
pub struct ServoConfig {
    /// 禁用默认首页
    pub load_default_page: bool,
    
    /// 禁用书签栏
    pub enable_bookmarks: bool,
    
    /// 禁用 Cookie
    pub enable_cookies: bool,
    
    /// 禁用 LocalStorage
    pub enable_local_storage: bool,
    
    /// 窗口宽度
    pub window_width: u32,
    
    /// 窗口高度
    pub window_height: u32,
    
    /// 窗口标题
    pub window_title: String,
}

impl Default for ServoConfig {
    fn default() -> Self {
        Self {
            // 核心：禁用所有默认行为
            load_default_page: false,
            enable_bookmarks: false,
            enable_cookies: false,
            enable_local_storage: false,
            
            // 默认窗口配置
            window_width: 1280,
            window_height: 720,
            window_title: "jrust App".to_string(),
        }
    }
}

impl ServoConfig {
    /// 创建最小化的配置（禁用所有默认行为）
    pub fn minimal() -> Self {
        Self::default()
    }
    
    /// 设置窗口大小
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }
    
    /// 设置窗口标题
    pub fn with_window_title(mut self, title: String) -> Self {
        self.window_title = title;
        self
    }
}

/// Servo 实例（简化版，用于后续对接真实 Servo）
pub struct ServoInstance {
    config: ServoConfig,
    initialized: bool,
}

impl ServoInstance {
    /// 创建新的 Servo 实例
    pub fn new(config: ServoConfig) -> Self {
        Self {
            config,
            initialized: false,
        }
    }
    
    /// 初始化 Servo
    pub fn init(&mut self) -> crate::error::Result<()> {
        println!("=== Servo 初始化 ===");
        println!("• 加载默认首页: {}", self.config.load_default_page);
        println!("• 启用书签栏: {}", self.config.enable_bookmarks);
        println!("• 启用 Cookie: {}", self.config.enable_cookies);
        println!("• 启用 LocalStorage: {}", self.config.enable_local_storage);
        println!("• 窗口大小: {}x{}", self.config.window_width, self.config.window_height);
        println!("• 窗口标题: {}", self.config.window_title);
        
        self.initialized = true;
        println!("✅ Servo 初始化完成！（真实 Servo 集成待实现）");
        
        Ok(())
    }
    
    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
