# React 运行时支持方案

## 概述

React 需要 jrust-runtime 实现部分 React 运行时 API，以支持 React 应用的转译和执行。

---

## React 编译流程

### JSX 转换

```
React JSX → SWC/Babel 转换 → React.createElement() 调用
```

**输入**:
```jsx
function App() {
  const [count, setCount] = useState(0);
  return (
    <div className="container">
      <button onClick={() => setCount(count + 1)}>
        Clicked {count} times
      </button>
    </div>
  );
}
```

**SWC 转换后**:
```javascript
function App() {
  const [count, setCount] = useState(0);
  return React.createElement(
    "div",
    { className: "container" },
    React.createElement(
      "button",
      { onClick: () => setCount(count + 1) },
      "Clicked ",
      count,
      " times"
    )
  );
}
```

---

## 方案对比

### 方案 A: 完整虚拟 DOM 实现

**优点**: 完全兼容 React
**缺点**: 性能开销大，实现复杂

```rust
// 虚拟 DOM 节点
struct VirtualNode {
    type_: NodeType,
    key: Option<String>,
    props: HashMap<String, JsValue>,
    children: Vec<VirtualNode>,
    dom_node: Option<Node>,
}

enum NodeType {
    Element(String),    // "div", "span"
    FunctionComponent(Box<dyn Fn()>),
    ClassComponent(Box<dyn Component>),
    Text(String),
}
```

### 方案 B: 直接 DOM 操作 (推荐)

**优点**: 性能好，实现简单
**缺点**: 不支持某些 React 特性

```rust
// 直接生成 DOM 操作
// React.createElement("div", props, children)
// → document.createElement("div")
// → element.setAttribute(...)
// → element.appendChild(...)
```

---

## jrust-runtime 实现方案 (方案 B)

### 1. React.createElement 实现

```rust
// src/jrust-runtime/src/bindings/react.rs

use crate::core::{JsValue, JsObject, JsArray};
use crate::dom::{Document, Element, Node, TextNode};
use crate::bindings::BindingRegistry;

pub fn register_react_bindings(registry: &mut BindingRegistry) {
    // 核心 API
    registry.register("React.createElement", react_create_element);
    registry.register("React.Fragment", react_fragment);
    
    // Hooks (需要特殊处理)
    registry.register("useState", react_use_state);
    registry.register("useEffect", react_use_effect);
    registry.register("useRef", react_use_ref);
    registry.register("useMemo", react_use_memo);
    registry.register("useCallback", react_use_callback);
    
    // 组件 API
    registry.register("React.Component", react_component);
}
```

### 2. createElement 实现

```rust
/// React.createElement(type, props, ...children)
fn react_create_element(args: Vec<JsValue>) -> JsValue {
    let type_ = &args[0];
    let props = args.get(1).cloned().unwrap_or(JsValue::Object(JsObject::new()));
    let children = &args[2..];
    
    match type_ {
        // HTML 元素
        JsValue::String(tag_name) => {
            create_element_from_tag(&tag_name, &props, children)
        }
        
        // 函数组件
        JsValue::Function(component_fn) => {
            invoke_function_component(component_fn, &props)
        }
        
        // Fragment
        JsValue::Object(obj) if obj.is_fragment() => {
            create_fragment(children)
        }
        
        _ => JsValue::Null
    }
}

fn create_element_from_tag(tag: &str, props: &JsValue, children: &[JsValue]) -> JsValue {
    let element = Document::create_element(tag);
    
    if let JsValue::Object(props_obj) = props {
        // 设置属性
        for (key, value) in props_obj.properties.iter() {
            match key.as_str() {
                // 特殊属性
                "className" => {
                    element.set_attribute("class", &value.to_string());
                }
                "style" if value.is_object() => {
                    apply_style(&element, value);
                }
                "children" => {
                    // children 属性会被 children 参数覆盖
                }
                
                // 事件属性
                key if key.starts_with("on") => {
                    let event_type = key[2..].to_lowercase();
                    let handler = value.as_function().expect("Expected function");
                    element.add_event_listener(&event_type, handler.clone());
                }
                
                // 普通属性
                _ => {
                    element.set_attribute(key, &value.to_string());
                }
            }
        }
    }
    
    // 添加子元素
    for child in children {
        append_child(&element, child);
    }
    
    JsValue::Object(JsObject::from_element(element))
}

fn append_child(parent: &Element, child: &JsValue) {
    match child {
        JsValue::String(text) => {
            let text_node = Document::create_text_node(text);
            parent.append_child(&text_node);
        }
        JsValue::Number(n) => {
            let text_node = Document::create_text_node(&n.to_string());
            parent.append_child(&text_node);
        }
        JsValue::Object(obj) => {
            if let Some(element) = obj.as_element() {
                parent.append_child(&element);
            } else if let Some(text_node) = obj.as_text_node() {
                parent.append_child(&text_node);
            }
        }
        JsValue::Array(arr) => {
            for item in arr.iter() {
                append_child(parent, item);
            }
        }
        JsValue::Null | JsValue::Undefined | JsValue::Boolean(false) => {
            // 不渲染
        }
        _ => {
            // 其他类型转为字符串
            let text_node = Document::create_text_node(&child.to_string());
            parent.append_child(&text_node);
        }
    }
}
```

