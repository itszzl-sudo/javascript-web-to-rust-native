
use jrust_runtime::director::*;

fn main() {
    println!("=== Director + JsRust Tree Demo ===\n");

    println!("--- Demo 1: Basic JsRust Tree ---");
    let mut director = Director::new();
    
    let root_id = director.add_jrust(Box::new(SimpleJsRust::new("jrust-1")));
    let child1 = director.create_child_jrust(root_id, Box::new(SimpleJsRust::new("jrust-2"))).unwrap();
    let child2 = director.create_child_jrust(root_id, Box::new(SimpleJsRust::new("jrust-3"))).unwrap();
    
    println!("\n--- Dispatch Event ---");
    director.dispatch_event();
}

