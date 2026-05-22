//! Android Platform Support
//!
//! Provides Jetpack Compose integration for JRust on Android.

use jrust_platform::{
    Platform, PlatformError, PlatformEvent, ViewId, EventHandler,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::any::Any;

mod bridge;
mod view;

pub use bridge::AndroidBridge;
pub use view::AndroidView;

/// Android Platform implementation
pub struct AndroidPlatform {
    views: RwLock<HashMap<ViewId, AndroidView>>,
    event_handlers: RwLock<HashMap<(ViewId, String), Vec<EventHandler>>>,
    next_view_id: Mutex<usize>,
    window_size: RwLock<(f64, f64)>,
}

impl AndroidPlatform {
    pub fn new() -> Self {
        Self {
            views: RwLock::new(HashMap::new()),
            event_handlers: RwLock::new(HashMap::new()),
            next_view_id: Mutex::new(1),
            window_size: RwLock::new((360.0, 640.0)), // Default Android size
        }
    }
    
    fn next_id(&self) -> ViewId {
        let mut id = self.next_view_id.lock().unwrap();
        let view_id = ViewId::new(*id);
        *id += 1;
        view_id
    }
}

impl Default for AndroidPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform for AndroidPlatform {
    fn create_view(&self, tag: &str) -> Result<ViewId, PlatformError> {
        let view_id = self.next_id();
        let view = AndroidView::new(view_id, tag);
        
        // In real implementation, this would call JNI to Compose
        // unsafe { bridge::create_compose_view(tag) }
        
        self.views.write().unwrap().insert(view_id, view);
        Ok(view_id)
    }
    
    fn destroy_view(&self, view: ViewId) -> Result<(), PlatformError> {
        self.views.write().unwrap().remove(&view)
            .ok_or(PlatformError::ViewNotFound(view))?;
        Ok(())
    }
    
    fn set_attribute(&self, view: ViewId, key: &str, value: &str) -> Result<(), PlatformError> {
        let mut views = self.views.write().unwrap();
        let v = views.get_mut(&view).ok_or(PlatformError::ViewNotFound(view))?;
        v.attributes.insert(key.to_string(), value.to_string());
        Ok(())
    }
    
    fn get_attribute(&self, view: ViewId, key: &str) -> Result<Option<String>, PlatformError> {
        let views = self.views.read().unwrap();
        let v = views.get(&view).ok_or(PlatformError::ViewNotFound(view))?;
        Ok(v.attributes.get(key).cloned())
    }
    
    fn set_text_content(&self, view: ViewId, text: &str) -> Result<(), PlatformError> {
        let mut views = self.views.write().unwrap();
        let v = views.get_mut(&view).ok_or(PlatformError::ViewNotFound(view))?;
        v.text_content = Some(text.to_string());
        Ok(())
    }
    
    fn get_text_content(&self, view: ViewId) -> Result<Option<String>, PlatformError> {
        let views = self.views.read().unwrap();
        let v = views.get(&view).ok_or(PlatformError::ViewNotFound(view))?;
        Ok(v.text_content.clone())
    }
    
    fn append_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError> {
        let mut views = self.views.write().unwrap();
        let p = views.get_mut(&parent).ok_or(PlatformError::ViewNotFound(parent))?;
        p.children.push(child);
        Ok(())
    }
    
    fn remove_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError> {
        let mut views = self.views.write().unwrap();
        let p = views.get_mut(&parent).ok_or(PlatformError::ViewNotFound(parent))?;
        p.children.retain(|&c| c != child);
        Ok(())
    }
    
    fn insert_before(&self, parent: ViewId, child: ViewId, before: ViewId) -> Result<(), PlatformError> {
        let mut views = self.views.write().unwrap();
        let p = views.get_mut(&parent).ok_or(PlatformError::ViewNotFound(parent))?;
        let pos = p.children.iter().position(|&c| c == before);
        if let Some(pos) = pos {
            p.children.insert(pos, child);
        } else {
            p.children.push(child);
        }
        Ok(())
    }
    
    fn parent(&self, view: ViewId) -> Result<Option<ViewId>, PlatformError> {
        let views = self.views.read().unwrap();
        let v = views.get(&view).ok_or(PlatformError::ViewNotFound(view))?;
        Ok(v.parent)
    }
    
    fn children(&self, view: ViewId) -> Result<Vec<ViewId>, PlatformError> {
        let views = self.views.read().unwrap();
        let v = views.get(&view).ok_or(PlatformError::ViewNotFound(view))?;
        Ok(v.children.clone())
    }
    
    fn add_event_listener(
        &self,
        view: ViewId,
        event_type: &str,
        handler: EventHandler,
    ) -> Result<(), PlatformError> {
        let mut handlers = self.event_handlers.write().unwrap();
        let key = (view, event_type.to_string());
        handlers.entry(key).or_default().push(handler);
        Ok(())
    }
    
    fn remove_event_listener(
        &self,
        view: ViewId,
        event_type: &str,
    ) -> Result<(), PlatformError> {
        let mut handlers = self.event_handlers.write().unwrap();
        let key = (view, event_type.to_string());
        handlers.remove(&key);
        Ok(())
    }
    
    fn window_size(&self) -> (f64, f64) {
        *self.window_size.read().unwrap()
    }
    
    fn set_window_size(&self, width: f64, height: f64) {
        *self.window_size.write().unwrap() = (width, height);
    }
    
    fn request_redraw(&self) {
        // JNI call: bridge::request_recomposition()
    }
    
    fn platform_name(&self) -> &'static str {
        "android"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// JNI initialization
#[no_mangle]
pub extern "jni" fn Java_io_jrust_JRust_init() {
    // Initialize JRust runtime for Android
}

/// Create view from Android
#[no_mangle]
pub extern "jni" fn Java_io_jrust_JRust_createView(tag: jni::strings::JString) -> i64 {
    // Return view ID as jlong
    0
}
