
use std::collections::HashMap;
use crate::core::JsValue;
use crate::dom::events::{Event, EventType, EventTarget};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Element {
    pub tag_name: String,
    pub id: Option<String>,
    pub class_list: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
    pub text_content: String,
    pub inner_html: String,
    #[serde(skip)]
    event_listeners: HashMap<EventType, Vec<Box<dyn Fn(&Event) -> JsValue>>>,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("tag_name", &self.tag_name)
            .field("id", &self.id)
            .field("class_list", &self.class_list)
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .field("text_content", &self.text_content)
            .field("inner_html", &self.inner_html)
            .finish()
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Element {
            tag_name: self.tag_name.clone(),
            id: self.id.clone(),
            class_list: self.class_list.clone(),
            attributes: self.attributes.clone(),
            children: self.children.clone(),
            text_content: self.text_content.clone(),
            inner_html: self.inner_html.clone(),
            event_listeners: HashMap::new(), // 不克隆事件监听器
        }
    }
}

impl Element {
    pub fn new(tag_name: &str) -> Self {
        Element {
            tag_name: tag_name.to_string(),
            id: None,
            class_list: Vec::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text_content: String::new(),
            inner_html: String::new(),
            event_listeners: HashMap::new(),
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn append_child(&mut self, child: Element) {
        self.children.push(child);
    }

    pub fn prepend_child(&mut self, child: Element) {
        self.children.insert(0, child);
    }

    pub fn remove_child(&mut self, index: usize) -> Option<Element> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    pub fn first_child(&self) -> Option<&Element> {
        self.children.first()
    }

    pub fn last_child(&self) -> Option<&Element> {
        self.children.last()
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    pub fn add_class(&mut self, class: &str) {
        if !self.class_list.iter().any(|c| c == class) {
            self.class_list.push(class.to_string());
        }
    }

    pub fn remove_class(&mut self, class: &str) {
        self.class_list.retain(|c| c != class);
    }

    pub fn toggle_class(&mut self, class: &str) {
        let pos = self.class_list.iter().position(|c| c == class);
        if let Some(idx) = pos {
            self.class_list.remove(idx);
        } else {
            self.class_list.push(class.to_string());
        }
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.class_list.iter().any(|c| c == class)
    }

    pub fn text_content(&self) -> &str {
        &self.text_content
    }

    pub fn set_text_content(&mut self, text: &str) {
        self.text_content = text.to_string();
    }

    pub fn inner_html(&self) -> &str {
        &self.inner_html
    }

    pub fn set_inner_html(&mut self, html: &str) {
        self.inner_html = html.to_string();
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<&Element> {
        let mut result = Vec::new();
        
        if let Some(id) = selector.strip_prefix('#') {
            if self.id.as_deref() == Some(id) {
                result.push(self);
            }
            for child in &self.children {
                result.extend(child.query_selector_all(selector));
            }
        } else if let Some(class) = selector.strip_prefix('.') {
            if self.has_class(class) {
                result.push(self);
            }
            for child in &self.children {
                result.extend(child.query_selector_all(selector));
            }
        } else {
            if self.tag_name.to_lowercase() == selector.to_lowercase() {
                result.push(self);
            }
            for child in &self.children {
                result.extend(child.query_selector_all(selector));
            }
        }
        
        result
    }

    pub fn query_selector(&self, selector: &str) -> Option<&Element> {
        self.query_selector_all(selector).first().copied()
    }
}

impl EventTarget for Element {
    fn add_event_listener(&mut self, event_type: EventType, handler: Box<dyn Fn(&Event) -> JsValue>) {
        self.event_listeners.entry(event_type).or_insert_with(Vec::new).push(handler);
    }

    fn remove_event_listener(&mut self, _event_type: EventType, _handler: Box<dyn Fn(&Event) -> JsValue>) {
        // Implementation for removing event listeners
    }

    fn dispatch_event(&self, event: Event) {
        if let Some(handlers) = self.event_listeners.get(&event.event_type()) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}
