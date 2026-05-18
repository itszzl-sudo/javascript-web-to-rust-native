# Svelte 运行时绑定实现

## 概述

Svelte 编译后生成标准 DOM 操作函数，jrust-runtime 需要实现这些函数的绑定。

---

## Svelte 编译产物分析

### 编译后的标准函数

```javascript
// Svelte 编译器生成的函数
function create_fragment(ctx) {
  let button;
  let t0;
  let t1;
  
  // c() - create: 创建 DOM 节点
  function c() {
    button = element("button");
    t0 = text("Clicked ");
    t1 = text(count);
  }
  
  // m() - mount: 挂载到 DOM
  function m(target, anchor) {
    insert(target, button, anchor);
    append(button, t0);
    append(button, t1);
  }
  
  // p() - update: 更新 DOM
  function p(ctx, dirty) {
    if (dirty & 1) {
      set_data(t1, ctx[0]);
    }
  }
  
  // d() - destroy: 销毁 DOM
  function d(detaching) {
    if (detaching) detach(button);
  }
  
  return { c, m, p, d };
}
```

---

## jrust-runtime 实现方案

### 1. 核心绑定函数

```rust
// src/jrust-runtime/src/bindings/svelte.rs

use crate::core::{JsValue, JsObject};
use crate::dom::{Document, Element, Node, TextNode};
use crate::bindings::BindingRegistry;

pub fn register_svelte_bindings(registry: &mut BindingRegistry) {
    // DOM 创建函数
    registry.register("svelte_element", svelte_element);
    registry.register("svelte_text", svelte_text);
    registry.register("svelte_space", svelte_space);
    registry.register("svelte_empty", svelte_empty);
    
    // DOM 操作函数
    registry.register("svelte_insert", svelte_insert);
    registry.register("svelte_append", svelte_append);
    registry.register("svelte_detach", svelte_detach);
    
    // 属性设置函数
    registry.register("svelte_attr", svelte_attr);
    registry.register("svelte_set_data", svelte_set_data);
    registry.register("svelte_set_style", svelte_set_style);
    registry.register("svelte_add_class", svelte_add_class);
    
    // 事件监听函数
    registry.register("svelte_listen", svelte_listen);
    
    // 组件函数
    registry.register("svelte_component", svelte_component);
}
```

### 2. DOM 创建函数实现

```rust
/// 创建元素节点
/// element(tag: string) -> Element
fn svelte_element(args: Vec<JsValue>) -> JsValue {
    let tag_name = args[0].as_string().expect("Expected tag name");
    let element = Document::create_element(&tag_name);
    JsValue::Object(JsObject::from_element(element))
}

/// 创建文本节点
/// text(content: string) -> TextNode
fn svelte_text(args: Vec<JsValue>) -> JsValue {
    let content = args[0].as_string().expect("Expected text content");
    let text_node = Document::create_text_node(&content);
    JsValue::Object(JsObject::from_text_node(text_node))
}

/// 创建空格节点
/// space() -> TextNode
fn svelte_space(_args: Vec<JsValue>) -> JsValue {
    let text_node = Document::create_text_node(" ");
    JsValue::Object(JsObject::from_text_node(text_node))
}

/// 创建空节点（注释节点）
/// empty() -> Comment
fn svelte_empty(_args: Vec<JsValue>) -> JsValue {
    let comment = Document::create_comment("");
    JsValue::Object(JsObject::from_comment(comment))
}
```

### 3. DOM 操作函数实现

```rust
/// 插入节点
/// insert(target: Node, node: Node, anchor?: Node) -> void
fn svelte_insert(args: Vec<JsValue>) -> JsValue {
    let target = args[0].as_node().expect("Expected target node");
    let node = args[1].as_node().expect("Expected node to insert");
    
    if args.len() > 2 {
        let anchor = args[2].as_node().expect("Expected anchor node");
        target.insert_before(&node, Some(&anchor));
    } else {
        target.append_child(&node);
    }
    
    JsValue::Undefined
}

/// 追加子节点
/// append(parent: Node, child: Node) -> void
fn svelte_append(args: Vec<JsValue>) -> JsValue {
    let parent = args[0].as_node().expect("Expected parent node");
    let child = args[1].as_node().expect("Expected child node");
    parent.append_child(&child);
    JsValue::Undefined
}

/// 移除节点
/// detach(node: Node) -> void
fn svelte_detach(args: Vec<JsValue>) -> JsValue {
    let node = args[0].as_node().expect("Expected node to detach");
    if let Some(parent) = node.parent_node() {
        parent.remove_child(&node);
    }
    JsValue::Undefined
}
```

