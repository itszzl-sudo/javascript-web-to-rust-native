//! Angular 运行时绑定
//!
//! 实现 Angular Ivy 渲染引擎的指令和依赖注入系统

use crate::bindings::BindingRegistry;
use crate::core::{JsValue, JsObject};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

thread_local! {
    static ANGULAR_CONTEXT: RefCell<AngularContext> = RefCell::new(AngularContext::new());
}

#[derive(Debug)]
struct AngularContext {
    current_view: Option<ViewContext>,
    view_stack: Vec<ViewContext>,
    injector_stack: Vec<Injector>,
    next_view_id: usize,
}

#[derive(Debug, Clone)]
struct ViewContext {
    id: usize,
    data: Vec<JsValue>,
    nodes: Vec<NodeRef>,
    bindings: Vec<Binding>,
    directives: Vec<DirectiveInstance>,
    current_node_index: usize,
}

#[derive(Debug, Clone)]
struct NodeRef {
    tag_name: String,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    styles: HashMap<String, String>,
    listeners: Vec<EventListener>,
    children: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug, Clone)]
struct Binding {
    target_index: usize,
    property_name: String,
    binding_type: BindingType,
}

#[derive(Debug, Clone)]
enum BindingType {
    Property,
    Attribute,
    Class,
    Style,
    Interpolation,
}

#[derive(Debug, Clone)]
struct EventListener {
    event_type: String,
    handler_index: usize,
}

#[derive(Debug, Clone)]
struct DirectiveInstance {
    directive_type: String,
    host_node: usize,
    inputs: HashMap<String, JsValue>,
    outputs: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
struct Injector {
    providers: HashMap<String, JsValue>,
    parent: Option<usize>,
}

impl AngularContext {
    fn new() -> Self {
        Self {
            current_view: None,
            view_stack: Vec::new(),
            injector_stack: Vec::new(),
            next_view_id: 0,
        }
    }
    
    fn next_view_id(&mut self) -> usize {
        let id = self.next_view_id;
        self.next_view_id += 1;
        id
    }
    
    fn create_view(&mut self) -> ViewContext {
        ViewContext {
            id: self.next_view_id(),
            data: Vec::new(),
            nodes: Vec::new(),
            bindings: Vec::new(),
            directives: Vec::new(),
            current_node_index: 0,
        }
    }
    
    fn push_view(&mut self, view: ViewContext) {
        self.view_stack.push(view);
        self.current_view = self.view_stack.last().cloned();
    }
    
    fn pop_view(&mut self) {
        self.view_stack.pop();
        self.current_view = self.view_stack.last().cloned();
    }
}

pub fn register_angular_bindings(registry: &mut BindingRegistry) {
    // Ivy 渲染指令
    registry.register("ɵɵelementStart", angular_element_start);
    registry.register("ɵɵelementEnd", angular_element_end);
    registry.register("ɵɵelement", angular_element);
    registry.register("ɵɵtext", angular_text);
    registry.register("ɵɵtextInterpolate", angular_text_interpolate);
    registry.register("ɵɵtextInterpolate1", angular_text_interpolate_1);
    registry.register("ɵɵtextInterpolateV", angular_text_interpolate_v);
    
    // 属性绑定
    registry.register("ɵɵproperty", angular_property);
    registry.register("ɵɵattribute", angular_attribute);
    registry.register("ɵɵclassProp", angular_class_prop);
    registry.register("ɵɵstyleProp", angular_style_prop);
    registry.register("ɵɵstyleMap", angular_style_map);
    registry.register("ɵɵclassMap", angular_class_map);
    
    // 事件绑定
    registry.register("ɵɵlistener", angular_listener);
    registry.register("ɵɵsyntheticHostListener", angular_synthetic_host_listener);
    
    // 指令
    registry.register("ɵɵdirectiveInheritsFeatures", angular_directive_inherits_features);
    registry.register("ɵɵInheritDefinitionFeature", angular_inherit_definition_feature);
    registry.register("ɵɵNgOnChangesFeature", angular_ng_on_changes_feature);
    
    // 模板引用
    registry.register("ɵɵreference", angular_reference);
    registry.register("ɵɵtemplate", angular_template);
    registry.register("ɵɵtemplateRefExtractor", angular_template_ref_extractor);
    
    // 内容投影
    registry.register("ɵɵprojection", angular_projection);
    registry.register("ɵɵprojectionDef", angular_projection_def);
    
    // Pipe
    registry.register("ɵɵpipe", angular_pipe);
    registry.register("ɵɵpipeBind1", angular_pipe_bind_1);
    registry.register("ɵɵpipeBind2", angular_pipe_bind_2);
    registry.register("ɵɵpipeBindV", angular_pipe_bind_v);
    
    // 组件/指令定义
    registry.register("ɵɵdefineComponent", angular_define_component);
    registry.register("ɵɵdefineDirective", angular_define_directive);
    registry.register("ɵɵdefinePipe", angular_define_pipe);
    registry.register("ɵɵdefineInjectable", angular_define_injectable);
    registry.register("ɵɵdefineInjector", angular_define_injector);
    
    // 依赖注入
    registry.register("ɵɵinject", angular_inject);
    registry.register("ɵɵinjectAttribute", angular_inject_attribute);
    registry.register("ɵɵinvalidFactory", angular_invalid_factory);
    
    // 生命周期
    registry.register("ɵɵNgDecorators", angular_ng_decorators);
    registry.register("ɵsetClassMetadata", angular_set_class_metadata);
    
    // 工具函数
    registry.register("ɵɵadvance", angular_advance);
    registry.register("ɵɵresetViewId", angular_reset_view_id);
    registry.register("ɵɵstorePropertyBindingMetadata", angular_store_property_binding_metadata);
    registry.register("ɵɵallocHostVars", angular_alloc_host_vars);
    
    // 组件实例化
    registry.register("ɵɵcomponentInstance", angular_component_instance);
    registry.register("ɵɵdirectiveInstance", angular_directive_instance);
}

// ==================== Ivy 渲染指令 ====================

fn angular_element_start(args: &[JsValue]) -> Result<JsValue, String> {
    let index = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => return Err("ɵɵelementStart: requires index argument".to_string()),
    };
    
