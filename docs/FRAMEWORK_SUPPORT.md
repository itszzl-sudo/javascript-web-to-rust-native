# Web 框架支持方案

**更新日期**: 2026-05-18

## 核心原则

> **"浏览器不关心源码框架，只关心打包后的 JavaScript"**

所有框架最终都打包为标准 JavaScript，jrust-translator 统一处理。

---

## 框架支持矩阵

| 框架 | 支持级别 | 预编译方案 | 运行时依赖 | 实现状态 |
|------|---------|-----------|-----------|---------|
| **Vue 3** | ✅ 完整支持 | Vite + @vitejs/plugin-vue | 极小 | ✅ 已实现 |
| **Svelte** | ✅ 原生支持 | Svelte 编译器 | 几乎无 | ✅ 已实现 |
| **Preact** | ✅ 支持 | SWC JSX 转换 | 极小 | ✅ 已实现 |
| **SolidJS** | ✅ 支持 | Solid 编译器 | 极小 | ✅ 已实现 |
| **React** | ⚠️ 部分支持 | SWC JSX 转换 | React 运行时 | 🚧 规划中 |
| **Angular** | ⚠️ 复杂 | AOT 编译 | Angular 运行时 | 🚧 规划中 |

---

## 一、Vue 3 支持 (✅ 已验证)

### 技术方案

```
Vue SFC → @vitejs/plugin-vue → 预编译 render 函数 → 标准 JS
```

### 关键特性

1. **模板预编译**: 无运行时模板编译
2. **无 eval/new Function**: 符合 CSP
3. **静态提升**: 性能优化
4. **PatchFlag**: 增量更新

### 集成步骤

```bash
# 1. Vue 项目构建
npm run build  # Vite 自动预编译模板

# 2. 转译为 Rust
jrust-translator dist/assets/index.js -o output.rs

# 3. 编译运行
cargo build --release
```

### Vue 运行时函数映射

| Vue 函数 | jrust-runtime 实现 |
|---------|-------------------|
| `createElementBlock` | `Element::create()` |
| `createBaseVNode` | `Element::create_text_node()` |
| `toDisplayString` | `JsValue::to_string()` |
| `normalizeClass` | `Element::set_class()` |
| `openBlock` | (虚拟 DOM 块标记) |

### 验证状态

- ✅ Vue 3 项目成功构建
- ✅ 无 eval/new Function
- ✅ jrust-translator 可以转译
- ✅ 事件绑定正常工作

---

## 二、Svelte 支持 (✅ 原生支持)

### 技术方案

```
Svelte 组件 → Svelte 编译器 → 纯 JS (无虚拟 DOM)
```

### Svelte 优势

1. **编译时框架**: 无运行时开销
2. **无虚拟 DOM**: 直接 DOM 操作
3. **极小产物**: ~2KB gzip
4. **无特殊语法**: 标准 JS 输出

### 编译示例

