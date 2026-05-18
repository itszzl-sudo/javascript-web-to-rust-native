//! Qwik 运行时绑定
//!
//! 实现 Qwik 的可恢复性、延迟加载和信号系统

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static QWIK_CONTEXT: RefCell<QwikContext> = RefCell::new(QwikContext::new());
}

#[derive(Debug)]
struct QwikContext {
    signals: HashMap<String, SignalState>,
    stores: HashMap<String, StoreState>,
    scopes: Vec<Scope>,
    next_signal_id: usize,
}

#[derive(Debug, Clone)]
struct SignalState {
    id: String,
    value: JsValue,
    subscribers: Vec<usize>,
}

#[derive(Debug, Clone)]
struct StoreState {
    id: String,
    value: JsValue,
}

#[derive(Debug, Clone)]
struct Scope {
    id: usize,
    signals: Vec<String>,
    stores: Vec<String>,
}

impl QwikContext {
    fn new() -> Self {
        Self {
            signals: HashMap::new(),
            stores: HashMap::new(),
            scopes: Vec::new(),
            next_signal_id: 0,
        }
    }
    
    fn next_signal_id(&mut self) -> String {
        let id = format!("sig_{}", self.next_signal_id);
        self.next_signal_id += 1;
        id
    }
}

pub fn register_qwik_bindings(registry: &mut BindingRegistry) {
    // 信号系统
    registry.register("useSignal", qwik_use_signal);
    registry.register("useStore", qwik_use_store);
    registry.register("useContext", qwik_use_context);
    registry.register("useContextProvider", qwik_use_context_provider);
    registry.register("useRef", qwik_use_ref);
    registry.register("useEnv", qwik_use_env);
    
    // 计算属性
    registry.register("useComputed$", qwik_use_computed);
    registry.register("useResource$", qwik_use_resource);
    registry.register("useResourceStream$", qwik_use_resource_stream);
    
    // 副作用
    registry.register("useEffect$", qwik_use_effect);
    registry.register("useVisibleTask$", qwik_use_visible_task);
    registry.register("useTask$", qwik_use_task);
    registry.register("useCleanup$", qwik_use_cleanup);
    
    // 生命周期
    registry.register("useMount$", qwik_use_mount);
    registry.register("useUnmount$", qwik_use_unmount);
    
    // 事件处理
    registry.register("$", qwik_dollar);
    registry.register("$(value)", qwik_dollar_value);
    
    // 组件
    registry.register("component$", qwik_component);
    registry.register("qrl", qwik_qrl);
    registry.register("inlinedQrl", qwik_inlined_qrl);
    
    // Props
    registry.register("useProps", qwik_use_props);
    registry.register("useProp", qwik_use_prop);
    
    // 样式
    registry.register("useStyles$", qwik_use_styles);
    registry.register("useScopedStyles$", qwik_use_scoped_styles);
    
    // 服务端
    registry.register("useServerData", qwik_use_server_data);
    registry.register("useClientData", qwik_use_client_data);
    
    // 路由
    registry.register("useLocation", qwik_use_location);
    registry.register("useNavigate", qwik_use_navigate);
    registry.register("useParams", qwik_use_params);
    registry.register("useQueryParam", qwik_use_query_param);
    
    // 表单
    registry.register("useForm", qwik_use_form);
    registry.register("useField", qwik_use_field);
    
    // ID 生成
    registry.register("useId", qwik_use_id);
    registry.register("useUniqueId", qwik_use_unique_id);
    
    // 服务端函数
    registry.register("server$", qwik_server);
    registry.register("routeLoader$", qwik_route_loader);
    registry.register("routeAction$", qwik_route_action);
    
    // 乐观更新
    registry.register("useSubmission", qwik_use_submission);
    registry.register("use enhancing", qwik_use_enhancing);
    
    // 懒加载
    registry.register("useOn", qwik_use_on);
    registry.register("useOnDocument", qwik_use_on_document);
    registry.register("useOnWindow", qwik_use_on_window);
    
    // 服务
    registry.register("useService", qwik_use_service);
    registry.register("useInjector", qwik_use_injector);
}

// ==================== 信号系统 ====================

