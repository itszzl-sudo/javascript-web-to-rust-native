# JavaScript-Web-to-Rust-Native 架构方案

## 1. 项目概述

JavaScript-Web-to-Rust-Native 是一个创新的技术方案，旨在将任意打包后的 JavaScript 代码编译为原生 Rust 二进制程序，通过 Servo 渲染引擎实现高性能、内存安全的跨平台应用。

### 1.1. 核心哲学

> **"浏览器不关心你的源码是什么框架，只关心最终能否执行 JavaScript"**

| 浏览器关心的 | 浏览器不关心的 |
|-------------|----------------|
| 最终的 JavaScript 代码 | 源码框架 (任意前端框架) |
| DOM/BOM API 调用 | 打包工具 (Vite/Webpack/Rollup) |
| 事件系统 | 模块系统 (ESM/CJS/UMD) |
| 运行时语义 | JSX/TS/模板语法 |

### 1.2. 核心目标

| 目标 | 描述 |
|------|------|
| **通用性** | 支持所有打包后的 JavaScript 代码 |
| **高性能** | 利用 Rust 的零成本抽象和 Servo 的并行渲染 |
| **内存安全** | Rust 的所有权系统消除内存安全漏洞 |
| **跨平台** | 支持 Windows、macOS、Linux、iOS、Android |

### 1.3. 技术栈

| 组件 | 技术 | 版本 |
|------|------|------|
| JavaScript 解析 | Okapi | 最新 |
| 转译器 | javascript2rust (自研) | 0.1 |
| 渲染引擎 | Servo | 最新 |
| 语言 | Rust | 1.95+ |
| 构建工具 | 任意 (Vite/Webpack/Rollup/esbuild) | - |

---

## 2. 核心架构

### 2.1. 整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          任意前端项目                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │ Vue + Vite │  │React+Vite  │  │Svelte+Rollup│  │  任意组合   │   │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
└─────────┼────────────────┼────────────────┼────────────────┼──────────┘
          │                │                │                │
          ▼                ▼                ▼                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     任意打包工具                                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │    Vite    │  │   Webpack   │  │   Rollup    │  │  esbuild   │   │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
└─────────┼────────────────┼────────────────┼────────────────┼──────────┘
          │                │                │                │
          ▼                ▼                ▼                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     打包产物 (统一入口)                                  │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                     标准 JavaScript (ESM/IIFE/CJS)                 │   │
│  │   - ESTree 兼容 AST                                               │   │
│  │   - 标准 JavaScript 语法                                           │   │
│  │   - DOM/BOM API 调用                                              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     javascript2rust 转译器                                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │ JavaScript  │  │  语义分析   │  │   Rust      │  │   运行时    │   │
│  │  解析       │→ │  & 类型推断 │→ │  代码生成   │  │   绑定      │   │
│  │ (Okapi)     │  │             │  │             │  │  (Servo)    │   │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     Rust 代码 + 运行时库                                 │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  转译后的 Rust 代码 (调用方)                                     │   │
│  │  +                                                             │   │
│  │  javascript-web-runtime (被调用方 - Servo 绑定层)               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     Cargo 构建                                           │
│  编译为原生二进制可执行文件                                              │
└───────────────────────────┬─────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     原生二进制                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │   Windows   │  │    macOS    │  │   Linux    │  │  Mobile    │   │
│  │   .exe      │  │   .app      │  │  ELF       │  │   .apk     │   │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2. 核心原则

#### 原则一：不关心源码框架

```
输入: 任意前端框架 (Vue/React/Svelte/其他)
         │
         ▼ (打包工具)
输出: 标准 JavaScript 函数调用
         │
处理: 统一的 JavaScript 语法
```

#### 原则二：只处理打包后的代码

```
打包产物示例:

// 任意框架打包后都是标准 JavaScript
import { createApp } from 'vendor'
createApp(App).mount('#app')

// 或者
var React = require('react')
React.createElement(App, null)

// 或者
function mount_component(target) { ... }
```

这些都是标准 JavaScript，我们的转译器统一处理。

---

## 3. 核心组件

### 3.1. javascript2rust 转译器

**职责**：将打包后的 JavaScript 转译为 Rust 代码

