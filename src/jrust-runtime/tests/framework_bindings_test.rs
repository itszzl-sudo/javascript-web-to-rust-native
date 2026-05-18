//! 框架绑定测试

use jrust_runtime::bindings::{BindingRegistry, register_all_framework_bindings};

// ==================== Svelte Tests ====================

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

// ==================== Preact Tests ====================

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

// ==================== SolidJS Tests ====================

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

// ==================== React Tests ====================

#[test]
fn test_react_create_element() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_create_element_with_props() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let props = jrust_runtime::core::JsObject::new();
    props.set("className", jrust_runtime::core::JsValue::String("container".to_string()));
    props.set("id", jrust_runtime::core::JsValue::String("app".to_string()));
    
    let result = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::Object(props),
    ]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.get("data").is_some());
    }
}

#[test]
fn test_react_create_element_with_children() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
        jrust_runtime::core::JsValue::String("Hello".to_string()),
        jrust_runtime::core::JsValue::String("World".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_use_state() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useState", &[jrust_runtime::core::JsValue::new_number(0.0)]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Array(arr) = result.unwrap() {
        assert_eq!(arr.len(), 2);
    }
}

#[test]
fn test_react_use_ref() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useRef", &[jrust_runtime::core::JsValue::Null]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.get("current").is_some());
    }
}

#[test]
fn test_react_use_memo() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useMemo", &[
        jrust_runtime::core::JsValue::new_function(|_| Ok(jrust_runtime::core::JsValue::new_number(42.0))),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_use_callback() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let callback = jrust_runtime::core::JsValue::new_function(|_| Ok(jrust_runtime::core::JsValue::Undefined));
    
    let result = registry.call("useCallback", &[
        callback,
        jrust_runtime::core::JsValue::Array(vec![]),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_fragment() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.Fragment", &[
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
        jrust_runtime::core::JsValue::String("Hello".to_string()),
        jrust_runtime::core::JsValue::String("World".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_create_ref() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.createRef", &[]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.get("current").is_some());
    }
}

#[test]
fn test_react_use_id() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useId", &[]);
    
    assert!(result.is_ok());
}

// ==================== Integration Tests ====================

#[test]
fn test_all_bindings_registered() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    assert!(registry.call("svelte_element", &[jrust_runtime::core::JsValue::String("div".to_string())]).is_ok());
    assert!(registry.call("preact_h", &[jrust_runtime::core::JsValue::String("div".to_string())]).is_ok());
    assert!(registry.call("solid_createSignal", &[]).is_ok());
    assert!(registry.call("React.createElement", &[jrust_runtime::core::JsValue::String("div".to_string())]).is_ok());
}

#[test]
fn test_react_nested_elements() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let button = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("button".to_string()),
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
        jrust_runtime::core::JsValue::String("Click me".to_string()),
    ]).unwrap();
    
    let div = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::Object(jrust_runtime::core::JsObject::new()),
        button,
    ]);
    
    assert!(div.is_ok());
}