### 4. 属性设置函数实现

```rust
/// 设置属性
/// attr(element: Element, name: string, value: string) -> void
fn svelte_attr(args: Vec<JsValue>) -> JsValue {
    let element = args[0].as_element().expect("Expected element");
    let name = args[1].as_string().expect("Expected attribute name");
    let value = args[2].as_string().expect("Expected attribute value");
    
    element.set_attribute(&name, &value);
    JsValue::Undefined
}

/// 设置文本内容
/// set_data(text_node: TextNode, value: string) -> void
fn svelte_set_data(args: Vec<JsValue>) -> JsValue {
    let text_node = args[0].as_text_node().expect("Expected text node");
    let value = args[1].as_string().expect("Expected value");
    
    text_node.set_data(&value);
    JsValue::Undefined
}

/// 设置样式
/// set_style(element: Element, property: string, value: string, important?: boolean) -> void
fn svelte_set_style(args: Vec<JsValue>) -> JsValue {
    let element = args[0].as_element().expect("Expected element");
    let property = args[1].as_string().expect("Expected property");
    let value = args[2].as_string().expect("Expected value");
    let important = args.get(3).map(|v| v.as_boolean()).flatten().unwrap_or(false);
    
    let priority = if important { "important" } else { "" };
    element.style().set_property(&property, &value, priority);
    
    JsValue::Undefined
}

/// 添加/移除类
/// toggle_class(element: Element, class: string, value: boolean) -> void
fn svelte_toggle_class(args: Vec<JsValue>) -> JsValue {
    let element = args[0].as_element().expect("Expected element");
    let class = args[1].as_string().expect("Expected class name");
    let value = args[2].as_boolean().expect("Expected boolean value");
    
    if value {
        element.class_list().add_1(&class);
    } else {
        element.class_list().remove_1(&class);
    }
    
    JsValue::Undefined
}
```

### 5. 事件监听函数实现

```rust
/// 添加事件监听
/// listen(node: EventTarget, event: string, handler: Function, options?: object) -> Function
fn svelte_listen(args: Vec<JsValue>) -> JsValue {
    let target = args[0].as_event_target().expect("Expected event target");
    let event_type = args[1].as_string().expect("Expected event type");
    let handler = args[2].as_function().expect("Expected handler function");
    
    let mut options = EventListenerOptions::default();
    if args.len() > 3 {
        let opts_obj = args[3].as_object().expect("Expected options object");
        if let Some(capture) = opts_obj.get("capture").and_then(|v| v.as_boolean()) {
            options.capture = capture;
        }
        if let Some(passive) = opts_obj.get("passive").and_then(|v| v.as_boolean()) {
            options.passive = passive;
        }
        if let Some(once) = opts_obj.get("once").and_then(|v| v.as_boolean()) {
            options.once = once;
        }
    }
    
    // 注册事件监听器
    let listener_id = target.add_event_listener_with_options(
        &event_type,
        handler.clone(),
        options
    );
    
    // 返回移除监听器的函数
    JsValue::Function(JsFunction::from_closure(move |_| {
        target.remove_event_listener(&event_type, listener_id);
        JsValue::Undefined
    }))
}
```

### 6. 响应式系统实现

```rust
/// Svelte 的响应式绑定
/// binding(node: Node, value: () => any, update: (value) => void) -> void
fn svelte_binding(args: Vec<JsValue>) -> JsValue {
    let node = args[0].as_node().expect("Expected node");
    let getter = args[1].as_function().expect("Expected getter function");
    let setter = args[2].as_function().expect("Expected setter function");
    
    // 创建响应式绑定
    let binding = ReactiveBinding {
        node: node.clone(),
        getter: getter.clone(),
        setter: setter.clone(),
    };
    
    // 注册到响应式系统
    ReactiveSystem::register(binding);
    
    JsValue::Undefined
}
```

---

## 完整示例

### Svelte 组件

```svelte
<script>
  let count = 0;
  function increment() {
    count += 1;
  }
</script>

<button on:click={increment}>
  Clicked {count} times
</button>

<style>
  button {
    color: blue;
  }
</style>
```

### 编译后 JS

