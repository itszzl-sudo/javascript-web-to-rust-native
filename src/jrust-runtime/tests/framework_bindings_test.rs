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
        jrust_runtime::core::JsValue::new_object(),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_preact_h_with_children() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("preact_h", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::new_object(),
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
    
    let mut props = jrust_runtime::core::JsObject::new();
    props.set("className", jrust_runtime::core::JsValue::String("container".to_string()));
    props.set("id", jrust_runtime::core::JsValue::String("app".to_string()));
    
    let result = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::from(props),
    ]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.borrow().get("data").is_some());
    }
}

#[test]
fn test_react_create_element_with_children() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::new_object(),
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
        assert_eq!(arr.borrow().len(), 2);
    }
}

#[test]
fn test_react_use_ref() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useRef", &[jrust_runtime::core::JsValue::Null]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.borrow().get("current").is_some());
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
        jrust_runtime::core::JsValue::new_array(),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_react_fragment() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("React.Fragment", &[
        jrust_runtime::core::JsValue::new_object(),
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
        assert!(obj.borrow().get("current").is_some());
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
        jrust_runtime::core::JsValue::new_object(),
        jrust_runtime::core::JsValue::String("Click me".to_string()),
    ]).unwrap();
    
    let div = registry.call("React.createElement", &[
        jrust_runtime::core::JsValue::String("div".to_string()),
        jrust_runtime::core::JsValue::new_object(),
        button,
    ]);
    
    assert!(div.is_ok());
}

// ==================== Angular Tests ====================