    let tag_name = match args.get(1) {
        Some(JsValue::String(s)) => s.clone(),
        _ => "div".to_string(),
    };
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node = NodeRef {
                tag_name: tag_name.clone(),
                attributes: HashMap::new(),
                classes: Vec::new(),
                styles: HashMap::new(),
                listeners: Vec::new(),
                children: Vec::new(),
                parent: if index > 0 { Some(index - 1) } else { None },
            };
            
            while view.nodes.len() <= index {
                view.nodes.push(NodeRef {
                    tag_name: "placeholder".to_string(),
                    attributes: HashMap::new(),
                    classes: Vec::new(),
                    styles: HashMap::new(),
                    listeners: Vec::new(),
                    children: Vec::new(),
                    parent: None,
                });
            }
            
            view.nodes[index] = node;
            view.current_node_index = index;
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_element_end(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn angular_element(args: &[JsValue]) -> Result<JsValue, String> {
    let index = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 0,
    };
    
    let tag_name = match args.get(1) {
        Some(JsValue::String(s)) => s.clone(),
        _ => "div".to_string(),
    };
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node = NodeRef {
                tag_name,
                attributes: HashMap::new(),
                classes: Vec::new(),
                styles: HashMap::new(),
                listeners: Vec::new(),
                children: Vec::new(),
                parent: None,
            };
            
            while view.nodes.len() <= index {
                view.nodes.push(NodeRef {
                    tag_name: "placeholder".to_string(),
                    attributes: HashMap::new(),
                    classes: Vec::new(),
                    styles: HashMap::new(),
                    listeners: Vec::new(),
                    children: Vec::new(),
                    parent: None,
                });
            }
            
            view.nodes[index] = node;
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_text(args: &[JsValue]) -> Result<JsValue, String> {
    let index = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 0,
    };
    
    let text_content = match args.get(1) {
        Some(JsValue::String(s)) => s.clone(),
        _ => "".to_string(),
    };
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node = NodeRef {
                tag_name: "#text".to_string(),
                attributes: [("textContent".to_string(), text_content)].into_iter().collect(),
                classes: Vec::new(),
                styles: HashMap::new(),
                listeners: Vec::new(),
                children: Vec::new(),
                parent: None,
            };
            
            while view.nodes.len() <= index {
                view.nodes.push(NodeRef {
                    tag_name: "placeholder".to_string(),
                    attributes: HashMap::new(),
                    classes: Vec::new(),
                    styles: HashMap::new(),
                    listeners: Vec::new(),
                    children: Vec::new(),
                    parent: None,
                });
            }
            
            view.nodes[index] = node;
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_text_interpolate(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::String("".to_string()));
    }
    Ok(args[0].clone())
}

