//! Preact 运行时绑定
//!
//! 实现 Preact 的 h() 函数和组件 API

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject};

pub fn register_preact_bindings(registry: &mut BindingRegistry) {
    registry.register("preact_h", preact_h);
    registry.register("preact_fragment", preact_fragment);
    registry.register("preact_createElement", preact_create_element);
    registry.register("preact_render", preact_render);
    registry.register("preact_toChildArray", preact_to_child_array);
    registry.register("preact_options", preact_options);
}

fn preact_h(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("preact_h: requires at least type argument".to_string());
    }
    
    let type_ = &args[0];
    let props = args.get(1).cloned().unwrap_or(JsValue::Object(JsObject::new()));
    let children = &args[2..];
    
    match type_ {
        JsValue::String(tag_name) => {
            create_element_from_tag(&tag_name, &props, children)
        }
        JsValue::Function(_) => {
            invoke_function_component(type_, &props, children)
        }
        _ => {
            Ok(JsValue::Null)
        }
    }
}

fn create_element_from_tag(tag: &str, props: &JsValue, children: &[JsValue]) -> Result<JsValue, String> {
    let mut element_data = format!("<{}", tag);
    
    if let JsValue::Object(props_obj) = props {
        let mut event_handlers = Vec::new();
        
        for (key, value) in props_obj.properties.iter() {
            match key.as_str() {
                "className" => {
                    element_data.push_str(&format!(" class=\"{}\"", value.to_string()));
                }
                "id" => {
                    element_data.push_str(&format!(" id=\"{}\"", value.to_string()));
                }
                "style" => {
                    element_data.push_str(&format!(" style=\"{}\"", value.to_string()));
                }
                "key" | "ref" => {
                }
                key if key.starts_with("on") => {
                    let event = &key[2..].to_lowercase();
                    event_handlers.push(format!("on{}=\"[handler]\"", event));
                }
                _ => {
                    element_data.push_str(&format!(" {}=\"{}\"", key, value.to_string()));
                }
            }
        }
        
        for handler in event_handlers {
            element_data.push_str(&format!(" {}", handler));
        }
    }
    
    element_data.push('>');
    
    for child in children {
        match child {
            JsValue::String(s) => element_data.push_str(s),
            JsValue::Number(n) => element_data.push_str(&n.to_string()),
            JsValue::Object(obj) => {
                if let Some(data) = obj.get("data") {
                    element_data.push_str(&data.to_string());
                }
            }
            _ => {}
        }
    }
    
    element_data.push_str(&format!("</{}>", tag));
    
    Ok(JsValue::new_object_with_data("element", element_data))
}

fn invoke_function_component(component: &JsValue, props: &JsValue, _children: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data(
        "component",
        format!("[Component with props: {:?}]", props),
    ))
}

fn preact_fragment(args: &[JsValue]) -> Result<JsValue, String> {
    let children = &args[1..];
    
    let mut fragment_data = String::new();
    for child in children {
        match child {
            JsValue::String(s) => fragment_data.push_str(s),
            JsValue::Number(n) => fragment_data.push_str(&n.to_string()),
            JsValue::Object(obj) => {
                if let Some(data) = obj.get("data") {
                    fragment_data.push_str(&data.to_string());
                }
            }
            _ => {}
        }
    }
    
    Ok(JsValue::new_object_with_data("fragment", fragment_data))
}

fn preact_create_element(args: &[JsValue]) -> Result<JsValue, String> {
    preact_h(args)
}

fn preact_render(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("preact_render: requires vnode and container".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn preact_to_child_array(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Array(vec![]));
    }
    
    let children = &args[0];
    match children {
        JsValue::Array(arr) => Ok(JsValue::Array(arr.clone())),
        JsValue::Null | JsValue::Undefined | JsValue::Boolean(false) => {
            Ok(JsValue::Array(vec![]))
        }
        value => Ok(JsValue::Array(vec![value.clone()])),
    }
}

fn preact_options(args: &[JsValue]) -> Result<JsValue, String> {
    let key = match args.get(0) {
        Some(JsValue::String(s)) => s.as_str(),
        _ => return Err("preact_options: requires key argument".to_string()),
    };
    
    match key {
        "vnode" | "diffed" | "commit" | "unmount" => {
            Ok(JsValue::new_object_with_data("hook", key.to_string()))
        }
        _ => Ok(JsValue::Undefined)
    }
}