### 3. Fragment 实现

```rust
/// React.Fragment 或 <>...</>
fn react_fragment(args: Vec<JsValue>) -> JsValue {
    let children = &args[1..]; // 跳过 type (Fragment)
    
    // 创建文档片段
    let fragment = Document::create_document_fragment();
    
    for child in children {
        append_child_to_fragment(&fragment, child);
    }
    
    JsValue::Object(JsObject::from_document_fragment(fragment))
}
```

---

## Hooks 实现

### useState

```rust
/// useState(initialValue) -> [state, setState]
fn react_use_state(args: Vec<JsValue>) -> JsValue {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Undefined);
    
    // 获取当前组件实例
    let component = ComponentContext::current();
    
    // 获取或创建状态
    let hook_index = component.next_hook_index();
    let state_cell = component.get_or_init_hook(hook_index, || {
        RefCell::new(initial_value.clone())
    });
    
    // 创建 setState 函数
    let component_id = component.id();
    let state_cell_clone = state_cell.clone();
    let set_state = JsFunction::from_closure(move |new_value| {
        let mut state = state_cell_clone.borrow_mut();
        *state = match new_value {
            JsValue::Function(updater) => {
                // 支持函数式更新: setState(prev => prev + 1)
                updater.call(vec![state.clone()])
            }
            value => value
        };
        
        // 触发重渲染
        ComponentScheduler::schedule_rerender(component_id);
        
        JsValue::Undefined
    });
    
    // 返回 [state, setState]
    let state = state_cell.borrow().clone();
    JsValue::Array(JsArray::from_vec(vec![state, JsValue::Function(set_state)]))
}
```

### useEffect

```rust
/// useEffect(effect, deps?) -> void
fn react_use_effect(args: Vec<JsValue>) -> JsValue {
    let effect_fn = args[0].as_function().expect("Expected effect function");
    let deps = args.get(1).cloned();
    
    let component = ComponentContext::current();
    let hook_index = component.next_hook_index();
    
    // 检查依赖是否变化
    let should_run = deps.as_ref().map(|d| {
        let prev_deps = component.get_prev_deps(hook_index);
        deps_changed(&prev_deps, d)
    }).unwrap_or(true);
    
    if should_run {
        // 清理之前的 effect
        if let Some(cleanup) = component.get_effect_cleanup(hook_index) {
            cleanup.call(vec![]);
        }
        
        // 运行新的 effect
        let result = effect_fn.call(vec![]);
        
        // 保存清理函数
        if let JsValue::Function(cleanup) = result {
            component.set_effect_cleanup(hook_index, cleanup);
        }
        
        // 保存新的依赖
        if let Some(d) = deps {
            component.set_prev_deps(hook_index, d);
        }
    }
    
    JsValue::Undefined
}
```

### useRef

```rust
/// useRef(initialValue) -> { current: value }
fn react_use_ref(args: Vec<JsValue>) -> JsValue {
    let initial_value = args.get(0).cloned().unwrap_or(JsValue::Null);
    
    let component = ComponentContext::current();
    let hook_index = component.next_hook_index();
    
    let ref_obj = component.get_or_init_hook(hook_index, || {
        let obj = JsObject::new();
        obj.set("current", initial_value);
        obj
    });
    
    JsValue::Object(ref_obj)
}
```

### useMemo

```rust
/// useMemo(factory, deps) -> value
fn react_use_memo(args: Vec<JsValue>) -> JsValue {
    let factory = args[0].as_function().expect("Expected factory function");
    let deps = args.get(1).cloned();
    
    let component = ComponentContext::current();
    let hook_index = component.next_hook_index();
    
    // 检查依赖
    let should_recompute = deps.as_ref().map(|d| {
        let prev_deps = component.get_prev_deps(hook_index);
        deps_changed(&prev_deps, d)
    }).unwrap_or(true);
    
    if should_recompute {
        let value = factory.call(vec![]);
        component.set_memo_value(hook_index, value.clone());
        
        if let Some(d) = deps {
            component.set_prev_deps(hook_index, d);
        }
        
        value
    } else {
        component.get_memo_value(hook_index)
    }
}
```

---

## 组件上下文管理