fn angular_text_interpolate_1(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Ok(JsValue::String("".to_string()));
    }
    
    let prefix = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => "".to_string(),
    };
    
    let value = &args[1];
    
    Ok(JsValue::String(format!("{}{}", prefix, value.to_string())))
}

fn angular_text_interpolate_v(args: &[JsValue]) -> Result<JsValue, String> {
    let result: String = args.iter().map(|v| v.to_string()).collect();
    Ok(JsValue::String(result))
}

// ==================== 属性绑定 ====================

fn angular_property(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵproperty: requires property name and value".to_string());
    }
    
    let prop_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵproperty: property name must be string".to_string()),
    };
    
    let value = &args[1];
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node_index = view.current_node_index;
            if node_index < view.nodes.len() {
                let binding = Binding {
                    target_index: node_index,
                    property_name: prop_name.clone(),
                    binding_type: BindingType::Property,
                };
                view.bindings.push(binding);
                
                view.nodes[node_index].attributes.insert(prop_name, value.to_string());
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_attribute(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵattribute: requires attribute name and value".to_string());
    }
    
    let attr_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵattribute: attribute name must be string".to_string()),
    };
    
    let value = &args[1];
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node_index = view.current_node_index;
            if node_index < view.nodes.len() {
                view.nodes[node_index].attributes.insert(attr_name, value.to_string());
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_class_prop(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵclassProp: requires class name and value".to_string());
    }
    
    let class_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵclassProp: class name must be string".to_string()),
    };
    
    let value = match &args[1] {
        JsValue::Boolean(b) => *b,
        _ => true,
    };
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node_index = view.current_node_index;
            if node_index < view.nodes.len() {
                if value {
                    view.nodes[node_index].classes.push(class_name);
                }
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_style_prop(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵstyleProp: requires style name and value".to_string());
    }
    
    let style_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵstyleProp: style name must be string".to_string()),
    };
    
    let value = args[1].to_string();
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node_index = view.current_node_index;
            if node_index < view.nodes.len() {
                view.nodes[node_index].styles.insert(style_name, value);
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_style_map(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    
    if let JsValue::Object(style_obj) = &args[0] {
        ANGULAR_CONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            
            if let Some(ref mut view) = ctx.current_view {
                let node_index = view.current_node_index;
                if node_index < view.nodes.len() {
                    for (key, value) in style_obj.properties.iter() {
                        view.nodes[node_index].styles.insert(key.clone(), value.to_string());
                    }
                }
            }
        });
    }
    
    Ok(JsValue::Undefined)
}

fn angular_class_map(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    
    if let JsValue::Object(class_obj) = &args[0] {
        ANGULAR_CONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            
            if let Some(ref mut view) = ctx.current_view {
                let node_index = view.current_node_index;
                if node_index < view.nodes.len() {
                    for (key, value) in class_obj.properties.iter() {
                        if value.to_string() == "true" {
                            view.nodes[node_index].classes.push(key.clone());
                        }
                    }
                }
            }
        });
    }
    
    Ok(JsValue::Undefined)
}

// ==================== 事件绑定 ====================

fn angular_listener(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵlistener: requires event name and handler".to_string());
    }
    
    let event_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵlistener: event name must be string".to_string()),
    };
    
    let _handler = &args[1];
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            let node_index = view.current_node_index;
            if node_index < view.nodes.len() {
                view.nodes[node_index].listeners.push(EventListener {
                    event_type: event_name,
                    handler_index: 0,
                });
            }
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_synthetic_host_listener(args: &[JsValue]) -> Result<JsValue, String> {
    angular_listener(args)
}

// ==================== 指令 ====================

fn angular_directive_inherits_features(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn angular_inherit_definition_feature(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn angular_ng_on_changes_feature(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

// ==================== 模板引用 ====================

fn angular_reference(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Ok(JsValue::Undefined);
    }
    
    let ref_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => "ref".to_string(),
    };
    
    Ok(JsValue::new_object_with_data("reference", ref_name))
}

fn angular_template(args: &[JsValue]) -> Result<JsValue, String> {
    let index = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 0,
    };
    
    Ok(JsValue::new_object_with_data("template", format!("template_{}", index)))
}

fn angular_template_ref_extractor(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

// ==================== 内容投影 ====================

fn angular_projection(args: &[JsValue]) -> Result<JsValue, String> {
    let slot = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 0,
    };
    
    Ok(JsValue::new_object_with_data("projection", format!("slot_{}", slot)))
}

fn angular_projection_def(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(args.to_vec()))
}

// ==================== Pipe ====================

fn angular_pipe(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵpipe: requires pipe definition".to_string());
    }
    
    Ok(args[0].clone())
}

