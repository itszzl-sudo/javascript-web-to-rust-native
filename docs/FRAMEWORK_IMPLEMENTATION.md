# 框架支持实现记录

**更新日期**: 2026-05-18

## 已完成框架

### Vue 3 ✅

**实现方式**: 预编译方案

**文档**: `VUE_PRECOMPILE.md`

**关键特性**:
- 模板预编译为 render 函数
- 无 `eval` / `new Function`
- 支持静态提升和 PatchFlag 优化

**编译产物示例**:
```javascript
createElementBlock("div", { id: "app" }, [
  createBaseVNode("h1", null, toDisplayString(title.value), 1)
], 2)
```

---

### Svelte ✅

**实现位置**: `src/jrust-runtime/src/bindings/svelte.rs`

**绑定函数** (14 个):

| 函数 | 说明 |
|------|------|
| `svelte_element` | 创建元素节点 |
| `svelte_text` | 创建文本节点 |
| `svelte_space` | 创建空格节点 |
| `svelte_empty` | 创建空节点 |
| `svelte_insert` | 插入节点 |
| `svelte_append` | 追加子节点 |
| `svelte_detach` | 移除节点 |
| `svelte_attr` | 设置属性 |
| `svelte_set_data` | 设置文本内容 |
| `svelte_set_style` | 设置样式 |
| `svelte_add_class` | 添加类 |
| `svelte_remove_class` | 移除类 |
| `svelte_listen` | 添加事件监听 |
| `svelte_binding` | 响应式绑定 |

**Svelte 编译产物**:
```javascript
function create_fragment(ctx) {
  let button;
  function c() { button = element("button"); }
  function m(target, anchor) { insert(target, button, anchor); }
  function p(ctx, dirty) { if (dirty & 1) set_data(t1, ctx[0]); }
  function d(detaching) { if (detaching) detach(button); }
  return { c, m, p, d };
}
```

---

### Preact ✅

**实现位置**: `src/jrust-runtime/src/bindings/preact.rs`

**绑定函数** (7 个):

| 函数 | 说明 |
|------|------|
| `preact_h` | hyperscript 函数 (创建元素) |
| `preact_fragment` | Fragment 支持 |
| `preact_createElement` | createElement 别名 |
| `preact_render` | 渲染到容器 |
| `preact_toChildArray` | 子元素数组转换 |
| `preact_options` | 选项钩子 |

**Preact 编译产物**:
```javascript
h("div", { className: "container" },
  h("button", { onClick: () => setCount(count + 1) }, "Click")
)
```

---

### SolidJS ✅

**实现位置**: `src/jrust-runtime/src/bindings/solid.rs`

**绑定函数** (15 个):

| 函数 | 说明 |
|------|------|
| `solid_createSignal` | 创建响应式信号 |
| `solid_createEffect` | 创建副作用 |
| `solid_createMemo` | 创建计算值 |
| `solid_createRenderEffect` | 创建渲染副作用 |
| `solid_createComputed` | 创建计算属性 |
| `solid_onMount` | 挂载回调 |
| `solid_onCleanup` | 清理回调 |
| `solid_untrack` | 取消追踪 |
| `solid_batch` | 批量更新 |
| `solid_template` | 模板创建 |
| `solid_insert` | 插入 DOM |
| `solid_spread` | 属性展开 |
| `solid_dynamic` | 动态组件 |
| `solid_suspense` | Suspense 支持 |
| `solid_portal` | Portal 支持 |

**SolidJS 编译产物**:
```javascript
const [count, setCount] = createSignal(0);
createEffect(() => console.log(count()));

const _el$ = document.createElement("button");
_el$.addEventListener("click", () => setCount(count() + 1));
createEffect(() => _el$.textContent = `Clicked ${count()} times`);
```

---

## 框架检测 ✅

**实现位置**: `src/jrust-translator/src/framework.rs`

**支持的框架检测**:
- Vue 3 (检测 `createElementBlock`, `createVNode`)
- Svelte (检测 `create_fragment`, `svelte_element`)
- Preact (检测 `h(`, `preact`)
- SolidJS (检测 `createSignal`, `createEffect`)
- React (检测 `React.createElement`, `useState`)
- Angular (检测 `ɵɵelementStart`, `ɵɵtext`)

**使用方式**:
```rust
use jrust_translator::{detect_framework, Framework};

let result = detect_framework(js_code);
println!("Primary: {}", result.primary.name());
println!("Confidence: {}", result.confidence);
```

---

## 统一绑定注册

```rust
use jrust_runtime::bindings::{BindingRegistry, register_all_framework_bindings};

let mut registry = BindingRegistry::new();
register_all_framework_bindings(&mut registry);

// 一次性注册所有框架绑定
// - DOM 绑定
// - Svelte 绑定
// - Preact 绑定
// - SolidJS 绑定
```

---

## 测试覆盖

**测试文件**: `src/jrust-runtime/tests/framework_bindings_test.rs`

**测试内容**:
- Svelte 绑定测试
- Preact 绑定测试
- SolidJS 绑定测试
- 框架检测测试

**运行测试**:
```bash
cargo test -p jrust-runtime --test framework_bindings_test
cargo test -p jrust-translator --lib framework
```

---

## 下一步：React

### 需要实现的功能

1. **createElement** - 创建虚拟 DOM 节点
2. **useState** - 状态 Hook
3. **useEffect** - 副作用 Hook
4. **useRef** - 引用 Hook
5. **useMemo** / **useCallback** - 缓存 Hook
6. **useContext** - 上下文 Hook

### React 编译产物

**JSX 转换后**:
```javascript
function Counter() {
  const [count, setCount] = useState(0);
  return React.createElement(
    "div",
    null,
    React.createElement("button", {
      onClick: () => setCount(count + 1)
    }, "Clicked ", count, " times")
  );
}
```

### 实现策略

**方案 B: 直接 DOM 操作** (推荐)
- 将 `React.createElement` 转换为直接 DOM 创建
- 避免虚拟 DOM diff 开销
- 性能更好

```rust
// React.createElement("div", props, children)
// ↓ 直接生成
// document.createElement("div")
// element.setAttribute(...)
// element.appendChild(...)
```
