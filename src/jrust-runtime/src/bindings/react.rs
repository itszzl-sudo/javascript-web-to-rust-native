//! React 运行时绑定
//!
//! 实现 React 的 createElement、Hooks 和组件系统

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject, JsFunction, JsArray};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

thread_local! {
    static REACT_CONTEXT: RefCell<ReactContext> = RefCell::new(ReactContext::new());
}

#[derive(Debug)]
struct ReactContext {
    component_stack: Vec<ComponentId>,
    hook_index: usize,
    component_registry: HashMap<ComponentId, ComponentState>,
    next_component_id: usize,
}

type ComponentId = usize;

#[derive(Debug)]
struct ComponentState {
    hooks: Vec<HookValue>,
    effect_cleanups: Vec<Option<JsFunction>>,
    prev_deps: Vec<Option<Vec<JsValue>>>,
}

#[derive(Debug, Clone)]
enum HookValue {
    State(JsValue),
    Ref(JsObject),
    Memo(JsValue),
}

impl ReactContext {
    fn new() -> Self {
        Self {
            component_stack: Vec::new(),
            hook_index: 0,
            component_registry: HashMap::new(),
            next_component_id: 0,
        }
    }
    
    fn current_component(&self) -> Option<ComponentId> {
        self.component_stack.last().copied()
    }
    
    fn next_hook_index(&mut self) -> usize {
        let idx = self.hook_index;
        self.hook_index += 1;
        idx
    }
    
    fn next_component_id(&mut self) -> ComponentId {
        let id = self.next_component_id;
        self.next_component_id += 1;
        id
    }
    
    fn push_component(&mut self, id: ComponentId) {
        self.component_stack.push(id);
        self.hook_index = 0;
    }
    
    fn pop_component(&mut self) {
        self.component_stack.pop();
    }
    
    fn get_or_create_component(&mut self, id: ComponentId) -> &mut ComponentState {
        self.component_registry.entry(id).or_insert_with(|| {
            ComponentState {
                hooks: Vec::new(),
                effect_cleanups: Vec::new(),
                prev_deps: Vec::new(),
            }
        })
    }
}

pub fn register_react_bindings(registry: &mut BindingRegistry) {
    registry.register("React.createElement", react_create_element);
    registry.register("React.Fragment", react_fragment);
    registry.register("React.cloneElement", react_clone_element);
    
    registry.register("useState", react_use_state);
    registry.register("useEffect", react_use_effect);
    registry.register("useLayoutEffect", react_use_layout_effect);
    registry.register("useRef", react_use_ref);
    registry.register("useMemo", react_use_memo);
    registry.register("useCallback", react_use_callback);
    registry.register("useContext", react_use_context);
    registry.register("useReducer", react_use_reducer);
    
    registry.register("useImperativeHandle", react_use_imperative_handle);
    registry.register("useDebugValue", react_use_debug_value);
    registry.register("useId", react_use_id);
    registry.register("useTransition", react_use_transition);
    registry.register("useDeferredValue", react_use_deferred_value);
    
    registry.register("React.createRef", react_create_ref);
    registry.register("React.forwardRef", react_forward_ref);
    registry.register("React.memo", react_memo);
    registry.register("React.lazy", react_lazy);
    registry.register("React.Suspense", react_suspense);
    
    registry.register("react_render", react_render);
    registry.register("react_unmount", react_unmount);
}

fn react_create_element(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("React.createElement: requires at least type argument".to_string());
    }
    
    let type_ = &args[0];
    let props = args.get(1).cloned().unwrap_or(JsValue::new_object());
    let children = if args.len() > 2 { &args[2..] } else { &[] };
    
    match type_ {
        JsValue::String(tag_name) => {
            create_element_from_tag(&tag_name, &props, children)
        }
        JsValue::Function(_) => {
            invoke_function_component(type_, &props, children)
        }
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            if obj.get("$$typeof").map(|v| v.to_string()) == Some("Symbol(react.fragment)".to_string()) {
                create_fragment(children)
            } else {
                Ok(JsValue::Null)
            }
        }
        _ => {
            Ok(JsValue::Null)
        }
    }
}