```
javascript2rust
├── frontend/          # JavaScript 前端
│   ├── mod.rs
│   ├── parser.rs         # 基于 Okapi 的 JS 解析
│   ├── lexer.rs          # 词法分析
│   ├── validator.rs      # 语法验证
│   └── estree.rs         # ESTree AST 定义
│
├── analysis/         # 语义分析
│   ├── mod.rs
│   ├── scope.rs          # 作用域分析
│   ├── typeinfer.rs      # 类型推断
│   └── callgraph.rs      # 调用图分析
│
├── transforms/      # 转换模块
│   ├── mod.rs
│   ├── esm_to_common.rs  # ESM → CommonJS 统一
│   ├── async_transform.rs # async/await 转换
│   └── class_transform.rs # class 转换
│
├── codegen/         # 代码生成
│   ├── mod.rs
│   ├── rust_gen.rs       # Rust 代码生成
│   ├── type_mapping.rs   # JS → Rust 类型映射
│   └── intrinsics.rs     # 内联函数处理
│
└── cli/             # 命令行接口
    └── main.rs
```

### 3.2. javascript-web-runtime (Servo 绑定层)

**职责**：提供 JavaScript 运行时的 Rust 实现

```
javascript-web-runtime
├── core/            # 核心运行时
│   ├── mod.rs
│   ├── value.rs         # JavaScript 值类型
│   ├── object.rs        # 对象系统
│   ├── array.rs         # 数组实现
│   ├── function.rs      # 函数实现
│   └── string.rs        # 字符串实现
│
├── dom/             # DOM API
│   ├── mod.rs
│   ├── document.rs      # Document API
│   ├── element.rs       # Element API
│   ├── node.rs          # Node API
│   └── events.rs        # 事件系统
│
├── bom/             # BOM API
│   ├── mod.rs
│   ├── window.rs        # Window API
│   ├── location.rs      # Location API
│   ├── navigator.rs     # Navigator API
│   ├── fetch.rs         # Fetch API
│   └── storage.rs       # Storage API
│
├── gc/              # 垃圾回收
│   ├── mod.rs
│   ├── tracer.rs        # 引用追踪
│   └── collector.rs     # 回收器
│
└── utils/           # 工具函数
    ├── mod.rs
    └── conversions.rs   # 类型转换
```

### 3.3. 类型映射

| JavaScript 类型 | Rust 类型 |
|-----------------|-----------|
| `number` | `f64` / `i32` |
| `string` | `String` |
| `boolean` | `bool` |
| `undefined` | `()` / `Option<T>` |
| `null` | `Option<T>` |
| `object` | `Rc<RefCell<Object>>` |
| `array` | `Rc<RefCell<Vec<Value>>>` |
| `function` | `Rc<Function>` |
| `Promise` | `tokio::spawn` / `async fn` |
| `Symbol` | `Rc<Symbol>` |

---

## 4. 数据流设计

### 4.1. 编译时数据流

```
打包产物 (标准 JavaScript)
         │
         ▼
    JavaScript 解析 → ESTree AST
         │
         ▼
    语义分析 → 类型信息 + 调用图
         │
         ▼
    JavaScript AST → Rust AST
         │
         ▼
    Rust 代码生成
         │
         ▼
    Cargo 编译 → 原生二进制
```

### 4.2. 运行时数据流

```
用户交互
    │
    ▼
事件系统 → 响应式更新 → DOM 变更 → Servo 渲染 → 屏幕呈现
```

---

## 5. 安全性设计

### 5.1. 内存安全

| JavaScript 特性 | Rust 实现 | 安全保障 |
|----------------|-----------|----------|
| 动态类型 | `Value` 枚举 | 编译期类型检查 |
| 原型链 | `Rc<Prototype>` | 借用检查 |
| 闭包 | `Rc<Closure>` | 所有权明确 |
| 垃圾回收 | 增量 GC | 内存安全 |
| 空引用 | `Option<T>` | 编译期保证 |

### 5.2. 沙箱机制

```
┌──────────────────────────────────────────┐
│              主进程 (UI)                  │
│  ┌────────────────────────────────────┐  │
│  │  egui / Native UI                  │  │
│  └────────────────────────────────────┘  │
└───────────────────────────┬──────────────┘
                            │ IPC
                            ▼
┌──────────────────────────────────────────┐
│              渲染进程 (Servo)            │
│  ┌────────────────────────────────────┐  │
│  │  转译后的 JS 应用                   │  │
│  │  + javascript-web-runtime          │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
```

---

## 6. 性能优化策略

### 6.1. 编译期优化

