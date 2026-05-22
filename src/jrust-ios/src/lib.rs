//! iOS Platform Support
//!
//! Provides SwiftUI/UIKit integration for JRust on iOS.

use jrust_platform::{
    Platform, PlatformError, PlatformEvent, ViewId, EventHandler,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::any::Any;

mod bridge;
mod view;

pub use bridge::IosBridge;
pub use view::IosView;

/// iOS Platform implementation
pub struct IosPlatform {
    views: RwLock<HashMap<ViewId, IosView>>,
    event_handlers: RwLock<HashMap<(ViewId, String), Vec<EventHandler>>>,
    next_view_id: Mutex<usize>,
    window_size: RwLock<(f64, f64)>,
}

impl IosPlatform {
    pub fn new() -> Self {
        Self {
            views: RwLock::new(HashMap::new()),
            event_handlers: RwLock::new(HashMap::new()),
            next_view_id: Mutex::new(1),
            window_size: RwLock::new((375.0, 812.0)), // iPhone X default
        }
    }
    
    fn next_id(&self) -> ViewId {
        let mut id = self.next_view_id.lock().unwrap();
        let view_id = ViewId::new(*id);
        *id += 1;
        view_id
    }
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform for IosPlatform {
    fn create_view(&self, tag: &str) -> Result<ViewId, PlatformError> {
        let view_id = self.next_id();
        let view = IosView::new(view_id, tag);
        
        // In real implementation, this would call UIKit/SwiftUI FFI
        // unsafe { bridge::create_native_view(tag) }
        
        self.views.write().unwrap().insert(view_id, view);
        Ok(view_id)
    }
    
    fn destroy_view(&self, view: ViewId) -> Result<(), PlatformError> {
        self.views.write().unwrap().remove(&view)
            .ok_or(PlatformError::ViewNotFound(view))?;
        Ok(())
    }
    
    fn set_attribute(&self, view: ViewId, key: &str, value: &str) -> Result<(), PlatformError> {
        let views = self.views.read().unwrap();
        let _v = views.get(&view).ok_or(PlatformError::ViewNotFound(view))?;
        
        // FFI call: bridge::set_attribute(view, key, value)
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
        // FFI call: bridge::set_needs_display()
    }
    
    fn platform_name(&self) -> &'static str {
        "ios"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