fn create_element_from_tag(tag: &str, props: &JsValue, children: &[JsValue]) -> Result<JsValue, String> {
    let mut element_data = format!("<{}", tag);
    let mut event_handlers = Vec::new();
    let mut style_data = String::new();
    
    if let JsValue::Object(props_obj) = props {
        let props_obj = props_obj.borrow();
        for (key, value) in props_obj.entries() {
            match key.as_str() {
                "className" => {
                    element_data.push_str(&format!(" class=\"{}\"", escape_html(&value.to_string())));
                }
                "id" => {
                    element_data.push_str(&format!(" id=\"{}\"", escape_html(&value.to_string())));
                }
                "key" | "ref" => {
                }
                "style" => {
                    if let JsValue::Object(style_obj) = &value {
                        let style_obj = style_obj.borrow();
                        for (prop, val) in style_obj.entries() {
                            if !style_data.is_empty() {
                                style_data.push_str("; ");
                            }
                            style_data.push_str(&format!("{}: {}", prop, val.to_string()));
                        }
                    } else {
                        style_data = value.to_string();
                    }
                }
                "children" => {
                }
                key if key.starts_with("on") && key.len() > 2 => {
                    let event = key[2..].to_lowercase();
                    event_handlers.push((event, value.clone()));
                }
                _ => {
                    element_data.push_str(&format!(" {}=\"{}\"", key, escape_html(&value.to_string())));
                }
            }
        }
    }
    
    if !style_data.is_empty() {
        element_data.push_str(&format!(" style=\"{}\"", escape_html(&style_data)));
    }
    
    element_data.push('>');
    
    let has_children = !children.is_empty();
    for child in children {
        append_child_content(&mut element_data, child);
    }
    
    if !is_void_element(tag) {
        element_data.push_str(&format!("</{}>", tag));
    }
    
    let mut result_obj = JsObject::new();
    result_obj.set("type", JsValue::String(tag.to_string()));
    result_obj.set("props", props.clone());
    result_obj.set("data", JsValue::String(element_data));
    
    let events: Vec<JsValue> = event_handlers.into_iter()
        .map(|(e, h)| {
            let mut obj = JsObject::new();
            obj.set("event", JsValue::String(e));
            obj.set("handler", h);
            JsValue::from(obj)
        })
        .collect();
    result_obj.set("events", JsValue::Array(Rc::new(RefCell::new(JsArray::from(events)))));
    
    Ok(JsValue::from(result_obj))
}

fn append_child_content(element_data: &mut String, child: &JsValue) {
    match child {
        JsValue::String(s) => element_data.push_str(&escape_html(s)),
        JsValue::Number(n) => element_data.push_str(&n.to_string()),
        JsValue::Boolean(b) => if *b { element_data.push_str("true"); },
        JsValue::Object(obj) => {
            if let Some(data) = obj.borrow().get("data") {
                element_data.push_str(&data.to_string());
            }
        }
        JsValue::Array(arr) => {
            let arr = arr.borrow();
            for item in arr.iter() {
                append_child_content(element_data, item);
            }
        }
        JsValue::Null | JsValue::Undefined | JsValue::Boolean(false) => {}
        _ => element_data.push_str(&child.to_string()),
    }
}

fn invoke_function_component(component_fn: &JsValue, props: &JsValue, children: &[JsValue]) -> Result<JsValue, String> {
    let component_id = REACT_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_component_id()
    });
    
    REACT_CONTEXT.with(|ctx| {
        ctx.borrow_mut().push_component(component_id);
    });
    
    let mut props_with_children = match props {
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            let mut cloned = JsObject::new();
            for (k, v) in obj.entries() {
                cloned.set(k, v);
            }
            cloned
        }
        _ => JsObject::new(),
    };
    
    if !children.is_empty() {
        props_with_children.set("children", JsValue::Array(Rc::new(RefCell::new(JsArray::from(children.to_vec())))));
    }
    
    let result = match component_fn {
        JsValue::Function(f) => f.borrow().call(&[JsValue::from(props_with_children)]),
        _ => Ok(JsValue::Null),
    };
    
    REACT_CONTEXT.with(|ctx| {
        ctx.borrow_mut().pop_component();
    });
    
    result
}

