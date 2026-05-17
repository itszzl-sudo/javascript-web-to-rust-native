//! Vue Demo - 展示Vue预编译后的代码如何在jrust-runtime中运行

use jrust_runtime::document::Document;
use jrust_runtime::element::Element;
use jrust_runtime::events::{EventType, EventTarget};
use jrust_runtime::core::{JsValue, JsObject};
use std::rc::Rc;
use std::cell::RefCell;

struct VueComponent {
    name: String,
    data: JsObject,
    document: Document,
}

impl VueComponent {
    fn new(name: &str) -> Self {
        VueComponent {
            name: name.to_string(),
            data: JsObject::new(),
            document: Document::new(),
        }
    }

    fn init_data(&mut self, props: &[(&str, JsValue)]) {
        for (key, value) in props {
            self.data.set(*key, value.clone());
        }
    }

    fn render(&mut self) -> Element {
        let title = self.data.get("title")
            .map(|v| v.clone())
            .unwrap_or_else(|| JsValue::new_string("Hello"));

        let mut app = Element::new("div");
        app.set_id("app");

        let mut h1 = Element::new("h1");
        h1.set_text_content(&title.to_string());
        app.append_child(h1);

        let mut button = Element::new("button");
        button.set_text_content("Click me");

        let data_ref = Rc::new(RefCell::new(self.data.clone()));
        button.add_event_listener(EventType::Click, Box::new(move |_event| {
            let mut data = data_ref.borrow_mut();
            let count = data.get("count")
                .map(|v| v.to_number() as i32)
                .unwrap_or(0);
            data.set("count", JsValue::new_number((count + 1) as f64));
            println!("Button clicked! Count: {}", count + 1);
            JsValue::new_undefined()
        }));

        app.append_child(button);

        app
    }

    fn mount(&mut self) {
        let app_element = self.render();
        self.document.append_to_body(app_element);
        println!("{} 组件挂载完成", self.name);
    }

    fn update(&mut self, updates: &[(&str, JsValue)]) {
        for (key, value) in updates {
            self.data.set(*key, value.clone());
        }
        println!("{} 组件更新", self.name);
    }
}

fn main() {
    println!("=== Vue Demo with jrust-runtime ===\n");

    let mut app = VueComponent::new("App");

    app.init_data(&[
        ("title", JsValue::new_string("JRust Vue Demo")),
        ("count", JsValue::new_number(0.0)),
    ]);

    app.mount();

    println!("\n--- 模拟用户交互 ---");

    if let Some(_button) = app.document.get_element_by_id("app")
        .and_then(|app_elem: &Element| app_elem.query_selector("button"))
    {
        println!("模拟点击按钮...");
    }

    app.update(&[("title", JsValue::new_string("Updated Title"))]);

    println!("\n--- Demo 完成 ---");
    println!("组件数据: {:?}", app.data);
}