#[test]
fn test_angular_element_start() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵelementStart", &[
        jrust_runtime::core::JsValue::new_number(0.0),
        jrust_runtime::core::JsValue::String("div".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_text() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵtext", &[
        jrust_runtime::core::JsValue::new_number(0.0),
        jrust_runtime::core::JsValue::String("Hello Angular".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_property() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵproperty", &[
        jrust_runtime::core::JsValue::String("value".to_string()),
        jrust_runtime::core::JsValue::String("test".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_attribute() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵattribute", &[
        jrust_runtime::core::JsValue::String("class".to_string()),
        jrust_runtime::core::JsValue::String("container".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_listener() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let handler = jrust_runtime::core::JsValue::new_function(|_| Ok(jrust_runtime::core::JsValue::Undefined));
    
    let result = registry.call("ɵɵlistener", &[
        jrust_runtime::core::JsValue::String("click".to_string()),
        handler,
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_style_prop() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵstyleProp", &[
        jrust_runtime::core::JsValue::String("color".to_string()),
        jrust_runtime::core::JsValue::String("red".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_class_prop() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵclassProp", &[
        jrust_runtime::core::JsValue::String("active".to_string()),
        jrust_runtime::core::JsValue::Boolean(true),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_define_component() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut config = jrust_runtime::core::JsObject::new();
    config.set("selector", jrust_runtime::core::JsValue::String("app-root".to_string()));
    
    let result = registry.call("ɵɵdefineComponent", &[
        jrust_runtime::core::JsValue::from(config),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_define_directive() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut config = jrust_runtime::core::JsObject::new();
    config.set("selector", jrust_runtime::core::JsValue::String("[appHighlight]".to_string()));
    
    let result = registry.call("ɵɵdefineDirective", &[
        jrust_runtime::core::JsValue::from(config),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_define_pipe() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut config = jrust_runtime::core::JsObject::new();
    config.set("name", jrust_runtime::core::JsValue::String("uppercase".to_string()));
    
    let result = registry.call("ɵɵdefinePipe", &[
        jrust_runtime::core::JsValue::from(config),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_inject() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵinject", &[
        jrust_runtime::core::JsValue::String("MyService".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_text_interpolate() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵtextInterpolate", &[
        jrust_runtime::core::JsValue::String("Hello".to_string()),
    ]);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), jrust_runtime::core::JsValue::String("Hello".to_string()));
}

#[test]
fn test_angular_advance() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵadvance", &[
        jrust_runtime::core::JsValue::new_number(2.0),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_template() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵtemplate", &[
        jrust_runtime::core::JsValue::new_number(0.0),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_angular_projection() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("ɵɵprojection", &[
        jrust_runtime::core::JsValue::new_number(0.0),
    ]);
    
    assert!(result.is_ok());
}

// ==================== Lit Tests ====================

#[test]
fn test_lit_html() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let arr = jrust_runtime::core::JsArray::from(vec![
        jrust_runtime::core::JsValue::String("<div>".to_string()),
        jrust_runtime::core::JsValue::String("</div>".to_string()),
    ]);
    let result = registry.call("html", &[
        jrust_runtime::core::JsValue::Array(std::rc::Rc::new(std::cell::RefCell::new(arr))),
        jrust_runtime::core::JsValue::String("Hello".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_render() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut template = jrust_runtime::core::JsObject::new();
    template.set("_template", jrust_runtime::core::JsValue::String("<div>Hello</div>".to_string()));
    
    let result = registry.call("render", &[
        jrust_runtime::core::JsValue::from(template),
        jrust_runtime::core::JsValue::new_object(),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_class_map() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut class_map = jrust_runtime::core::JsObject::new();
    class_map.set("active", jrust_runtime::core::JsValue::Boolean(true));
    class_map.set("disabled", jrust_runtime::core::JsValue::Boolean(false));
    
    let result = registry.call("classMap", &[
        jrust_runtime::core::JsValue::from(class_map),
    ]);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), jrust_runtime::core::JsValue::String("active".to_string()));
}

#[test]
fn test_lit_style_map() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut style_map = jrust_runtime::core::JsObject::new();
    style_map.set("color", jrust_runtime::core::JsValue::String("red".to_string()));
    style_map.set("fontSize", jrust_runtime::core::JsValue::String("16px".to_string()));
    
    let result = registry.call("styleMap", &[
        jrust_runtime::core::JsValue::from(style_map),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_custom_element() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("@customElement", &[
        jrust_runtime::core::JsValue::String("my-component".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_css() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let arr = jrust_runtime::core::JsArray::from(vec![
        jrust_runtime::core::JsValue::String(":host { display: block; }".to_string()),
    ]);
    let result = registry.call("css", &[
        jrust_runtime::core::JsValue::Array(std::rc::Rc::new(std::cell::RefCell::new(arr))),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_repeat_directive() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let items = jrust_runtime::core::JsValue::new_array();
    
    let key_fn = jrust_runtime::core::JsValue::new_function(|args| {
        Ok(args.get(0).cloned().unwrap_or(jrust_runtime::core::JsValue::Undefined))
    });
    
    let render_fn = jrust_runtime::core::JsValue::new_function(|args| {
        Ok(args.get(0).cloned().unwrap_or(jrust_runtime::core::JsValue::Undefined))
    });
    
    let result = registry.call("repeat", &[items, key_fn, render_fn]);
    
    assert!(result.is_ok());
}

#[test]
fn test_lit_when_directive() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("when", &[
        jrust_runtime::core::JsValue::Boolean(true),
        jrust_runtime::core::JsValue::String("Yes".to_string()),
        jrust_runtime::core::JsValue::String("No".to_string()),
    ]);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), jrust_runtime::core::JsValue::String("Yes".to_string()));
}

// ==================== Qwik Tests ====================

#[test]
fn test_qwik_use_signal() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useSignal", &[
        jrust_runtime::core::JsValue::new_number(0.0),
    ]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.borrow().get("id").is_some());
        assert!(obj.borrow().get("value").is_some());
    }
}

#[test]
fn test_qwik_use_store() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut initial_state = jrust_runtime::core::JsObject::new();
    initial_state.set("count", jrust_runtime::core::JsValue::new_number(0.0));
    
    let result = registry.call("useStore", &[
        jrust_runtime::core::JsValue::from(initial_state),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_use_context() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useContext", &[
        jrust_runtime::core::JsValue::String("MyContext".to_string()),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_use_computed() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let compute_fn = jrust_runtime::core::JsValue::new_function(|_| {
        Ok(jrust_runtime::core::JsValue::new_number(42.0))
    });
    
    let result = registry.call("useComputed$", &[compute_fn]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_use_resource() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let resource_fn = jrust_runtime::core::JsValue::new_function(|_| {
        Ok(jrust_runtime::core::JsValue::String("data".to_string()))
    });
    
    let result = registry.call("useResource$", &[resource_fn]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_dollar() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let handler = jrust_runtime::core::JsValue::new_function(|_| {
        Ok(jrust_runtime::core::JsValue::Undefined)
    });
    
    let result = registry.call("$", &[handler]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert_eq!(obj.borrow().get("type"), Some(jrust_runtime::core::JsValue::String("qrl".to_string())));
    }
}

#[test]
fn test_qwik_component() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let render_fn = jrust_runtime::core::JsValue::new_function(|_| {
        Ok(jrust_runtime::core::JsValue::String("<div>Hello Qwik</div>".to_string()))
    });
    
    let result = registry.call("component$", &[render_fn]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_use_location() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useLocation", &[]);
    
    assert!(result.is_ok());
    
    if let jrust_runtime::core::JsValue::Object(obj) = result.unwrap() {
        assert!(obj.borrow().get("pathname").is_some());
    }
}

#[test]
fn test_qwik_use_form() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let mut initial = jrust_runtime::core::JsObject::new();
    initial.set("username", jrust_runtime::core::JsValue::String("".to_string()));
    
    let result = registry.call("useForm", &[
        jrust_runtime::core::JsValue::from(initial),
    ]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_use_id() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let result = registry.call("useId", &[]);
    
    assert!(result.is_ok());
}

#[test]
fn test_qwik_server_function() {
    let mut registry = BindingRegistry::new();
    register_all_framework_bindings(&mut registry);
    
    let server_fn = jrust_runtime::core::JsValue::new_function(|_| {
        Ok(jrust_runtime::core::JsValue::String("server result".to_string()))
    });
    
    let result = registry.call("server$", &[server_fn]);
    
    assert!(result.is_ok());
}