| 优化项 | 说明 |
|--------|------|
| 死代码消除 | Cargo LTO |
| 内联展开 | Rust 编译器 |
| 常量传播 | 编译期计算 |
| 跨语言优化 | link-time optimization |

### 6.2. 运行时优化

| 优化项 | 说明 |
|--------|------|
| 增量 GC | 避免全量回收 |
| 虚函数优化 | 内联缓存 |
| 数组优化 | 类型化数组 |
| 字符串优化 | 字符串池 |

### 6.3. Servo 优化

| 优化项 | 说明 |
|--------|------|
| 并行渲染 | Servo 原生 |
| WebRender | GPU 加速 |
| 组件化渲染 | 独立层 |

---

## 7. 支持的场景

### 7.1. 打包工具

| 工具 | 状态 | 说明 |
|------|------|------|
| Vite | ✅ 支持 | ESM 输出 |
| Webpack | ✅ 支持 | 多种输出模式 |
| Rollup | ✅ 支持 | ESM/IIFE/CJS |
| esbuild | ✅ 支持 | 快速构建 |
| Parcel | ⏳ 规划 | 需测试 |

### 7.2. 模块系统

| 系统 | 状态 | 说明 |
|------|------|------|
| ES Modules | ✅ 支持 | 主要目标 |
| CommonJS | ✅ 支持 | require 转换 |
| IIFE | ✅ 支持 | 立即执行 |
| UMD | ✅ 支持 | 兼容模式 |

---

## 8. 项目结构

```
javascript-web-to-rust-native/
├── docs/                  # 文档
│   ├── ARCHITECTURE.md        # 架构说明 (本文件)
│   ├── ROADMAP.md             # 实现路线图
│   └── API.md                 # API 文档
│
├── src/                   # 源代码
│   ├── javascript2rust/         # JavaScript → Rust 转译器
│   │   ├── frontend/              # JS 前端
│   │   ├── analysis/              # 语义分析
│   │   ├── transforms/            # 转换模块
│   │   └── codegen/               # 代码生成
│   │
│   └── javascript-web-runtime/ # JavaScript 运行时 (基于 Servo)
│       ├── core/                    # 核心运行时
│       ├── dom/                     # DOM API
│       ├── bom/                     # BOM API
│       ├── gc/                      # 垃圾回收
│       └── utils/                   # 工具
│
├── tests/                 # 测试
│   ├── unit/                  # 单元测试
│   ├── integration/            # 集成测试
│   └── e2e/                   # E2E 测试
│
├── examples/              # 示例
    └── vanilla-js/            # 原生 JavaScript 示例
│
├── Cargo.toml             # Rust 项目配置
├── package.json          # Node.js 项目配置
└── README.md             # 项目介绍
```

---

## 9. 技术选型对比

| 方案 | 性能 | 安全性 | 开发体验 | 生态成熟度 | 通用性 |
|------|------|--------|----------|------------|--------|
| JS-Web-to-Rust-Native | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| Tauri | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| Electron | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| Flutter | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |

**核心优势**：通用性 + 性能 + 安全性的平衡

---

## 10. jrust 与 runtime 架构详解

### 10.1. 核心角色分工

| 角色 | 职责 | 特点 |
|------|------|------|
| **director** | 将 JavaScript 代码翻译为 jrust1，管理 jrust 树 | 控制和调度中心 |
| **jrust1** | 主 jrust 实例，负责 DOM 初始化，捕获大多数事件 | 根实例 |
| **jrust2、jrust3...** | 由 jrust1 创建，处理动态部署的任务 | 子实例 |
| **javascript-web-runtime** (runtime) | 维护 DOM、提供事件源、执行 jrust 的更新指令 | 渲染和事件层 |

### 10.2. 完整工作流

#### 阶段 1: 初始化 (jrust 主动)

```
jrust 启动 → 执行入口代码 → 构造完整的 DOM 结构 → 绑定事件处理函数 → 静默等待
```

**jrust 职责**：
- 构建初始 UI
- 设置业务逻辑
- 绑定事件监听器

#### 阶段 2: 事件触发 (runtime 主动)

```
事件源 (时间/网络/地理位置/点击/触摸/键盘) → runtime 检测事件 → 加入事件队列
```

**runtime 职责**：
- 监听所有外部事件
- 维护事件队列
- 按顺序处理事件

