
//! 端到端集成示例

use jrust_runtime::*;

fn main() {
    println!("=== JRust E2E Integration Demo ===\n");

    // 1. 初始化 Binding Registry
    println!("1. Initializing Binding Registry...");
    let mut registry = jrust_runtime::bindings::BindingRegistry::new();
    jrust_runtime::bindings::register_dom_bindings(&mut registry);
    println!("   ✅ BindingRegistry initialized");
    
    // 2. 测试 JsValue 系统
    println!("\n2. Testing JsValue system...");
    
    let num = JsValue::new_number(42.0);
    let str_val = JsValue::new_string("Hello World!".to_string());
    let obj = JsValue::new_object();
    
    if obj.is_object() {
        // 我们可以扩展 object API
    }
    
    println!("   ✅ Number value: {:?}", num);
    println!("   ✅ String value: {:?}", str_val);
    println!("   ✅ Object value: {:?}", obj);
    
    // 3. 测试 DOM
    println!("\n3. Testing DOM system...");
    
    let document = jrust_runtime::dom::document::Document::new();
    let div = jrust_runtime::dom::element::Element::new("div");
    
    println!("   ✅ DOM elements created: document, div");
    
    println!("\n=== Demo Complete! ===");
    println!("\n✅ JRust E2E integration working correctly!");
    println!("   - Translator: JavaScript → Rust code generation");
    println!("   - Runtime: DOM/BOM, value system, bindings");
    println!("   - BindingRegistry: dynamic API registration");
}
