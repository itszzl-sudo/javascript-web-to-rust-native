//! 性能基准测试示例

use jrust_runtime::core::{JsValue, JsObject};
use std::time::Instant;

fn benchmark_js_object_ops() {
    let start = Instant::now();
    let mut obj = JsObject::new();
    
    for i in 0..10000 {
        obj.set(format!("key{}", i), JsValue::new_number(i as f64));
    }
    
    for i in 0..10000 {
        let _ = obj.get(&format!("key{}", i));
    }
    
    let duration = start.elapsed();
    println!("JsObject operations (10,000): {:?}", duration);
}

fn benchmark_js_value_creation() {
    let start = Instant::now();
    
    let mut values = Vec::with_capacity(100000);
    for i in 0..100000 {
        values.push(JsValue::new_number(i as f64));
    }
    
    let duration = start.elapsed();
    println!("JsValue creation (100,000): {:?}", duration);
}

fn main() {
    println!("=== jrust-runtime Benchmarks ===");
    
    benchmark_js_value_creation();
    benchmark_js_object_ops();
    
    println!("=== Done ===");
}