fn qwik_use_signal(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    
    let signal_id = QWIK_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let id = ctx.next_signal_id();
        
        ctx.signals.insert(id.clone(), SignalState {
            id: id.clone(),
            value: initial_value.clone(),
            subscribers: Vec::new(),
        });
        
        id
    });
    
    let mut signal_obj = JsObject::new();
    signal_obj.set("id", JsValue::String(signal_id.clone()));
    signal_obj.set("value", initial_value);
    signal_obj.set("type", JsValue::String("signal".to_string()));
    
    Ok(JsValue::from(signal_obj))
}

fn qwik_use_store(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_state = args.get(0).cloned().unwrap_or(JsValue::new_object());
    
    let store_id = QWIK_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let id = format!("store_{}", ctx.next_signal_id);
        ctx.next_signal_id += 1;
        
        ctx.stores.insert(id.clone(), StoreState {
            id: id.clone(),
            value: initial_state.clone(),
        });
        
        id
    });
    
    let mut store_obj = JsObject::new();
    store_obj.set("id", JsValue::String(store_id));
    store_obj.set("value", initial_state);
    store_obj.set("type", JsValue::String("store".to_string()));
    
    Ok(JsValue::from(store_obj))
}

fn qwik_use_context(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useContext: requires context token".to_string());
    }
    
    let ctx_token = &args[0];
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("context".to_string()));
    obj.set("token", ctx_token.clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_context_provider(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("useContextProvider: requires context token and value".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("contextProvider".to_string()));
    obj.set("token", args[0].clone());
    obj.set("value", args[1].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_ref(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    
    let mut obj = JsObject::new();
    obj.set("current", initial_value);
    
    Ok(JsValue::from(obj))
}

fn qwik_use_env(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useEnv: requires key".to_string());
    }
    
    Ok(JsValue::new_object_with_data("env", args[0].to_string()))
}

// ==================== 计算属性 ====================

fn qwik_use_computed(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useComputed$: requires compute function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("computed".to_string()));
    obj.set("computeFn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_resource(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useResource$: requires resource function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("resource".to_string()));
    obj.set("resourceFn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_resource_stream(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useResourceStream$: requires stream function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("resourceStream".to_string()));
    obj.set("streamFn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

// ==================== 副作用 ====================

fn qwik_use_effect(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useEffect$: requires effect function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("effect", args[0].to_string()))
}

fn qwik_use_visible_task(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useVisibleTask$: requires task function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("visibleTask".to_string()));
    obj.set("taskFn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_task(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useTask$: requires task function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("task", args[0].to_string()))
}

fn qwik_use_cleanup(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useCleanup$: requires cleanup function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("cleanup", args[0].to_string()))
}

// ==================== 生命周期 ====================

fn qwik_use_mount(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useMount$: requires mount function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("lifecycle", format!("mount: {}", args[0].to_string())))
}

fn qwik_use_unmount(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useUnmount$: requires unmount function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("lifecycle", format!("unmount: {}", args[0].to_string())))
}

// ==================== 事件处理 ====================

fn qwik_dollar(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("$: requires function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("qrl".to_string()));
    obj.set("fn", args[0].clone());
    obj.set("lazy", JsValue::Boolean(true));
    
    Ok(JsValue::from(obj))
}

fn qwik_dollar_value(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    
    Ok(JsValue::new_object_with_data("qrl-value", args[0].to_string()))
}

// ==================== 组件 ====================

fn qwik_component(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("component$: requires component function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("component".to_string()));
    obj.set("renderFn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_qrl(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("qrl: requires path or function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("qrl".to_string()));
    obj.set("target", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_inlined_qrl(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("inlinedQrl: requires function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("inlinedQrl", args[0].to_string()))
}

// ==================== Props ====================

fn qwik_use_props(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("props", "all".to_string()))
}

fn qwik_use_prop(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useProp: requires prop name".to_string());
    }
    
    Ok(JsValue::new_object_with_data("prop", args[0].to_string()))
}

// ==================== 样式 ====================

fn qwik_use_styles(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useStyles$: requires styles".to_string());
    }
    
    Ok(JsValue::new_object_with_data("styles", args[0].to_string()))
}

fn qwik_use_scoped_styles(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useScopedStyles$: requires styles".to_string());
    }
    
    Ok(JsValue::new_object_with_data("scopedStyles", args[0].to_string()))
}

// ==================== 服务端 ====================

fn qwik_use_server_data(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useServerData: requires key".to_string());
    }
    
    Ok(JsValue::new_object_with_data("serverData", args[0].to_string()))
}

fn qwik_use_client_data(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useClientData: requires key".to_string());
    }
    
    Ok(JsValue::new_object_with_data("clientData", args[0].to_string()))
}