fn create_fragment(children: &[JsValue]) -> Result<JsValue, String> {
    let mut fragment_data = String::new();
    for child in children {
        append_child_content(&mut fragment_data, child);
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("fragment".to_string()));
    obj.set("data", JsValue::String(fragment_data));
    
    Ok(JsValue::from(obj))
}

fn react_fragment(args: &[JsValue]) -> Result<JsValue, String> {
    let children = if args.len() > 1 { &args[1..] } else { &args[0..0] };
    create_fragment(children)
}

fn react_clone_element(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("React.cloneElement: requires element argument".to_string());
    }
    
    let element = &args[0];
    let new_props = args.get(1);
    let new_children = &args[2..];
    
    let merged_props = match (element, new_props) {
        (JsValue::Object(el), Some(JsValue::Object(np))) => {
            let el = el.borrow();
            let np = np.borrow();
            let mut merged = JsObject::new();
            for (k, v) in el.entries() {
                merged.set(k, v);
            }
            for (k, v) in np.entries() {
                merged.set(k, v);
            }
            JsValue::from(merged)
        }
        (_, Some(p)) => p.clone(),
        (JsValue::Object(_), None) => element.clone(),
        _ => JsValue::new_object(),
    };
    
    react_create_element(&[
        match element {
            JsValue::Object(el) => el.borrow().get("type").unwrap_or(JsValue::String("div".to_string())),
            _ => JsValue::String("div".to_string()),
        },
        merged_props,
    ])
}

fn react_use_state(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    
    let (state, setter) = REACT_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        let component_id = match ctx.current_component() {
            Some(id) => id,
            None => return (initial_value.clone(), create_noop_setter()),
        };
        
        let hook_index = ctx.next_hook_index();
        let component = ctx.get_or_create_component(component_id);
        
        if hook_index >= component.hooks.len() {
            component.hooks.push(HookValue::State(initial_value.clone()));
            component.effect_cleanups.push(None);
            component.prev_deps.push(None);
        }
        
        let state = match &component.hooks[hook_index] {
            HookValue::State(v) => v.clone(),
            _ => initial_value.clone(),
        };
        
        let setter = create_state_setter(component_id, hook_index);
        
        (state, setter)
    });
    
    let arr = JsArray::from(vec![state, setter]);
    Ok(JsValue::Array(Rc::new(RefCell::new(arr))))
}

fn create_state_setter(component_id: ComponentId, hook_index: usize) -> JsValue {
    JsValue::new_function(move |args| {
        if args.is_empty() {
            return Ok(JsValue::Undefined);
        }
        
        let new_value = &args[0];
        
        REACT_CONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            let component = ctx.get_or_create_component(component_id);
            
            if hook_index < component.hooks.len() {
                let current = match &component.hooks[hook_index] {
                    HookValue::State(v) => v.clone(),
                    _ => JsValue::Undefined,
                };
                
                let updated = match new_value {
                    JsValue::Function(f) => f.borrow().call(&[current.clone()]).unwrap_or(current),
                    v => v.clone(),
                };
                
                component.hooks[hook_index] = HookValue::State(updated);
            }
        });
        
        Ok(JsValue::Undefined)
    })
}

fn create_noop_setter() -> JsValue {
    JsValue::new_function(|_| Ok(JsValue::Undefined))
}

