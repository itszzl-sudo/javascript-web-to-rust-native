//! Lit 运行时绑定
//!
//! 实现 Lit 的 html 模板、指令和 Web Components API

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject, JsArray};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_lit_bindings(registry: &mut BindingRegistry) {
    // 核心模板函数
    registry.register("html", lit_html);
    registry.register("svg", lit_svg);
    registry.register("render", lit_render);
    
    // 指令
    registry.register("unsafeHTML", lit_unsafe_html);
    registry.register("unsafeSVG", lit_unsafe_svg);
    registry.register("ifDefined", lit_if_defined);
    registry.register("guard", lit_guard);
    registry.register("cache", lit_cache);
    registry.register("repeat", lit_repeat);
    registry.register("map", lit_map);
    registry.register("join", lit_join);
    registry.register("range", lit_range);
    registry.register("classMap", lit_class_map);
    registry.register("styleMap", lit_style_map);
    registry.register("ref", lit_ref);
    registry.register("live", lit_live);
    registry.register("asyncAppend", lit_async_append);
    registry.register("asyncReplace", lit_async_replace);
    registry.register("until", lit_until);
    registry.register("when", lit_when);
    registry.register("choose", lit_choose);
    
    // 组件装饰器
    registry.register("@customElement", lit_custom_element);
    registry.register("@property", lit_property);
    registry.register("@state", lit_state);
    registry.register("@query", lit_query);
    registry.register("@queryAll", lit_query_all);
    registry.register("@queryAsync", lit_query_async);
    registry.register("@eventOptions", lit_event_options);
    
    // 生命周期
    registry.register("connectedCallback", lit_connected_callback);
    registry.register("disconnectedCallback", lit_disconnected_callback);
    registry.register("adoptedCallback", lit_adopted_callback);
    registry.register("attributeChangedCallback", lit_attribute_changed_callback);
    registry.register("firstUpdated", lit_first_updated);
    registry.register("updated", lit_updated);
    registry.register("shouldUpdate", lit_should_update);
    registry.register("willUpdate", lit_will_update);
    
    // 响应式系统
    registry.register("requestUpdate", lit_request_update);
    registry.register("get updateComplete", lit_update_complete);
    
    // LitElement 基类方法
    registry.register("createRenderRoot", lit_create_render_root);
    registry.register("render", lit_render_method);
    registry.register("adoptStyles", lit_adopt_styles);
    
    // 静态方法
    registry.register("LitElement.styles", lit_element_styles);
    registry.register("LitElement.properties", lit_element_properties);
    
    // CSS
    registry.register("css", lit_css);
    registry.register("unsafeCSS", lit_unsafe_css);
}

// ==================== 核心模板函数 ====================

fn lit_html(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let template_strings = &args[0];
    let values = &args[1..];
    
    let mut result = String::new();
    
    if let JsValue::Array(strings) = template_strings {
        let strings = strings.borrow();
        for (i, string) in strings.iter().enumerate() {
            result.push_str(&string.to_string());
            if i < values.len() {
                result.push_str(&render_value(&values[i]));
            }
        }
    } else {
        result = template_strings.to_string();
    }
    
    let mut obj = JsObject::new();
    obj.set("_tag", JsValue::String("html".to_string()));
    obj.set("_template", JsValue::String(result.clone()));
    obj.set("_values", JsValue::Array(Rc::new(RefCell::new(JsArray::from(values.to_vec())))));
    
    Ok(JsValue::from(obj))
}

fn lit_svg(args: &[JsValue]) -> Result<JsValue, String> {
    let result = lit_html(args)?;
    
    if let JsValue::Object(obj) = &result {
        obj.borrow_mut().set("_tag", JsValue::String("svg".to_string()));
        return Ok(result);
    }
    
    Ok(result)
}

fn lit_render(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("render: requires template and container".to_string());
    }
    
    let _template = &args[0];
    let _container = &args[1];
    let options = args.get(2);
    
    let mut obj = JsObject::new();
    obj.set("rendered", JsValue::Boolean(true));
    if let Some(opts) = options {
        obj.set("options", opts.clone());
    }
    
    Ok(JsValue::from(obj))
}

fn render_value(value: &JsValue) -> String {
    match value {
        JsValue::String(s) => escape_html(s),
        JsValue::Number(n) => n.to_string(),
        JsValue::Boolean(b) => b.to_string(),
        JsValue::Array(arr) => {
            let arr = arr.borrow();
            arr.iter().map(|v| render_value(v)).collect::<Vec<_>>().join("")
        }
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            if let Some(template) = obj.get("_template") {
                template.to_string()
            } else {
                "[object Object]".to_string()
            }
        }
        JsValue::Null | JsValue::Undefined => "".to_string(),
        _ => value.to_string(),
    }
}