```javascript
function create_fragment(ctx) {
  let button;
  let t0;
  let t1_value = ctx[0]; // count
  let t1;
  let dispose;
  
  return {
    c() {
      button = element("button");
      t0 = text("Clicked ");
      t1 = text(t1_value);
      attr(button, "style", "color: blue");
    },
    
    m(target, anchor) {
      insert(target, button, anchor);
      append(button, t0);
      append(button, t1);
      dispose = listen(button, "click", ctx[1]); // increment
    },
    
    p(ctx, dirty) {
      if (dirty & 1 && t1_value !== (t1_value = ctx[0])) {
        set_data(t1, t1_value);
      }
    },
    
    d(detaching) {
      if (detaching) detach(button);
      dispose();
    }
  };
}

function instance($$self, $$props, $$invalidate) {
  let count = 0;
  
  function increment() {
    $$invalidate(0, count += 1);
  }
  
  return [count, increment];
}

class Component extends SvelteComponent {
  constructor(options) {
    super();
    init(this, options, instance, create_fragment, safe_not_equal, []);
  }
}

export default Component;
```

### jrust-runtime 执行

```rust
use jrust_runtime::bindings::svelte::register_svelte_bindings;

fn main() {
    // 1. 初始化运行时
    let mut registry = BindingRegistry::new();
    register_svelte_bindings(&mut registry);
    
    // 2. 创建 Svelte 组件实例
    let component = Component::new(ComponentOptions {
        target: Document::body(),
        props: HashMap::new(),
    });
    
    // 3. 组件生命周期
    // create_fragment().c()  - 创建
    // create_fragment().m()  - 挂载
    // create_fragment().p()  - 更新
    // create_fragment().d()  - 销毁
}
```

---

## 性能优化

### 1. 静态提升

```rust
// Svelte 编译器会提升静态内容
// jrust-runtime 应复用这些静态节点

thread_local! {
    static STATIC_NODES: RefCell<HashMap<usize, Node>> = RefCell::new(HashMap::new());
}

fn get_or_create_static_node(id: usize, create_fn: impl FnOnce() -> Node) -> Node {
    STATIC_NODES.with(|nodes| {
        let mut nodes = nodes.borrow_mut();
        nodes.entry(id).or_insert_with(create_fn).clone()
    })
}
```

### 2. 批量更新

```rust
// 使用 dirty 标志位批量更新
pub struct UpdateContext {
    dirty: u32,  // 位图标记哪些变量变化了
}

impl UpdateContext {
    pub fn is_dirty(&self, bit: u32) -> bool {
        (self.dirty & (1 << bit)) != 0
    }
    
    pub fn set_dirty(&mut self, bit: u32) {
        self.dirty |= (1 << bit);
    }
}
```

---

## 测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_svelte_element() {
        let mut registry = BindingRegistry::new();
        register_svelte_bindings(&mut registry);
        
        let result = registry.call("svelte_element", vec![JsValue::String("button".to_string())]);
        
        assert!(result.is_object());
        let element = result.as_element().unwrap();
        assert_eq!(element.tag_name(), "BUTTON");
    }
    
    #[test]
    fn test_svelte_counter() {
        // 模拟 Svelte 计数器组件
        let count = 0;
        let button = Document::create_element("button");
        
        // 点击事件
        let mut count_ref = count;
        button.add_event_listener("click", move |_| {
            count_ref += 1;
            JsValue::Undefined
        });
        
        // 验证初始状态
        assert_eq!(count_ref, 0);
        
        // 模拟点击
        button.dispatch_event(&Event::new("click"));
        assert_eq!(count_ref, 1);
    }
}
```

---

## 与 jrust-translator 集成

在 jrust-translator 中识别 Svelte 模式：

```rust
// src/jrust-translator/src/codegen.rs

fn generate_svelte_runtime_call(function: &str, args: &[Expression]) -> String {
    match function {
        "element" => format!("svelte_element({})", format_args(args)),
        "text" => format!("svelte_text({})", format_args(args)),
        "insert" => format!("svelte_insert({})", format_args(args)),
        "append" => format!("svelte_append({})", format_args(args)),
        "attr" => format!("svelte_attr({})", format_args(args)),
        "listen" => format!("svelte_listen({})", format_args(args)),
        "set_data" => format!("svelte_set_data({})", format_args(args)),
        // ...
        _ => format!("{}({})", function, format_args(args)),
    }
}
```

---

## 总结

Svelte 支持的核心是：
1. 实现 Svelte 运行时函数绑定
2. 在 jrust-translator 中识别 Svelte 编译产物
3. 映射到 jrust-runtime 的 DOM API

Svelte 的编译时特性使得它成为最容易支持的框架之一。