**输入** (App.svelte):
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
```

**输出** (编译后 JS):
```javascript
function create_fragment(ctx) {
  let button;
  let t0;
  let t1;
  let count = ctx[0];
  
  return {
    c() {
      button = element("button");
      t0 = text("Clicked ");
      t1 = text(count);
    },
    m(target, anchor) {
      insert(target, button, anchor);
      append(button, t0);
      append(button, t1);
    },
    p(ctx, dirty) {
      if (dirty & 1) set_data(t1, ctx[0]);
    }
  };
}
```

### jrust-runtime 映射

| Svelte 函数 | jrust-runtime |
|------------|---------------|
| `element("div")` | `Document::create_element("div")` |
| `text("hello")` | `Document::create_text_node("hello")` |
| `insert(target, node, anchor)` | `Node::insert_before()` |
| `append(parent, child)` | `Node::append_child()` |
| `set_data(node, value)` | `TextNode::set_data()` |
| `listen(node, event, handler)` | `EventTarget::add_event_listener()` |

### 实现建议

```rust
// 在 jrust-runtime 中添加 Svelte 运行时绑定
pub fn register_svelte_bindings(registry: &mut BindingRegistry) {
    registry.register("element", |args| {
        let tag_name = args[0].as_string().unwrap();
        JsValue::Object(document.create_element(&tag_name))
    });
    
    registry.register("text", |args| {
        let content = args[0].as_string().unwrap();
        JsValue::Object(document.create_text_node(&content))
    });
    
    registry.register("listen", |args| {
        // 事件监听绑定
    });
}
```

---

## 三、React 支持 (⚠️ 需要运行时)

### 技术方案

```
React JSX → SWC/Babel 转换 → React.createElement() 调用
```

### 关键挑战

1. **React 运行时**: 需要实现 `React.createElement`, `React.Component` 等
2. **虚拟 DOM**: React 的 diff 算法需要实现
3. **Hooks**: `useState`, `useEffect` 等需要运行时支持

### JSX 转换

**输入**:
```jsx
function App() {
  const [count, setCount] = useState(0);
  return (
    <div>
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
  return React.createElement("div", null,
    React.createElement("button", {
      onClick: () => setCount(count + 1)
    }, "Clicked ", count, " times")
  );
}
```

### jrust-runtime 实现方案

#### 方案 A: 完整 React 运行时实现

```rust
// 实现 React.createElement
pub fn react_create_element(args: Vec<JsValue>) -> JsValue {
    let tag = args[0].as_string().unwrap();
    let props = args[1].as_object().unwrap();
    let children = &args[2..];
    
    let element = Document::create_element(&tag);
    
    // 设置属性
    for (key, value) in props.properties.iter() {
        element.set_attribute(key, &value.to_string());
    }
    
    // 添加子元素
    for child in children {
        element.append_child(&child);
    }
    
    JsValue::Object(element)
}
```

#### 方案 B: 转换为原生 DOM 操作

在 jrust-translator 阶段识别 React 模式，直接生成原生 DOM 操作：

```rust
// React.createElement("div", props, children)
// ↓ 转换为
// document.createElement("div")
// element.setAttribute(...)
// element.appendChild(...)
```

### React 运行时 API 清单

| API | 实现难度 | 说明 |
|-----|---------|------|
| `React.createElement` | 简单 | 创建虚拟 DOM 节点 |
| `React.Component` | 中等 | 类组件基类 |
| `useState` | 中等 | 状态 Hook |
| `useEffect` | 复杂 | 副作用 Hook |
| `useContext` | 中等 | 上下文 Hook |
| `useRef` | 简单 | 引用 Hook |
| React Scheduler | 复杂 | 并发渲染调度 |

### 建议实现路径

1. **Phase 1**: 支持 `React.createElement` 转换为原生 DOM
2. **Phase 2**: 支持 `useState` / `useRef`
3. **Phase 3**: 支持 `useEffect`
4. **Phase 4**: 支持 React 18 并发特性

---

## 四、Preact 支持 (✅ 易支持)

### 技术方案

```
Preact JSX → SWC 转换 → h() 函数调用
```

### Preact 优势

1. **极小体积**: ~3KB
2. **更接近原生**: 轻少的抽象层
3. **与 React API 兼容**: 迁移成本低

### h() 函数实现

```rust
pub fn preact_h(args: Vec<JsValue>) -> JsValue {
    let tag = args[0].as_string().unwrap();
    let props = args[1].as_object();
    let children = &args[2..];
    
    // 直接创建 DOM 元素
    let element = match tag.as_str() {
        "div" | "span" | "button" | ... => {
            Document::create_element(&tag)
        }
        _ => {
            // 自定义组件
            Component::create(&tag, props, children)
        }
    };
    
    // 设置属性和子元素
    // ...
    
    JsValue::Object(element)
}
```

---

## 五、SolidJS 支持 (✅ 易支持)

### 技术方案

```
Solid JSX → Solid 编译器 → 细粒度响应式 DOM
```

### Solid 优势

1. **无虚拟 DOM**: 直接 DOM 更新
2. **细粒度响应式**: 高性能
3. **编译时优化**: 类似 Svelte

### 编译示例

**输入**:
```jsx
import { createSignal } from "solid-js";

function Counter() {
  const [count, setCount] = createSignal(0);
  return (
    <button onClick={() => setCount(count() + 1)}>
      Clicked {count()} times
    </button>
  );
}
```

**编译后**:
```javascript
function Counter() {
  const [count, setCount] = createSignal(0);
  
  const _el$ = document.createElement("button");
  _el$.addEventListener("click", () => setCount(count() + 1));
  
  // 响应式绑定
  createEffect(() => {
    _el$.textContent = `Clicked ${count()} times`;
  });
  
  return _el$;
}
```

### jrust-runtime 映射

| Solid 函数 | jrust-runtime |
|-----------|---------------|
| `createSignal` | `Signal::new()` |
| `createEffect` | `Effect::new()` |
| `createMemo` | `Memo::new()` |

---

## 六、Angular 支持 (⚠️ 复杂)

### 技术方案

```
Angular 组件 → ngc AOT 编译 → 工厂函数
```

### 关键挑战

1. **Angular 运行时**: 依赖较重 (~30KB)
2. **Ivy 渲染引擎**: 复杂的编译产物
3. **依赖注入**: 需要完整实现

### AOT 编译示例

**输入**:
```typescript
@Component({
  selector: 'app-counter',
  template: '<button (click)="increment()">{{count}}</button>'
})
export class CounterComponent {
  count = 0;
  increment() { this.count++; }
}
```

**AOT 编译后**:
```javascript
function CounterComponent_Factory(t) {
  return new (t || CounterComponent)();
}

function CounterComponent_Template(rf, ctx) {
  if (rf & 1) {
    // 创建阶段
    elementStart(0, "button");
    listener("click", CounterComponent_click_handler);
    text(1);
    elementEnd();
  }
  if (rf & 2) {
    // 更新阶段
    textBinding(1, interpolation1("", ctx.count, ""));
  }
}
```

### 建议策略

1. **短期**: 不支持 Angular，建议用户使用 Vue/React
2. **中期**: 支持 Angular Elements (Web Components)
3. **长期**: 完整 Angular 运行时实现

---

## 七、其他框架

### Qwik

- **特点**: 可恢复性，延迟加载
- **挑战**: 特殊的序列化机制
- **优先级**: P2

### Lit

- **特点**: Web Components
- **优势**: 标准 API，易于支持
- **优先级**: P1

### Alpine.js

- **特点**: 轻量级，HTML 中直接使用
- **优势**: 无构建步骤
- **优先级**: P2

---

## 实现优先级

### P0 - 已完成 ✅

1. **Vue 3**: ✅ 已支持 - 完整预编译方案
2. **Svelte**: ✅ 已实现 - `bindings/svelte.rs`
3. **Preact**: ✅ 已实现 - `bindings/preact.rs`
4. **SolidJS**: ✅ 已实现 - `bindings/solid.rs`
5. **框架检测**: ✅ 已实现 - `translator/framework.rs`

### P1 - 近期支持

1. **React**: 需要完整运行时实现
2. **Lit**: Web Components

### P2 - 远期支持

1. **Angular**: 复杂度最高
2. **Qwik**: 特殊机制

---

## 统一处理流程

无论哪个框架，最终流程相同：

```
框架源码
    ↓
框架编译器/打包工具
    ↓
标准 JavaScript (无框架特性)
    ↓
jrust-translator (统一处理)
    ↓
Rust 代码
    ↓
jrust-runtime
    ↓
原生二进制
```

---

## 框架检测与适配

在 jrust-translator 中添加框架检测：

```rust
pub enum Framework {
    Vue,
    Svelte,
    React,
    Preact,
    Solid,
    Angular,
    Vanilla,
}

impl Framework {
    pub fn detect(js_code: &str) -> Self {
        if js_code.contains("createElementBlock") || js_code.contains("createVNode") {
            Framework::Vue
        } else if js_code.contains("create_fragment") {
            Framework::Svelte
        } else if js_code.contains("React.createElement") {
            Framework::React
        } else if js_code.contains("h(") && js_code.contains("preact") {
            Framework::Preact
        } else {
            Framework::Vanilla
        }
    }
    
    pub fn get_runtime_bindings(&self) -> Vec<&'static str> {
        match self {
            Framework::Vue => vec!["createElementBlock", "createBaseVNode", "toDisplayString"],
            Framework::Svelte => vec!["element", "text", "insert", "listen"],
            Framework::React => vec!["createElement", "useState", "useEffect"],
            // ...
        }
    }
}
```

---

## 总结

### 已完成框架支持

| 框架 | 实现位置 | 测试状态 |
|------|---------|---------|
| Vue 3 | 预编译方案 | ✅ 已验证 |
| Svelte | `bindings/svelte.rs` | ✅ 测试通过 |
| Preact | `bindings/preact.rs` | ✅ 测试通过 |
| SolidJS | `bindings/solid.rs` | ✅ 测试通过 |

### 核心实现

1. **框架检测**: `jrust-translator/src/framework.rs` - 自动检测编译产物所属框架
2. **运行时绑定**: `jrust-runtime/src/bindings/` - 各框架 API 绑定
3. **统一入口**: `register_all_framework_bindings()` - 一键注册所有绑定

### 使用方式

```rust
use jrust_runtime::bindings::{BindingRegistry, register_all_framework_bindings};

let mut registry = BindingRegistry::new();
register_all_framework_bindings(&mut registry);

// 自动检测框架
let result = jrust_translator::detect_framework(js_code);
println!("Detected: {}", result.primary.name());
```

### 下一步

1. **React 完整运行时**: 实现 Hooks、虚拟 DOM
2. **Lit 支持**: Web Components 标准实现
3. **性能优化**: 各框架特定优化路径