```rust
// src/jrust-runtime/src/react/context.rs

thread_local! {
    static CURRENT_COMPONENT: RefCell<Option<ComponentId>> = RefCell::new(None);
    static COMPONENT_REGISTRY: RefCell<HashMap<ComponentId, ComponentInstance>> = RefCell::new(HashMap::new());
}

pub struct ComponentContext {
    id: ComponentId,
    hook_index: RefCell<usize>,
    hooks: RefCell<Vec<HookValue>>,
}

impl ComponentContext {
    pub fn current() -> Self {
        let id = CURRENT_COMPONENT.with(|c| {
            c.borrow().expect("No current component context")
        });
        
        COMPONENT_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            registry.get(&id).cloned().expect("Component not found")
        })
    }
    
    pub fn next_hook_index(&self) -> usize {
        let mut index = self.hook_index.borrow_mut();
        let current = *index;
        *index += 1;
        current
    }
    
    pub fn get_or_init_hook<F>(&self, index: usize, init: F) -> HookValue
    where
        F: FnOnce() -> HookValue
    {
        let mut hooks = self.hooks.borrow_mut();
        if hooks.len() <= index {
            hooks.push(init());
        }
        hooks[index].clone()
    }
}

/// 进入组件上下文
pub fn with_component_context<T, F>(component: &ComponentInstance, f: F) -> T
where
    F: FnOnce() -> T
{
    CURRENT_COMPONENT.with(|c| {
        *c.borrow_mut() = Some(component.id);
    });
    
    let result = f();
    
    CURRENT_COMPONENT.with(|c| {
        *c.borrow_mut() = None;
    });
    
    result
}
```

---

## jrust-translator 集成

### 识别 React 模式

```rust
// src/jrust-translator/src/codegen.rs

fn is_react_create_element(call: &CallExpression) -> bool {
    match &call.callee {
        Expression::Member(member) => {
            if let Expression::Identifier(ident) = &*member.object {
                ident.name == "React" && member.property.name() == "createElement"
            } else {
                false
            }
        }
        _ => false
    }
}

fn generate_react_element(call: &CallExpression) -> String {
    let args = &call.arguments;
    
    // React.createElement(type, props, ...children)
    let type_ = &args[0];
    let props = args.get(1);
    let children = &args[2..];
    
    // 生成 jrust-runtime 调用
    format!(
        "react_create_element(vec![{}, {}, {}])",
        generate_expression(type_),
        props.map(generate_expression).unwrap_or("JsValue::Undefined".to_string()),
        generate_children(children)
    )
}
```

### JSX 转换选项

```toml
# swc 配置
[[transform]]
syntax = "ecmascript"
jsx = true

[transform.react]
runtime = "automatic"  # 使用新的 JSX 转换
importSource = "react"  # 或 "jrust-runtime/react"
```

---

## 完整示例

### React 组件

```jsx
import { useState } from 'react';

function Counter() {
  const [count, setCount] = useState(0);
  
  return (
    <div className="counter">
      <button onClick={() => setCount(count + 1)}>
        Clicked {count} times
      </button>
    </div>
  );
}
```

### 转译后 Rust 代码

```rust
use jrust_runtime::bindings::react::*;

struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            count: Signal::new(0),
        }
    }
    
    fn render(&self) -> Element {
        let count = self.count.clone();
        
        react_create_element(vec![
            JsValue::String("div".to_string()),
            JsValue::Object(json!({"className": "counter"})),
            
            // button 子元素
            react_create_element(vec![
                JsValue::String("button".to_string()),
                JsValue::Object(json!({
                    "onClick": JsFunction::from_closure(move |_| {
                        count.set(count.get() + 1);
                        JsValue::Undefined
                    })
                })),
                JsValue::String("Clicked ".to_string()),
                JsValue::Number(self.count.get() as f64),
                JsValue::String(" times".to_string()),
            ]),
        ])
    }
}
```

---

## 限制与注意事项

### 支持的特性

- ✅ JSX 转换
- ✅ 函数组件
- ✅ useState, useRef, useMemo, useCallback
- ✅ 事件处理
- ✅ 条件渲染
- ✅ 列表渲染

### 不支持的特性

- ❌ 类组件 (React.Component)
- ❌ 生命周期方法 (componentDidMount 等)
- ❌ 错误边界
- ❌ React Portal
- ❌ React.Suspense
- ❌ React.Concurrent Mode

### 性能考虑

1. **避免虚拟 DOM diff**: 使用方案 B 直接操作 DOM
2. **批量更新**: 使用调度器批量处理状态更新
3. **事件委托**: 利用事件冒泡优化事件处理

---

## 下一步

1. 实现 `react_create_element` 绑定
2. 实现 Hooks 支持
3. 添加组件调度器
4. 测试常见 React 模式
5. 性能优化
