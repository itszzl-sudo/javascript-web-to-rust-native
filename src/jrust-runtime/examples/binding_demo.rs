
//! Binding Generator 使用示例
//! 演示如何使用 BindingRegistry

use jrust_runtime::*;

fn main() {
    println!("=== JRust Binding Registry Demo ===\n");

    // 1. 创建并初始化 BindingRegistry
    let mut registry = jrust_runtime::bindings::BindingRegistry::new();
    jrust_runtime::bindings::register_dom_bindings(&mut registry);
    println!("✓ DOM bindings registered\n");

    // 2. 添加自定义的绑定
    registry.register("math.add", |args| -> Result<JsValue, String> {
        if args.len() != 2 {
            return Err("Expected 2 arguments for math.add".to_string());
        }
        let a = args[0].as_number().ok_or("First argument must be a number")?;
        let b = args[1].as_number().ok_or("Second argument must be a number")?;
        Ok(JsValue::new_number(a + b))
    });

    // 3. 测试我们的绑定
    println!("=== Testing Bindings ===\n");

    // Test math.add
    match registry.call("math.add", &[JsValue::new_number(42.0), JsValue::new_number(100.0)]) {
        Ok(result) => println!("math.add(42, 100) = {:?}", result),
        Err(e) => println!("math.add error: {}", e),
    }

    // Test DOM binding
    match registry.call("document.createElement", &[JsValue::new_string("div".to_string())]) {
        Ok(result) => println!("\ndocument.createElement('div') = {:?}", result),
        Err(e) => println!("\ndocument.createElement error: {}", e),
    }

    println!("\n=== Demo Complete ===\n");
    println!("✓ Binding system functional!\n");
    println!("Next Steps:");
    println!("1. Expand as_*() helper methods on JsValue");
    println!("2. Integrate with jrust-translator");
    println!("3. Add DOM event binding support");
}
