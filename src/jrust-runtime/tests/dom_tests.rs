
use jrust_runtime::dom::*;

#[test]
fn test_element_creation() {
    let elem = element::Element::new("div");
    assert_eq!(elem.tag_name, "div");
    assert!(elem.id().is_none());
    assert_eq!(elem.class_list.len(), 0);
    assert_eq!(elem.children.len(), 0);
}

#[test]
fn test_element_attributes() {
    let mut elem = element::Element::new("div");
    
    elem.set_attribute("id", "my-div");
    elem.set_attribute("class", "container");
    
    assert_eq!(elem.get_attribute("id"), Some("my-div"));
    assert_eq!(elem.get_attribute("class"), Some("container"));
    
    elem.remove_attribute("class");
    assert!(elem.get_attribute("class").is_none());
}

#[test]
fn test_element_classes() {
    let mut elem = element::Element::new("div");
    
    elem.add_class("foo");
    elem.add_class("bar");
    assert!(elem.has_class("foo"));
    assert!(elem.has_class("bar"));
    
    elem.remove_class("foo");
    assert!(!elem.has_class("foo"));
    
    elem.toggle_class("bar");
    assert!(!elem.has_class("bar"));
    
    elem.toggle_class("bar");
    assert!(elem.has_class("bar"));
}

#[test]
fn test_element_children() {
    let mut parent = element::Element::new("div");
    let child1 = element::Element::new("p");
    let child2 = element::Element::new("span");
    
    parent.append_child(child1);
    parent.append_child(child2);
    
    assert_eq!(parent.child_count(), 2);
    
    let removed = parent.remove_child(0);
    assert!(removed.is_some());
    assert_eq!(parent.child_count(), 1);
}

#[test]
fn test_element_text_content() {
    let mut elem = element::Element::new("div");
    
    elem.set_text_content("Hello, world!");
    assert_eq!(elem.text_content(), "Hello, world!");
}

#[test]
fn test_document_creation() {
    let doc = document::Document::new();
    
    assert_eq!(doc.title(), "Untitled Document");
    assert_eq!(doc.body().tag_name, "body");
}

#[test]
fn test_document_title() {
    let mut doc = document::Document::new();
    
    doc.set_title("My Page");
    assert_eq!(doc.title(), "My Page");
}

#[test]
fn test_document_create_element() {
    let doc = document::Document::new();
    
    let elem = doc.create_element("div");
    assert_eq!(elem.tag_name, "div");
}

#[test]
fn test_document_append_to_body() {
    let mut doc = document::Document::new();
    
    let mut elem = doc.create_element("div");
    elem.set_id("my-div");
    
    doc.append_to_body(elem);
    
    let found = doc.get_element_by_id("my-div");
    assert!(found.is_some());
}

#[test]
fn test_document_query_selector() {
    let mut doc = document::Document::new();
    
    let mut div1 = doc.create_element("div");
    div1.add_class("container");
    
    let mut div2 = doc.create_element("div");
    div2.set_id("content");
    
    doc.append_to_body(div1);
    doc.append_to_body(div2);
    
    let by_id = doc.query_selector("#content");
    assert!(by_id.is_some());
    
    let by_class = doc.query_selector_all(".container");
    assert_eq!(by_class.len(), 1);
    
    let by_tag = doc.query_selector_all("div");
    assert_eq!(by_tag.len(), 2);
}

#[test]
fn test_element_query_selector() {
    let mut parent = element::Element::new("div");
    
    let mut child1 = element::Element::new("p");
    child1.add_class("text");
    
    let mut child2 = element::Element::new("span");
    child2.set_id("highlight");
    
    parent.append_child(child1);
    parent.append_child(child2);
    
    let by_id = parent.query_selector("#highlight");
    assert!(by_id.is_some());
    
    let by_class = parent.query_selector_all(".text");
    assert_eq!(by_class.len(), 1);
}
