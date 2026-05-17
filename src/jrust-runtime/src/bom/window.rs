
use crate::core::JsValue;
use crate::dom::document::Document;

pub struct Window {
    pub document: Document,
}

impl Window {
    pub fn new() -> Self {
        Window {
            document: Document::new(),
        }
    }

    pub fn alert(&self, message: &str) {
        println!("Alert: {}", message);
    }

    pub fn console_log(&self, args: &[JsValue]) {
        let mut message = String::new();
        for arg in args {
            message.push_str(&arg.to_string());
            message.push(' ');
        }
        println!("Console.log: {}", message);
    }
}