// ==================== 指令 ====================

fn lit_unsafe_html(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("unsafeHTML".to_string()));
    obj.set("_value", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_unsafe_svg(args: &[JsValue]) -> Result<JsValue, String> {
    let result = lit_unsafe_html(args)?;
    
    if let JsValue::Object(obj) = &result {
        obj.borrow_mut().set("_directive", JsValue::String("unsafeSVG".to_string()));
        return Ok(result);
    }
    
    Ok(result)
}

fn lit_if_defined(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    
    match &args[0] {
        JsValue::Null | JsValue::Undefined => Ok(JsValue::Undefined),
        value => Ok(value.clone()),
    }
}

fn lit_guard(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("guard: requires dependency and factory".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("guard".to_string()));
    obj.set("_dependency", args[0].clone());
    obj.set("_factory", args[1].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_cache(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("cache: requires value".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("cache".to_string()));
    obj.set("_value", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_repeat(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 3 {
        return Err("repeat: requires items, keyFn, renderFn".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("repeat".to_string()));
    obj.set("_items", args[0].clone());
    obj.set("_keyFn", args[1].clone());
    obj.set("_renderFn", args[2].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_map(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("map: requires iterable and callback".to_string());
    }
    
    let iterable = &args[0];
    let callback = &args[1];
    
    if let JsValue::Array(items) = iterable {
        let items = items.borrow();
        let mapped: Vec<JsValue> = items.iter()
            .map(|item| {
                if let JsValue::Function(f) = callback {
                    f.borrow().call(&[item.clone()]).unwrap_or(JsValue::Undefined)
                } else {
                    JsValue::Undefined
                }
            })
            .collect();
        return Ok(JsValue::Array(Rc::new(RefCell::new(JsArray::from(mapped)))));
    }
    
    Ok(JsValue::new_array())
}

fn lit_join(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("join: requires iterable and joiner".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directive", "join".to_string()))
}

fn lit_range(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::new_array());
    }
    
    let start = match &args[0] {
        JsValue::Number(n) => *n as i32,
        _ => 0,
    };
    
    let end = match args.get(1) {
        Some(JsValue::Number(n)) => *n as i32,
        _ => start,
    };
    
    let step = match args.get(2) {
        Some(JsValue::Number(n)) => *n as i32,
        _ => 1,
    };
    
    let mut result = Vec::new();
    let mut i = start;
    while (step > 0 && i < end) || (step < 0 && i > end) {
        result.push(JsValue::new_number(i as f64));
        i += step;
    }
    
    Ok(JsValue::Array(Rc::new(RefCell::new(JsArray::from(result)))))
}

fn lit_class_map(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let mut classes = Vec::new();
    
    if let JsValue::Object(map) = &args[0] {
        let map = map.borrow();
        for (class, value) in map.entries() {
            if value.to_string() == "true" {
                classes.push(class);
            }
        }
    }
    
    Ok(JsValue::String(classes.join(" ")))
}

fn lit_style_map(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let mut styles = Vec::new();
    
    if let JsValue::Object(map) = &args[0] {
        let map = map.borrow();
        for (prop, value) in map.entries() {
            styles.push(format!("{}: {}", prop, value.to_string()));
        }
    }
    
    Ok(JsValue::String(styles.join("; ")))
}

fn lit_ref(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ref: requires ref object".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("ref".to_string()));
    obj.set("_ref", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_live(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("live: requires value".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("_directive", JsValue::String("live".to_string()));
    obj.set("_value", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn lit_async_append(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("asyncAppend: requires async iterable".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directive", "asyncAppend".to_string()))
}

fn lit_async_replace(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("asyncReplace: requires async iterable".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directive", "asyncReplace".to_string()))
}

fn lit_until(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("until: requires at least one argument".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directive", format!("until({} args)", args.len())))
}

fn lit_when(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("when: requires condition and trueCase".to_string());
    }
    
    let condition = &args[0];
    let true_case = &args[1];
    let false_case = args.get(2);
    
    match condition {
        JsValue::Boolean(true) => Ok(true_case.clone()),
        JsValue::Boolean(false) => Ok(false_case.cloned().unwrap_or(JsValue::Undefined)),
        _ => Ok(true_case.clone()),
    }
}

fn lit_choose(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("choose: requires at least one case".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directive", "choose".to_string()))
}

// ==================== 组件装饰器 ====================

fn lit_custom_element(args: &[JsValue]) -> Result<JsValue, String> {
    let tag_name = match args.get(0) {
        Some(JsValue::String(s)) => s.clone(),
        _ => return Err("@customElement: requires tag name".to_string()),
    };
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("customElement".to_string()));
    obj.set("tagName", JsValue::String(tag_name));
    
    Ok(JsValue::from(obj))
}

fn lit_property(args: &[JsValue]) -> Result<JsValue, String> {
    let options = args.get(0).cloned().unwrap_or(JsValue::new_object());
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("property".to_string()));
    obj.set("options", options);
    
    Ok(JsValue::from(obj))
}

fn lit_state(args: &[JsValue]) -> Result<JsValue, String> {
    let options = args.get(0).cloned().unwrap_or(JsValue::new_object());
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("state".to_string()));
    obj.set("options", options);
    
    Ok(JsValue::from(obj))
}

fn lit_query(args: &[JsValue]) -> Result<JsValue, String> {
    let selector = match args.get(0) {
        Some(JsValue::String(s)) => s.clone(),
        _ => return Err("@query: requires selector".to_string()),
    };
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("query".to_string()));
    obj.set("selector", JsValue::String(selector));
    
    Ok(JsValue::from(obj))
}

fn lit_query_all(args: &[JsValue]) -> Result<JsValue, String> {
    let selector = match args.get(0) {
        Some(JsValue::String(s)) => s.clone(),
        _ => return Err("@queryAll: requires selector".to_string()),
    };
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("queryAll".to_string()));
    obj.set("selector", JsValue::String(selector));
    
    Ok(JsValue::from(obj))
}

fn lit_query_async(args: &[JsValue]) -> Result<JsValue, String> {
    let selector = match args.get(0) {
        Some(JsValue::String(s)) => s.clone(),
        _ => return Err("@queryAsync: requires selector".to_string()),
    };
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("queryAsync".to_string()));
    obj.set("selector", JsValue::String(selector));
    
    Ok(JsValue::from(obj))
}

fn lit_event_options(args: &[JsValue]) -> Result<JsValue, String> {
    let options = args.get(0).cloned().unwrap_or(JsValue::new_object());
    
    let mut obj = JsObject::new();
    obj.set("decorator", JsValue::String("eventOptions".to_string()));
    obj.set("options", options);
    
    Ok(JsValue::from(obj))
}

// ==================== 生命周期 ====================

fn lit_connected_callback(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "connected".to_string()))
}

fn lit_disconnected_callback(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "disconnected".to_string()))
}

fn lit_adopted_callback(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "adopted".to_string()))
}

fn lit_attribute_changed_callback(args: &[JsValue]) -> Result<JsValue, String> {
    let name = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    let old_val = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    let new_val = args.get(2).cloned().unwrap_or(JsValue::Undefined);
    
    let mut obj = JsObject::new();
    obj.set("lifecycle", JsValue::String("attributeChanged".to_string()));
    obj.set("name", name);
    obj.set("oldValue", old_val);
    obj.set("newValue", new_val);
    
    Ok(JsValue::from(obj))
}

fn lit_first_updated(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "firstUpdated".to_string()))
}