#### 阶段 3: 事件通知 (runtime → jrust)

```
runtime 从队列取出事件 → 调用 jrust 对应的事件处理函数 → 传递事件信息
```

#### 阶段 4: 事件响应 (jrust 主动)

```
jrust 执行事件处理逻辑 → 分析业务需求 → 修改 DOM → 调用 runtime API
```

**jrust 职责**：
- 处理业务逻辑
- 决定 DOM 如何修改
- 通过 runtime API 发送更新指令

#### 阶段 5: 更新执行 (runtime 主动)

```
runtime 执行 jrust 的更新指令 → 更新 DOM 树 → 重新渲染 → 用户可见
```

**循环**：回到阶段 2，等待下一个事件

### 10.3. 事件系统串联机制

所有事件都可以在 runtime 中串联处理，不影响业务逻辑：

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Event Loop (runtime 维护)                                               │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ Event Queue (所有事件按顺序排列)                                  │   │
│  ├─────────────────────────────────────────────────────────────────┤   │
│  │ [TimerEvent] → [NetworkEvent] → [TouchEvent] → [KeyEvent] → ...  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ↓                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ 事件分发器 (逐个发送给对应的 jrust 事件处理函数)                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ↓                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ jrust 业务逻辑 (对顺序不敏感，按事件逻辑处理)                     │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 10.4. 动态代码执行架构

jrust 支持动态代码执行和多模块交互：

```
┌─────────────────────────────────────────────────────────────────────────┐
│  动态代码执行场景                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ 1. jrust 调用 eval('...')                                         │   │
│  │ 2. jrust 在 DOM 上动态添加事件处理逻辑                               │   │
│  │ 3. 多个模块相互调用                                                 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ↓                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ runtime 检测到动态代码需求 → 实例化新的 jrust 实例 → 处理动态代码 │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ↓                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ 多模块交互：jrust-1 → runtime → jrust-2 → runtime → jrust-1      │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 10.5. jrust 与 runtime 接口设计

```rust
// ================== jrust 与 runtime 的核心接口 ==================

// jrust 调用 runtime 的 API
pub trait RuntimeAPI {
    // DOM 操作
    fn create_element(&mut self, tag: &str) -> ElementId;
    fn set_attribute(&mut self, element: ElementId, name: &str, value: &str);
    fn append_child(&mut self, parent: ElementId, child: ElementId);
    
    // 事件绑定
    fn add_event_listener(&mut self, element: ElementId, event_type: EventType, handler: EventHandler);
    
    // 其他 API
    fn console_log(&self, message: &str);
    fn set_timeout(&self, callback: TimeoutCallback, delay_ms: u32);
    fn fetch(&self, url: &str) -> Promise;
}

// runtime 调用 jrust 的接口
pub trait JsRustApp {
    // 初始化
    fn init(&mut self, runtime: &mut impl RuntimeAPI);
    
    // 事件处理
    fn handle_event(&mut self, event: &Event) -> UpdateCommand;
}
```

### 10.6. 线程 vs 进程架构选择

| 架构 | 性能 | 安全性 | 开发复杂度 | 隔离性 | 推荐场景 |
|------|------|--------|------------|--------|----------|
| **双线程 + 队列** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | 低 | 弱 | 开发阶段，单一应用 |
| **双进程 + IPC** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 高 | 强 | 生产阶段，安全性要求高 |
| **混合架构** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 中 | 中 | 长期方案，平衡性能与安全 |

**推荐方案**：短期采用双线程架构快速验证，长期采用混合架构实现最佳平衡。

---

## 11. Director 与 jrust 树架构

### 11.1. 核心架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Director (指挥中心)                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ 1. 将初始 JavaScript 翻译为 jrust1                               │   │
│  │ 2. 接收动态 JavaScript 代码 (来自 jrust1)                        │   │
│  │ 3. 翻译为新的 jrust2、jrust3...                                  │   │
│  │ 4. 管理整个 jrust 树的创建和销毁                                │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        jrust 树结构                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    jrust1 (根实例)                               │   │
│  │  - DOM 初始化                                                     │   │
│  │  - 捕获大多数事件                                                 │   │
│  │  - 部署新的 JavaScript 监听任务 → 通知 director                  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                           │                                              │
│           ┌───────────────┴─────────────────┐                          │
│           ▼                               ▼                          │
│  ┌─────────────────┐            ┌─────────────────┐                   │
│  │  jrust2 (子实例) │            │  jrust3 (子实例) │                   │
│  │  - 动态部署的   │            │  - 动态部署的   │                   │
│  │    任务         │            │    任务         │                   │
│  └─────────────────┘            └─────────────────┘                   │
│       │                                  │                            │
│       └─ 还可创建 jrust4、jrust5...        └─ 还可创建更多子实例        │
└─────────────────────────────────────────────────────────────────────────┘
```