fn angular_pipe_bind_1(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵpipeBind1: requires pipe and value".to_string());
    }
    Ok(args[1].clone())
}

fn angular_pipe_bind_2(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵpipeBind2: requires pipe and values".to_string());
    }
    Ok(args[1].clone())
}

fn angular_pipe_bind_v(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Err("ɵɵpipeBindV: requires pipe and values".to_string());
    }
    Ok(args[1].clone())
}

// ==================== 定义 ====================

fn angular_define_component(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdefineComponent: requires component config".to_string());
    }
    
    let obj = JsObject::new();
    obj.set("type", JsValue::String("component".to_string()));
    obj.set("config", args[0].clone());
    
    Ok(JsValue::Object(obj))
}

fn angular_define_directive(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdefineDirective: requires directive config".to_string());
    }
    
    let obj = JsObject::new();
    obj.set("type", JsValue::String("directive".to_string()));
    obj.set("config", args[0].clone());
    
    Ok(JsValue::Object(obj))
}

fn angular_define_pipe(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdefinePipe: requires pipe config".to_string());
    }
    
    let obj = JsObject::new();
    obj.set("type", JsValue::String("pipe".to_string()));
    obj.set("config", args[0].clone());
    
    Ok(JsValue::Object(obj))
}

fn angular_define_injectable(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdefineInjectable: requires injectable config".to_string());
    }
    
    let obj = JsObject::new();
    obj.set("type", JsValue::String("injectable".to_string()));
    obj.set("config", args[0].clone());
    
    Ok(JsValue::Object(obj))
}

fn angular_define_injector(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdefineInjector: requires injector config".to_string());
    }
    
    let injector = Injector {
        providers: HashMap::new(),
        parent: None,
    };
    
    Ok(JsValue::new_object_with_data("injector", format!("{:?}", injector)))
}

// ==================== 依赖注入 ====================

fn angular_inject(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵinject: requires token".to_string());
    }
    
    let token = &args[0];
    
    Ok(JsValue::new_object_with_data("inject", token.to_string()))
}

fn angular_inject_attribute(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵinjectAttribute: requires attribute name".to_string());
    }
    
    let attr_name = match &args[0] {
        JsValue::String(s) => s.clone(),
        _ => return Err("ɵɵinjectAttribute: attribute name must be string".to_string()),
    };
    
    Ok(JsValue::new_object_with_data("injectAttribute", attr_name))
}

fn angular_invalid_factory(_args: &[JsValue]) -> Result<JsValue, String> {
    Err("Invalid factory".to_string())
}

// ==================== 生命周期 ====================

fn angular_ng_decorators(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(args.to_vec()))
}

fn angular_set_class_metadata(args: &[JsValue]) -> Result<JsValue, String> {
    if args.len() < 2 {
        return Ok(JsValue::Undefined);
    }
    
    let obj = JsObject::new();
    obj.set("class", args[0].clone());
    obj.set("metadata", args[1].clone());
    
    Ok(JsValue::Object(obj))
}

// ==================== 工具函数 ====================

fn angular_advance(args: &[JsValue]) -> Result<JsValue, String> {
    let steps = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 1,
    };
    
    ANGULAR_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        
        if let Some(ref mut view) = ctx.current_view {
            view.current_node_index += steps;
        }
    });
    
    Ok(JsValue::Undefined)
}

fn angular_reset_view_id(_args: &[JsValue]) -> Result<JsValue, String> {
    ANGULAR_CONTEXT.with(|ctx| {
        ctx.borrow_mut().next_view_id = 0;
    });
    
    Ok(JsValue::Undefined)
}

fn angular_store_property_binding_metadata(_args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Undefined)
}

fn angular_alloc_host_vars(args: &[JsValue]) -> Result<JsValue, String> {
    let count = match args.get(0) {
        Some(JsValue::Number(n)) => *n as usize,
        _ => 0,
    };
    
    Ok(JsValue::new_object_with_data("hostVars", format!("allocated: {}", count)))
}

// ==================== 组件实例化 ====================

fn angular_component_instance(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵcomponentInstance: requires component type".to_string());
    }
    
    Ok(JsValue::new_object_with_data("componentInstance", args[0].to_string()))
}

fn angular_directive_instance(args: &[JsValue]) -> Result<JsValue, String> {
    if args.is_empty() {
        return Err("ɵɵdirectiveInstance: requires directive type".to_string());
    }
    
    Ok(JsValue::new_object_with_data("directiveInstance", args[0].to_string()))
}
