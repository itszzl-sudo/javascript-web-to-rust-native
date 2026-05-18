//! Svelte 运行时绑定
//!
//! 实现 Svelte 编译器生成的 DOM 操作函数

use crate::bindings::BindingRegistry;
use crate::core::JsValue;
use crate::dom::{Document, Element};

pub fn register_svelte_bindings(registry: &mut BindingRegistry) {
    registry.register("svelte_element", svelte_element);
    registry.register("svelte_text", svelte_text);
    registry.register("svelte_space", svelte_space);
    registry.register("svelte_empty", svelte_empty);
    registry.register("svelte_insert", svelte_insert);
    registry.register("svelte_append", svelte_append);
    registry.register("svelte_detach", svelte_detach);
    registry.register("svelte_attr", svelte_attr);
    registry.register("svelte_set_data", svelte_set_data);
    registry.register("svelte_set_style", svelte_set_style);
    registry.register("svelte_add_class", svelte_add_class);
    registry.register("svelte_remove_class", svelte_remove_class);
    registry.register("svelte_listen", svelte_listen);
    registry.register("svelte_binding", svelte_binding);
}

fn svelte_element(args: &[JsValue]) -> Result<JsValue, String> {
    let tag_name = match &args.get(0) {
        Some(JsValue::String(s)) => s.as_str(),
        _ => return Err("svelte_element: expected tag name string".to_string()),
    };
    
    let mut element = Element::new(tag_name);
    element.set_attribute("data-svelte", "true");
    
    Ok(JsValue::new_object_with_data("element", element.tag_name.clone()))
}

fn svelte_text(args: &[JsValue]) -> Result<JsValue, String> {
    let content = match &args.get(0) {
        Some(JsValue::String(s)) => s.clone(),
        Some(JsValue::Number(n)) => n.to_string(),
        _ => "".to_string(),
    };
    
    Ok(JsValue::new_object_with_data("text", content))
}

fn svelte_space(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("text", " ".to_string()))
}

fn svelte_empty(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("comment", "".to_string()))
}

fn svelte_insert(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("svelte_insert: requires at least 2 arguments".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn svelte_append(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("svelte_append: requires at least 2 arguments".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn svelte_detach(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("svelte_detach: requires node argument".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn svelte_attr(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 3 {
        return Err("svelte_attr: requires element, name, value".to_string());
    }
    
    let name = match &args[1] {
        JsValue::String(s) => s.as_str(),
        _ => return Err("svelte_attr: name must be string".to_string()),
    };
    
    let value = match &args[2] {
        JsValue::String(s) => s.clone(),
        JsValue::Number(n) => n.to_string(),
        JsValue::Boolean(b) => b.to_string(),
        _ => "".to_string(),
    };
    
    Ok(JsValue::new_object_with_data(
        "attr",
        format!("{}=\"{}\"", name, value),
    ))
}

fn svelte_set_data(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("svelte_set_data: requires text_node and value".to_string());
    }
    
    let value = match &args[1] {
        JsValue::String(s) => s.clone(),
        JsValue::Number(n) => n.to_string(),
        _ => "".to_string(),
    };
    
    Ok(JsValue::new_object_with_data("text", value))
}

fn svelte_set_style(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 3 {
        return Err("svelte_set_style: requires element, property, value".to_string());
    }
    
    let property = match &args[1] {
        JsValue::String(s) => s.as_str(),
        _ => return Err("svelte_set_style: property must be string".to_string()),
    };
    
    let value = match &args[2] {
        JsValue::String(s) => s.clone(),
        JsValue::Number(n) => n.to_string(),
        _ => "".to_string(),
    };
    
    Ok(JsValue::new_object_with_data(
        "style",
        format!("{}: {}", property, value),
    ))
}

fn svelte_add_class(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("svelte_add_class: requires element and class".to_string());
    }
    
    let class = match &args[1] {
        JsValue::String(s) => s.clone(),
        _ => return Err("svelte_add_class: class must be string".to_string()),
    };
    
    Ok(JsValue::new_object_with_data("class", class))
}

fn svelte_remove_class(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("svelte_remove_class: requires element and class".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn svelte_listen(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 3 {
        return Err("svelte_listen: requires node, event, handler".to_string());
    }
    
    let event_type = match &args[1] {
        JsValue::String(s) => s.clone(),
        _ => return Err("svelte_listen: event must be string".to_string()),
    };
    
    Ok(JsValue::new_object_with_data(
        "listener",
        format!("on{}", event_type),
    ))
}

fn svelte_binding(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 3 {
        return Err("svelte_binding: requires node, value, update".to_string());
    }
    
    Ok(JsValue::Undefined)
}
