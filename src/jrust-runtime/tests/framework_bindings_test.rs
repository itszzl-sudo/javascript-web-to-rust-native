//! 框架绑定测试

use jrust_runtime::bindings::{BindingRegistry, register_all_framework_bindings};

#[test]
fn test_svelte_element() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("svelte_element", &[jrust_runtime::core::JsValue::String("button".to_string())]);
    
    assert!(result.is_ok());
}

#[test]
fn test_svelte_text() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("svelte_text", &[jrust_runtime::core::JsValue::String("Hello".to_string())]);
    
    assert!(result.is_ok());
}

#[test]
fn test_svelte_attr() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("svelte_attr", &[
        jrust_runtime::core::JsValue::new_object(),
        jrust_runtime::core::JsValue::String("class".to_string()),
        jrust_runtime::core::JsValue::String("btn".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_preact_h() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("preact_h", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_preact_h_with_children() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("preact_h", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
        jrust_runtime::core::JsValue::String("Hello".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_solid_create_signal() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("solid_createSignal", &[jrust_runtime::core::JsValue::new_number(0.0)]);
    
    assert!(result.is_ok());
}

#[test]
fn test_solid_create_effect() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("solid_createEffect", &[
        jrust_runtime::core::JsValue::new_function(|_| Ok(jrust_runtime::core::JsValue::Undefined)),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_solid_create_memo() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("solid_createMemo", &[
        jrust_runtime::core::JsValue::new_function(|_| Ok(jrust_runtime::core::JsValue::new_number(42.0))),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_all_bindings_registered() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    assert!(registry.call("svelte_element", &[jrust_runtime::core::JsValue::String("div".to_string())]).is_ok());
    assert!(registry.call("preact_h", &[jrust_runtime::core::JsValue::String("div".to_string())]).is_ok());
    assert!(registry.call("solid_createSignal", &[]).is_ok());
}
