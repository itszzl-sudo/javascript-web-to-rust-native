use crate::dom::element::Element;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    body: Element,
    title: String,
}

impl Document {
    pub fn new() -> Self {
        let body = Element::new("body");
        Document {
            body,
            title: String::from("Untitled Document"),
        }
    }

    pub fn create_element(&self, tag_name: &str) -> Element {
        Element::new(tag_name)
    }

    pub fn create_text_node(&self, text: &str) -> Element {
        let mut text_node = Element::new("text");
        text_node.set_text_content(text);
        text_node
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn body(&self) -> &Element {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Element {
        &mut self.body
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<&Element> {
        self.body.query_selector(&format!("#{}", id))
    }

    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Element> {
        self.body.query_selector_all(tag_name)
    }

    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<&Element> {
        self.body.query_selector_all(&format!(".{}", class_name))
    }

    pub fn query_selector(&self, selector: &str) -> Option<&Element> {
        self.body.query_selector(selector)
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<&Element> {
        self.body.query_selector_all(selector)
    }

    pub fn append_to_body(&mut self, element: Element) {
        self.body.append_child(element);
    }

    pub fn prepend_to_body(&mut self, element: Element) {
        self.body.prepend_child(element);
    }
}