### 11.2. 完整工作流 - 包含 director

#### 阶段 1: 初始化 (director 主动)

```
director 启动 → 接收初始 JavaScript → 翻译为 jrust1 → jrust1 初始化 DOM → 准备就绪
```

#### 阶段 2: 动态代码部署 (jrust1 主动)

```
jrust1 在 DOM 上部署新的 JavaScript 监听任务 → 通知 director → director 翻译为 jrust2/jrust3...
```

#### 阶段 3: 事件捕获 (runtime 主动)

```
runtime 捕获事件 → 进入事件队列
```

#### 阶段 4: 事件传播 (核心流程)

```
runtime 开始事件轮询 → 按顺序通知 jrust 树

第一步: 通知 jrust1
   ↓
   (jrust1 可以选择: 处理并阻止传播 / 处理并继续传播 / 不处理直接传播)
   ↓
第二步: 如果传播继续，同时通知 jrust2、jrust3... (jrust1 创建的子实例)
   ↓
   (jrust2、jrust3 可以选择: 处理并阻止传播 / 处理并继续传播 / 不处理直接传播)
   ↓
第三步: 继续深入子树...
```

### 11.3. 事件传播算法

```
// 伪代码描述事件传播流程
function dispatch_event(event, root_jrust) {
    let stop_propagation = false;
    
    // 阶段 1: 从根开始
    stop_propagation = root_jrust.handle_event(event);
    
    if (!stop_propagation && root_jrust.has_children()) {
        // 阶段 2: 同时通知所有子 jrust
        for child in root_jrust.get_children() {
            if (!stop_propagation) {
                stop_propagation = child.handle_event(event);
            }
            // 注意：同一层的 jrust 都能收到事件，即使前面的阻止了
        }
        
        // 阶段 3: 继续向下传播到更深层
        for child in root_jrust.get_children() {
            if (!stop_propagation) {
                stop_propagation = dispatch_event(event, child);
            }
        }
    }
    
    return stop_propagation;
}
```

### 11.4. Director 接口设计

```rust
pub trait Director {
    // 初始化阶段：创建根 jrust
    fn create_root_jrust(&mut self, javascript_code: &str) -> JsRustId;
    
    // 动态部署阶段：创建新的子 jrust
    fn create_child_jrust(&mut self, parent_id: JsRustId, javascript_code: &str) -> JsRustId;
    
    // 销毁 jrust
    fn destroy_jrust(&mut self, jrust_id: JsRustId);
    
    // 事件分发
    fn dispatch_event(&mut self, event: Event);
}

// jrust 与 director 通信的接口
pub trait JsRustInstance {
    // 初始化
    fn init(&mut self, runtime: &mut impl RuntimeAPI);
    
    // 处理事件，返回是否阻止传播
    fn handle_event(&mut self, event: &Event) -> bool;
    
    // 部署新的 JavaScript 任务
    fn deploy_javascript_task(&mut self, js_code: &str);
    
    // 获取子实例列表
    fn get_children(&self) -> Vec<JsRustId>;
}
```

### 11.5. 事件传播示例

场景：点击按钮，jrust1 阻止部分传播，jrust2、jrust3 响应

```
事件: ClickEvent
  ↓
jrust1 处理 (更新按钮文字) → 选择: 不阻止传播
  ↓
jrust2 处理 (记录日志) → 选择: 不阻止传播
jrust3 处理 (播放动画) → 选择: 不阻止传播
  ↓
jrust2 的子 jrust4 处理 (发送网络请求)
jrust3 的子 jrust5 处理 (更新本地存储)
  ↓
事件处理完成
```

### 11.6. 事件阻止示例

场景：jrust1 阻止事件传播

```
事件: KeyPressEvent
  ↓
jrust1 处理 (处理键盘快捷键) → 选择: 阻止传播
  ↓
jrust2、jrust3、jrust4、jrust5... 都收不到事件
  ↓
事件处理完成 (仅 jrust1 响应)
```