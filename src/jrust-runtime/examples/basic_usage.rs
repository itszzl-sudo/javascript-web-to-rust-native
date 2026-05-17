
use jrust_runtime::*;
use jrust_runtime::bom::window::Window;
use jrust_runtime::core::*;

fn main() {
    // Initialize the runtime window
    let mut window = Window::new();
    println!("=== JavaScript Web Runtime ===");
    
    // Test console log
    window.console_log(&[
        JsValue::new_string("Hello"),
        JsValue::new_string("from"),
        JsValue::new_string("JavaScript-Web-Rust-Native!"),
    ]);
    
    // Test alert
    window.alert("This is an alert message!");
    
    // Test DOM operations
    let document = &window.document;
    let div = document.create_element("div");
    let paragraph = document.create_element("p");
    
    println!("\n=== DOM Operations ===");
    println!("Created element: div");
    println!("Created element: p");

    // Test JsValue usage
    println!("\n=== JsValue Examples ===");
    let num = JsValue::new_number(42.0);
    let str = JsValue::new_string("Test string");
    let bool = JsValue::new_boolean(true);
    println!("Number: {}", num);
    println!("String: {}", str);
    println!("Boolean: {}", bool);
    
    // Test conversions
    println!("\n=== Conversions ===");
    println!("Number to boolean: {}", num.to_boolean());
    println!("String to boolean: {}", str.to_boolean());
    println!("Number to string: {}", num.to_string());
}
