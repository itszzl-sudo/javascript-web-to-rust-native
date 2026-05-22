//! Platform Abstraction Layer
//!
//! Provides a unified interface for cross-platform UI rendering.

use std::sync::Arc;
use std::any::Any;

pub mod platform;
pub mod view;
pub mod event;

pub use platform::{Platform, PlatformError};
pub use view::{ViewId, ViewTrait};
pub use event::{PlatformEvent, EventHandler};

/// Platform-agnostic view identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(pub usize);

impl ViewId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// Event handler type
pub type EventHandler = Arc<dyn Fn(PlatformEvent) + Send + Sync>;

/// Platform-specific error
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("View not found: {0}")]
    ViewNotFound(ViewId),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Platform error: {0}")]
    PlatformSpecific(String),
    
    #[error("FFI error: {0}")]
    FfiError(String),
}

/// Platform abstraction trait
pub trait Platform: Send + Sync {
    /// Create a new view with the given tag name
    fn create_view(&self, tag: &str) -> Result<ViewId, PlatformError>;
    
    /// Destroy a view
    fn destroy_view(&self, view: ViewId) -> Result<(), PlatformError>;
    
    /// Set attribute on a view
    fn set_attribute(&self, view: ViewId, key: &str, value: &str) -> Result<(), PlatformError>;
    
    /// Get attribute from a view
    fn get_attribute(&self, view: ViewId, key: &str) -> Result<Option<String>, PlatformError>;
    
    /// Set text content
    fn set_text_content(&self, view: ViewId, text: &str) -> Result<(), PlatformError>;
    
    /// Get text content
    fn get_text_content(&self, view: ViewId) -> Result<Option<String>, PlatformError>;
    
    /// Append child view
    fn append_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError>;
    
    /// Remove child view
    fn remove_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError>;
    
    /// Insert child before another child
    fn insert_before(&self, parent: ViewId, child: ViewId, before: ViewId) -> Result<(), PlatformError>;
    
    /// Get parent view
    fn parent(&self, view: ViewId) -> Result<Option<ViewId>, PlatformError>;
    
    /// Get children
    fn children(&self, view: ViewId) -> Result<Vec<ViewId>, PlatformError>;
    
    /// Add event listener
    fn add_event_listener(
        &self,
        view: ViewId,
        event_type: &str,
        handler: EventHandler,
    ) -> Result<(), PlatformError>;
    
    /// Remove event listener
    fn remove_event_listener(
        &self,
        view: ViewId,
        event_type: &str,
    ) -> Result<(), PlatformError>;
    
    /// Get window size
    fn window_size(&self) -> (f64, f64);
    
    /// Set window size
    fn set_window_size(&self, width: f64, height: f64);
    
    /// Request redraw
    fn request_redraw(&self);
    
    /// Get platform name
    fn platform_name(&self) -> &'static str;
    
    /// Downcast to platform-specific type
    fn as_any(&self) -> &dyn Any;
}

/// Platform event types
#[derive(Debug, Clone)]
pub enum PlatformEvent {
    Click {
        x: f64,
        y: f64,
    },
    TouchStart {
        x: f64,
        y: f64,
        id: i32,
    },
    TouchMove {
        x: f64,
        y: f64,
        id: i32,
    },
    TouchEnd {
        x: f64,
        y: f64,
        id: i32,
    },
    KeyDown {
        key: String,
        code: String,
    },
    KeyUp {
        key: String,
        code: String,
    },
    Resize {
        width: f64,
        height: f64,
    },
    Custom {
        name: String,
        data: Box<dyn Any + Send + Sync>,
    },
}

/// View trait for platform-specific views
pub trait ViewTrait: Send + Sync {
    fn id(&self) -> ViewId;
    fn tag_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