fn lit_updated(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "updated".to_string()))
}

fn lit_should_update(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Boolean(true));
    }
    Ok(args[0].clone())
}

fn lit_will_update(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("lifecycle", "willUpdate".to_string()))
}

// ==================== 响应式系统 ====================

fn lit_request_update(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Boolean(true))
}

fn lit_update_complete(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("promise", "updateComplete".to_string()))
}

// ==================== LitElement 方法 ====================

fn lit_create_render_root(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("element", "shadowRoot".to_string()))
}

fn lit_render_method(args: &[JsValue]) -> Result<JsValue, String> {
    lit_html(args)
}

fn lit_adopt_styles(args: &[JsValue]) -> Result<JsValue, String> {
    let styles = args.get(0).cloned().unwrap_or_else(|| JsValue::new_array());
    
    let mut obj = JsObject::new();
    obj.set("styles", styles);
    
    Ok(JsValue::from(obj))
}

fn lit_element_styles(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(Rc::new(RefCell::new(JsArray::from(args.to_vec())))))
}

fn lit_element_properties(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(Rc::new(RefCell::new(JsArray::from(args.to_vec())))))
}

// ==================== CSS ====================

fn lit_css(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let css_strings = &args[0];
    let values = &args[1..];
    
    let mut result = String::new();
    
    if let JsValue::Array(strings) = css_strings {
        let strings = strings.borrow();
        for (i, string) in strings.iter().enumerate() {
            result.push_str(&string.to_string());
            if i < values.len() {
                result.push_str(&values[i].to_string());
            }
        }
    }
    
    let mut obj = JsObject::new();
    obj.set("_tag", JsValue::String("css".to_string()));
    obj.set("_css", JsValue::String(result));
    
    Ok(JsValue::from(obj))
}

fn lit_unsafe_css(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    
    let mut obj = JsObject::new();
    obj.set("_tag", JsValue::String("css".to_string()));
    obj.set("_unsafe", JsValue::Boolean(true));
    obj.set("_css", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