fn react_use_effect(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useEffect: requires effect function".to_string());
    }
    
    let effect_fn = match &args[0] {
        JsValue::Function(f) => f.clone(),
        _ => return Err("useEffect: first argument must be function".to_string()),
    };
    
    let deps: Option<Vec<JsValue>> = args.get(1).and_then(|d| {
        if let JsValue::Array(arr) = d {
            Some(arr.borrow().to_vec())
        } else {
            None
        }
    });
    
    REACT_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        let component_id = match ctx.current_component() {
            Some(id) => id,
            None => return,
        };
        
        let hook_index = ctx.next_hook_index();
        let component = ctx.get_or_create_component(component_id);
        
        if hook_index >= component.effect_cleanups.len() {
            component.effect_cleanups.push(None);
            component.prev_deps.push(deps.clone());
            
            let cleanup = effect_fn.borrow().call(&[]).ok();
            if let Some(JsValue::Function(f)) = cleanup {
                component.effect_cleanups[hook_index] = Some(f.borrow().clone());
            }
        } else {
            let should_run = match (&component.prev_deps[hook_index], &deps) {
                (None, None) => true,
                (None, Some(_)) => true,
                (Some(_), None) => true,
                (Some(prev), Some(current)) => !deps_equal(prev, current),
            };
            
            if should_run {
                if let Some(cleanup) = &component.effect_cleanups[hook_index] {
                    let _ = cleanup.call(&[]);
                }
                
                let cleanup = effect_fn.borrow().call(&[]).ok();
                if let Some(JsValue::Function(f)) = cleanup {
                    component.effect_cleanups[hook_index] = Some(f.borrow().clone());
                }
                
                component.prev_deps[hook_index] = deps;
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn deps_equal(a: &[JsValue], b: &[JsValue]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (ai, bi) in a.iter().zip(b.iter()) {
        if !values_equal(ai, bi) {
            return false;
        }
    }
    true
}

fn values_equal(a: &JsValue, b: &JsValue) -> bool {
    match (a, b) {
        (JsValue::Undefined, JsValue::Undefined) => true,
        (JsValue::Null, JsValue::Null) => true,
        (JsValue::Boolean(a), JsValue::Boolean(b)) => a == b,
        (JsValue::Number(a), JsValue::Number(b)) => a == b,
        (JsValue::String(a), JsValue::String(b)) => a == b,
        _ => false,
    }
}

fn react_use_layout_effect(args: &[JsValue]) -> Result<JsValue, String> {
    react_use_effect(args)
}

fn react_use_ref(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Null);
    
    REACT_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        let component_id = match ctx.current_component() {
            Some(id) => id,
            None => {
                let mut obj = JsObject::new();
                obj.set("current", initial_value);
                return Ok(JsValue::from(obj));
            }
        };
        
        let hook_index = ctx.next_hook_index();
        let component = ctx.get_or_create_component(component_id);
        
        if hook_index >= component.hooks.len() {
            let mut obj = JsObject::new();
            obj.set("current", initial_value);
            component.hooks.push(HookValue::Ref(obj));
            component.effect_cleanups.push(None);
            component.prev_deps.push(None);
        }
        
        match &component.hooks[hook_index] {
            HookValue::Ref(obj) => Ok(JsValue::from(obj.clone())),
            _ => {
                let mut obj = JsObject::new();
                obj.set("current", JsValue::Null);
                Ok(JsValue::from(obj))
            }
        }
    })
}

fn react_use_memo(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useMemo: requires factory function".to_string());
    }
    
    let factory = match &args[0] {
        JsValue::Function(f) => f.clone(),
        _ => return Err("useMemo: first argument must be function".to_string()),
    };
    
    let deps: Option<Vec<JsValue>> = args.get(1).and_then(|d| {
        if let JsValue::Array(arr) = d {
            Some(arr.borrow().to_vec())
        } else {
            None
        }
    });
    
    REACT_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        let component_id = match ctx.current_component() {
            Some(id) => id,
            None => return factory.borrow().call(&[]),
        };
        
        let hook_index = ctx.next_hook_index();
        let component = ctx.get_or_create_component(component_id);
        
        if hook_index >= component.hooks.len() {
            let value = factory.borrow().call(&[])?;
            component.hooks.push(HookValue::Memo(value.clone()));
            component.effect_cleanups.push(None);
            component.prev_deps.push(deps.clone());
            return Ok(value);
        }
        
        let should_recompute = match (&component.prev_deps[hook_index], &deps) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(prev), Some(current)) => !deps_equal(prev, current),
        };
        
        if should_recompute {
            let value = factory.borrow().call(&[])?;
            component.hooks[hook_index] = HookValue::Memo(value.clone());
            component.prev_deps[hook_index] = deps;
            return Ok(value);
        }
        
        match &component.hooks[hook_index] {
            HookValue::Memo(v) => Ok(v.clone()),
            _ => factory.borrow().call(&[]),
        }
    })
}

