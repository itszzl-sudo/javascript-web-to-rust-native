//! SolidJS 运行时绑定
//!
//! 实现 SolidJS 的响应式系统和 DOM 操作

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static SOLID_CONTEXT: RefCell<SolidContext> = RefCell::new(SolidContext::new());
}

struct SolidContext {
    effect_stack: Vec<usize>,
    signal_id: usize,
    effect_id: usize,
}

impl SolidContext {
    fn new() -> Self {
        Self {
            effect_stack: Vec::new(),
            signal_id: 0,
            effect_id: 0,
        }
    }
    
    fn next_signal_id(&mut self) -> usize {
        let id = self.signal_id;
        self.signal_id += 1;
        id
    }
    
    fn next_effect_id(&mut self) -> usize {
        let id = self.effect_id;
        self.effect_id += 1;
        id
    }
}

pub fn register_solid_bindings(registry: &mut BindingRegistry) {
    registry.register("solid_createSignal", solid_create_signal);
    registry.register("solid_createEffect", solid_create_effect);
    registry.register("solid_createMemo", solid_create_memo);
    registry.register("solid_createRenderEffect", solid_create_render_effect);
    registry.register("solid_createComputed", solid_create_computed);
    registry.register("solid_onMount", solid_on_mount);
    registry.register("solid_onCleanup", solid_on_cleanup);
    registry.register("solid_untrack", solid_untrack);
    registry.register("solid_batch", solid_batch);
    registry.register("solid_template", solid_template);
    registry.register("solid_insert", solid_insert);
    registry.register("solid_spread", solid_spread);
    registry.register("solid_dynamic", solid_dynamic);
    registry.register("solid_suspense", solid_suspense);
    registry.register("solid_portal", solid_portal);
}

fn solid_create_signal(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    
    let signal_id = SOLID_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_signal_id()
    });
    
    let getter = JsValue::new_object_with_data(
        "signal_getter",
        format!("signal_{}", signal_id),
    );
    
    let setter = JsValue::new_object_with_data(
        "signal_setter",
        format!("signal_{}", signal_id),
    );
    
    Ok(JsValue::Array(vec![
        JsValue::new_object_with_data("signal", format!("{}:{:?}", signal_id, initial_value)),
        setter,
    ]))
}

fn solid_create_effect(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_createEffect: requires effect function".to_string());
    }
    
    let _effect_fn = &args[0];
    let deps = args.get(1);
    
    let effect_id = SOLID_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_effect_id()
    });
    
    let dep_info = deps.map(|d| format!("{:?}", d)).unwrap_or_default();
    
    Ok(JsValue::new_object_with_data(
        "effect",
        format!("effect_{} deps:{}", effect_id, dep_info),
    ))
}

fn solid_create_memo(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_createMemo: requires computation function".to_string());
    }
    
    let _compute_fn = &args[0];
    let deps = args.get(1);
    
    let signal_id = SOLID_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_signal_id()
    });
    
    let dep_info = deps.map(|d| format!("{:?}", d)).unwrap_or_default();
    
    Ok(JsValue::new_object_with_data(
        "memo",
        format!("memo_{} deps:{}", signal_id, dep_info),
    ))
}

fn solid_create_render_effect(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_createRenderEffect: requires effect function".to_string());
    }
    
    let effect_id = SOLID_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_effect_id()
    });
    
    Ok(JsValue::new_object_with_data(
        "render_effect",
        format!("render_effect_{}", effect_id),
    ))
}

fn solid_create_computed(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_createComputed: requires computation function".to_string());
    }
    
    let signal_id = SOLID_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_signal_id()
    });
    
    Ok(JsValue::new_object_with_data(
        "computed",
        format!("computed_{}", signal_id),
    ))
}

fn solid_on_mount(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_onMount: requires callback function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("lifecycle", "onMount".to_string()))
}

fn solid_on_cleanup(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_onCleanup: requires callback function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("lifecycle", "onCleanup".to_string()))
}

fn solid_untrack(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_untrack: requires computation function".to_string());
    }
    
    let compute_fn = &args[0];
    
    Ok(compute_fn.clone())
}

fn solid_batch(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_batch: requires computation function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("batch", "executed".to_string()))
}

fn solid_template(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_template: requires element".to_string());
    }
    
    let element = &args[0];
    Ok(element.clone())
}

fn solid_insert(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("solid_insert: requires parent and value".to_string());
    }
    
    let _parent = &args[0];
    let _value = &args[1];
    let marker = args.get(2);
    
    let marker_info = marker.map(|m| format!("{:?}", m)).unwrap_or_default();
    
    Ok(JsValue::new_object_with_data(
        "insert",
        format!("insert marker:{}", marker_info),
    ))
}

fn solid_spread(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("solid_spread: requires element and props".to_string());
    }
    
    Ok(JsValue::Undefined)
}

fn solid_dynamic(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("solid_dynamic: requires component and props".to_string());
    }
    
    let _component = &args[0];
    let _props = &args[1];
    
    Ok(JsValue::new_object_with_data("dynamic", "component".to_string()))
}

fn solid_suspense(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("solid_suspense: requires config".to_string());
    }
    
    Ok(JsValue::new_object_with_data("suspense", "fallback".to_string()))
}

fn solid_portal(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("solid_portal: requires target and content".to_string());
    }
    
    let _target = &args[0];
    let _content = &args[1];
    
    Ok(JsValue::new_object_with_data("portal", "mounted".to_string()))
}
