//! Vue场景测试套件
//! 测试Vue预编译和序列化相关功能

use jrust_runtime::dom::{document, element, events};
use jrust_runtime::core::JsValue;
use jrust_runtime::events::EventTarget;

#[test]
fn test_element_serialization() {
    let mut elem = element::Element::new("div");
    elem.set_id("app");
    elem.add_class("container");
    elem.set_attribute("data-v-app", "");
    elem.set_text_content("Hello Vue");

    let serialized = serde_json::to_string(&elem).unwrap();
    assert!(serialized.contains("\"tag_name\":\"div\""));
    assert!(serialized.contains("\"id\":\"app\""));
    assert!(serialized.contains("\"class_list\":[\"container\"]"));
    assert!(serialized.contains("\"text_content\":\"Hello Vue\""));
}

#[test]
fn test_element_deserialization() {
    let mut elem = element::Element::new("div");
    elem.set_id("app");
    elem.set_text_content("Hello Vue");

    let serialized = serde_json::to_string(&elem).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.tag_name, "div");
    assert_eq!(deserialized.id(), Some("app"));
    assert_eq!(deserialized.text_content(), "Hello Vue");
}

#[test]
fn test_document_serialization() {
    let mut doc = document::Document::new();
    doc.set_title("Vue App");

    let mut app = element::Element::new("div");
    app.set_id("app");

    let mut h1 = element::Element::new("h1");
    h1.set_text_content("Welcome");
    app.append_child(h1);

    doc.append_to_body(app);

    let serialized = serde_json::to_string(&doc).unwrap();
    assert!(serialized.contains("\"title\":\"Vue App\""));
    assert!(serialized.contains("\"id\":\"app\""));
    assert!(serialized.contains("\"text_content\":\"Welcome\""));
}

#[test]
fn test_vue_component_tree_serialization() {
    let mut root = element::Element::new("div");
    root.set_id("root");

    let mut header = element::Element::new("header");
    header.add_class("header");

    let mut nav = element::Element::new("nav");
    nav.add_class("nav");
    header.append_child(nav);

    let mut main = element::Element::new("main");
    main.add_class("main-content");

    let mut article = element::Element::new("article");
    article.set_text_content("Article content");
    main.append_child(article);

    root.append_child(header);
    root.append_child(main);

    let serialized = serde_json::to_string(&root).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id(), Some("root"));
    assert_eq!(deserialized.children.len(), 2);
    assert_eq!(deserialized.children[0].tag_name, "header");
    assert_eq!(deserialized.children[1].tag_name, "main");
}

#[test]
fn test_event_listeners_not_serialized() {
    let mut elem = element::Element::new("button");

    elem.add_event_listener(events::EventType::Click, Box::new(|_| JsValue::new_undefined()));

    let serialized = serde_json::to_string(&elem).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.tag_name, "button");
}

#[test]
fn test_nested_component_serialization() {
    let mut app = element::Element::new("div");
    app.set_id("app");

    let mut card = element::Element::new("div");
    card.add_class("card");

    let mut card_header = element::Element::new("div");
    card_header.add_class("card-header");
    card_header.set_text_content("Card Title");

    let mut card_body = element::Element::new("div");
    card_body.add_class("card-body");
    card_body.set_text_content("Card Content");

    card.append_child(card_header);
    card.append_child(card_body);

    app.append_child(card);

    let serialized = serde_json::to_string(&app).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    let card_elem = deserialized.query_selector(".card").unwrap();
    assert_eq!(card_elem.children.len(), 2);
}

#[test]
fn test_vue_sfc_style_dom_structure() {
    let mut template = element::Element::new("div");
    template.set_id("app");

    let mut button = element::Element::new("button");
    button.add_class("btn");
    button.add_class("btn-primary");
    button.set_attribute(":class", "{ active: isActive }");
    button.set_text_content("Click me");

    let mut input = element::Element::new("input");
    input.set_attribute("v-model", "inputValue");
    input.set_attribute("type", "text");
    input.set_attribute("placeholder", "Enter text");

    template.append_child(button);
    template.append_child(input);

    let serialized = serde_json::to_string(&template).unwrap();

    assert!(serialized.contains("\"class_list\":[\"btn\",\"btn-primary\"]"));
    assert!(serialized.contains("\"v-model\""));
    assert!(serialized.contains("\":class\""));
}

#[test]
fn test_element_clone_preserves_structure() {
    let mut elem = element::Element::new("div");
    elem.set_id("original");
    elem.add_class("container");

    let mut child = element::Element::new("p");
    child.set_text_content("Child text");
    elem.append_child(child);

    let cloned = elem.clone();

    assert_eq!(cloned.id(), Some("original"));
    assert!(cloned.has_class("container"));
    assert_eq!(cloned.children.len(), 1);
    assert_eq!(cloned.children[0].text_content, "Child text");
}

#[test]
fn test_complex_dom_tree_serialization() {
    let mut root = element::Element::new("div");
    root.set_id("app");

    for i in 1..=5 {
        let mut item = element::Element::new("li");
        item.set_attribute("key", &i.to_string());
        item.set_text_content(&format!("Item {}", i));
        root.append_child(item);
    }

    let serialized = serde_json::to_string(&root).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.children.len(), 5);
}

#[test]
fn test_vue_template_with_attributes() {
    let mut elem = element::Element::new("img");
    elem.set_attribute("src", "/logo.png");
    elem.set_attribute("alt", "Vue Logo");
    elem.set_attribute("v-bind:title", "logo");
    elem.set_attribute("@click", "handleClick");

    let serialized = serde_json::to_string(&elem).unwrap();
    let deserialized: element::Element = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.get_attribute("src"), Some("/logo.png"));
    assert_eq!(deserialized.get_attribute("alt"), Some("Vue Logo"));
}