fn react_use_callback(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useCallback: requires callback function".to_string());
    }
    
    let callback = &args[0];
    let deps: Option<Vec<JsValue>> = args.get(1).and_then(|d| {
        if let JsValue::Array(arr) = d {
            Some(arr.borrow().to_vec())
        } else {
            None
        }
    });
    
    REACT_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        let component_id = match ctx.current_component() {
            Some(id) => id,
            None => return Ok(callback.clone()),
        };
        
        let hook_index = ctx.next_hook_index();
        let component = ctx.get_or_create_component(component_id);
        
        if hook_index >= component.hooks.len() {
            component.hooks.push(HookValue::Memo(callback.clone()));
            component.effect_cleanups.push(None);
            component.prev_deps.push(deps.clone());
            return Ok(callback.clone());
        }
        
        let should_update = match (&component.prev_deps[hook_index], &deps) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(prev), Some(current)) => !deps_equal(prev, current),
        };
        
        if should_update {
            component.hooks[hook_index] = HookValue::Memo(callback.clone());
            component.prev_deps[hook_index] = deps;
        }
        
        Ok(callback.clone())
    })
}

fn react_use_context(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useContext: requires context argument".to_string());
    }
    
    Ok(JsValue::new_object_with_data("context", "value".to_string()))
}

fn react_use_reducer(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useReducer: requires reducer function".to_string());
    }
    
    let _reducer = &args[0];
    let initial_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    
    let dispatch = JsValue::new_function(|_| Ok(JsValue::Undefined));
    
    let arr = JsArray::from(vec![initial_arg, dispatch]);
    Ok(JsValue::Array(Rc::new(RefCell::new(arr))))
}

fn react_use_imperative_handle(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn react_use_debug_value(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn react_use_id(_args: &[JsValue]) -> Result<JsValue, String> {
    static mut ID_COUNTER: usize = 0;
    let id = unsafe {
        ID_COUNTER += 1;
        ID_COUNTER
    };
    Ok(JsValue::String(format!(":r{}", id)))
}

fn react_use_transition(args: &[JsValue]) -> Result<JsValue, String> {
    let start_transition = JsValue::new_function(|args| {
        if let Some(JsValue::Function(f)) = args.get(0) {
            let _ = f.borrow().call(&[]);
        }
        Ok(JsValue::Undefined)
    });
    
    let arr = JsArray::from(vec![JsValue::Boolean(false), start_transition]);
    Ok(JsValue::Array(Rc::new(RefCell::new(arr))))
}

fn react_use_deferred_value(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    Ok(args[0].clone())
}

fn react_create_ref(_args: &[JsValue]) -> Result<JsValue, String> {
    let mut obj = JsObject::new();
    obj.set("current", JsValue::Null);
    Ok(JsValue::from(obj))
}

fn react_forward_ref(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("React.forwardRef: requires component function".to_string());
    }
    Ok(args[0].clone())
}

fn react_memo(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("React.memo: requires component function".to_string());
    }
    Ok(args[0].clone())
}

fn react_lazy(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("React.lazy: requires loader function".to_string());
    }
    Ok(JsValue::new_object_with_data("lazy", "component".to_string()))
}

fn react_suspense(args: &[JsValue]) -> Result<JsValue, String> {
    let fallback = args.get(0).cloned().unwrap_or(JsValue::Null);
    Ok(JsValue::new_object_with_data("suspense", format!("{:?}", fallback)))
}

fn react_render(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("render: requires element and container".to_string());
    }
    Ok(JsValue::Undefined)
}

fn react_unmount(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

fn is_void_element(tag: &str) -> bool {
    matches!(tag, 
        "area" | "base" | "br" | "col" | "embed" | 
        "hr" | "img" | "input" | "link" | "meta" | 
        "param" | "source" | "track" | "wbr"
    )
}