// ==================== 路由 ====================

fn qwik_use_location(_args: &[JsValue]) -> Result<JsValue, String> {
    let mut obj = JsObject::new();
    obj.set("pathname", JsValue::String("/".to_string()));
    obj.set("search", JsValue::String("".to_string()));
    obj.set("hash", JsValue::String("".to_string()));
    
    Ok(JsValue::from(obj))
}

fn qwik_use_navigate(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_function(|args: &[JsValue]| {
        if let Some(path) = args.get(0) {
            Ok(JsValue::new_object_with_data("navigate", path.to_string()))
        } else {
            Ok(JsValue::Undefined)
        }
    }))
}

fn qwik_use_params(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object())
}

fn qwik_use_query_param(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useQueryParam: requires param name".to_string());
    }
    
    Ok(JsValue::new_object_with_data("queryParam", args[0].to_string()))
}

// ==================== 表单 ====================

fn qwik_use_form(args: &[JsValue]) -> Result<JsValue, String> {
    let initial_values = args.get(0).cloned().unwrap_or(JsValue::new_object());
    
    let mut obj = JsObject::new();
    obj.set("values", initial_values);
    obj.set("valid", JsValue::Boolean(true));
    obj.set("dirty", JsValue::Boolean(false));
    
    Ok(JsValue::from(obj))
}

fn qwik_use_field(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useField: requires field name".to_string());
    }
    
    let field_name = &args[0];
    let initial_value = args.get(1).cloned().unwrap_or(JsValue::String("".to_string()));
    
    let mut obj = JsObject::new();
    obj.set("name", field_name.clone());
    obj.set("value", initial_value);
    obj.set("valid", JsValue::Boolean(true));
    obj.set("touched", JsValue::Boolean(false));
    
    Ok(JsValue::from(obj))
}

// ==================== ID 生成 ====================

fn qwik_use_id(_args: &[JsValue]) -> Result<JsValue, String> {
    static mut ID_COUNTER: usize = 0;
    let id = unsafe {
        ID_COUNTER += 1;
        ID_COUNTER
    };
    Ok(JsValue::String(format!("q{}", id)))
}

fn qwik_use_unique_id(_args: &[JsValue]) -> Result<JsValue, String> {
    qwik_use_id(&[])
}

// ==================== 服务端函数 ====================

fn qwik_server(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("server$: requires server function".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("serverFunction".to_string()));
    obj.set("fn", args[0].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_route_loader(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("routeLoader$: requires loader function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("routeLoader", args[0].to_string()))
}

fn qwik_route_action(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("routeAction$: requires action function".to_string());
    }
    
    Ok(JsValue::new_object_with_data("routeAction", args[0].to_string()))
}

// ==================== 乐观更新 ====================

fn qwik_use_submission(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useSubmission: requires action".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("action", args[0].clone());
    obj.set("status", JsValue::String("idle".to_string()));
    
    Ok(JsValue::from(obj))
}

fn qwik_use_enhancing(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("enhancing", "true".to_string()))
}

// ==================== 懒加载 ====================

fn qwik_use_on(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("useOn: requires event and handler".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("useOn".to_string()));
    obj.set("event", args[0].clone());
    obj.set("handler", args[1].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_on_document(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("useOnDocument: requires event and handler".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("useOnDocument".to_string()));
    obj.set("event", args[0].clone());
    obj.set("handler", args[1].clone());
    
    Ok(JsValue::from(obj))
}

fn qwik_use_on_window(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("useOnWindow: requires event and handler".to_string());
    }
    
    let mut obj = JsObject::new();
    obj.set("type", JsValue::String("useOnWindow".to_string()));
    obj.set("event", args[0].clone());
    obj.set("handler", args[1].clone());
    
    Ok(JsValue::from(obj))
}

// ==================== 服务 ====================

fn qwik_use_service(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("useService: requires service token".to_string());
    }
    
    Ok(JsValue::new_object_with_data("service", args[0].to_string()))
}

fn qwik_use_injector(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::new_object_with_data("injector", "ready".to_string()))
}
