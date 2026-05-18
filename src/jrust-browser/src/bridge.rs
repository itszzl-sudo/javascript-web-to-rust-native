
//! Browser Bridge - Integration with rust-browser
//! 
//! This module provides a high-level interface to rust-browser's WebNativeBridge

use rust_browser::bridge::WebNativeBridge;
use serde::{Serialize, Deserialize};

/// Browser configuration
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window title
    pub title: String,
    /// Enable JavaScript
    pub enable_js: bool,
    /// Enable GUI
    pub enable_gui: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "JRust App".to_string(),
            enable_js: true,
            enable_gui: false,
        }
    }
}

impl BrowserConfig {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_js(mut self, enable: bool) -> Self {
        self.enable_js = enable;
        self
    }

    pub fn with_gui(mut self, enable: bool) -> Self {
        self.enable_gui = enable;
        self
    }
}

/// Browser instance wrapping WebNativeBridge
pub struct BrowserInstance {
    bridge: WebNativeBridge,
    config: BrowserConfig,
}

impl BrowserInstance {
    /// Create a new browser instance
    pub fn new(config: BrowserConfig) -> Result<Self, String> {
        println!("=== Browser Instance Creation ===");
        println!("Size: {}x{}", config.width, config.height);
        println!("Title: {}", config.title);
        println!("JS Enabled: {}", config.enable_js);
        println!("GUI Enabled: {}", config.enable_gui);

        let bridge = WebNativeBridge::new(config.width, config.height);

        println!("✅ Browser instance created successfully!");

        Ok(Self { bridge, config })
    }

    /// Set HTML content
    pub fn set_html(&mut self, html: &str) -> Result<(), String> {
        self.bridge.set_html(html);
        Ok(())
    }

    /// Set CSS content
    pub fn set_css(&mut self, css: &str) -> Result<(), String> {
        self.bridge.set_css(css);
        Ok(())
    }

    /// Set style for an element
    pub fn set_style(&mut self, selector: &str, property: &str, value: &str) -> Result<(), String> {
        self.bridge.set_style(selector, property, value);
        Ok(())
    }

    /// Query element by selector
    pub fn query(&self, selector: &str) -> Option<usize> {
        self.bridge.query(selector)
    }

    /// Set attribute on element
    pub fn set_attr(&mut self, node_id: usize, name: &str, value: &str) -> Result<(), String> {
        self.bridge.set_attr(node_id, name, value);
        Ok(())
    }

    /// Get attribute from element
    pub fn get_attr(&self, node_id: usize, name: &str) -> Option<String> {
        self.bridge.get_attr(node_id, name)
    }

    /// Get text content of element
    pub fn text(&self, node_id: usize) -> Option<String> {
        self.bridge.text(node_id)
    }

    /// Execute JavaScript
    pub fn eval_js(&mut self, code: &str) -> String {
        self.bridge.eval_js(code)
    }

    /// Render the current state and get PNG bytes
    pub fn render(&mut self) -> Vec<u8> {
        self.bridge.render()
    }

    /// Get element rectangle
    pub fn get_rect(&self, selector: &str) -> Option<(f32, f32, f32, f32)> {
        self.bridge.get_rect(selector)
    }

    /// Get all element rectangles
    pub fn all_rects(&self) -> Vec<(usize, String, f32, f32, f32, f32)> {
        self.bridge.all_rects()
    }

    /// Handle click event
    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        self.bridge.handle_click(x, y)
    }

    /// Bind click event handler
    pub fn on_click<F>(&mut self, selector: &str, handler: F)
    where
        F: FnMut(f32, f32) + Send + 'static,
    {
        self.bridge.on_click(selector, Box::new(handler));
    }

    /// Get viewport size
    pub fn viewport(&self) -> (u32, u32) {
        self.bridge.viewport()
    }

    /// Set viewport size
    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.bridge.set_viewport(width, height);
    }

    /// Get DOM reference
    pub fn dom(&self) -> &rust_browser::dom_wrapper::DomWrapper {
        self.bridge.dom()
    }

    /// Get DOM mutable reference
    pub fn dom_mut(&mut self) -> &mut rust_browser::dom_wrapper::DomWrapper {
        self.bridge.dom_mut()
    }
}

/// Browser events for jrust-browser event handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JrustBrowserEvent {
    /// Click event
    Click {
        selector: String,
        x: f32,
        y: f32,
    },
    /// Form submit event
    FormSubmit {
        selector: String,
        fields: std::collections::HashMap<String, String>,
    },
    /// Custom JS event
    JsEvent {
        code: String,
        result: Option<String>,
    },
}
