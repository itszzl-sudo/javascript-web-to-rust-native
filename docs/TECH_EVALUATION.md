# 技术评估报告：DOM预编译与序列化方案

**日期**：2025-XX-XX
**状态**：初稿
**版本**：v0.1

---

## 1. 背景与目标

### 1.1 问题描述

当前项目面临以下技术挑战：
- **启动性能**：jrust1需要完整执行DOM初始化代码，导致启动时间较长
- **复杂动态代码**：部分框架（如React、Angular）的JS代码难以直接转译为Rust
- **生态兼容性**：需要支持Vue、React、Angular等多种前端框架

### 1.2 目标

1. 减少目标应用的启动时间
2. 扩大框架支持范围（Vue、React、Angular）
3. 保持架构的简洁性和可维护性

---

## 2. 框架预编译方案分析

### 2.1 Vue 预编译方案 ✅ 完全可行

**技术方案**：
- **vite-plugin-vue-precompile**：Vite官方插件，预售编译Vue SFC中的模板
- **Vue仅运行时模式**：使用vue.runtime.esm-bundler.js，模板需预编译

**实现效果**：
- ✅ 无 eval/new Function
- ✅ 可在 QuickJS 等轻量引擎运行
- ✅ 可在浏览器扩展 CSP 环境运行
- ✅ 减小打包体积

**结论**：Vue 3的预编译方案最成熟，推荐优先支持。

### 2.2 React JSX转换方案 ⚠️ 需进一步评估

**技术现状**：
- **swc已支持JSX转换**：swc的测试用例显示完整的React JSX解析和转换能力
- **配置项丰富**：
  ```json
  {
    "jsc": {
      "parser": {
        "syntax": "ecmascript",
        "jsx": true
      },
      "transform": {
        "react": {
          "runtime": "automatic",
          "development": true,
          "refresh": true
        }
      }
    }
  }
  ```

**关键问题**：
- React的JSX转换后仍需要React运行时（React.createElement）
- 需要评估Servo中SpiderMonkey是否能承担此任务
- 需要识别"初始化代码"与"事件监听代码"的边界

**结论**：swc具备JSX转换能力，但具体执行需要SpiderMonkey支持。

### 2.3 Angular AOT编译方案 ⚠️ 需进一步评估

**技术现状**：
- Angular的AOT（Ahead-of-Time）编译可以将模板预编译为JavaScript
- 编译后仍需要Angular运行时

**关键问题**：
- Angular运行时依赖较重
- Ivy渲染引擎的复杂性
- 需评估与Servo的兼容性

**结论**：Angular AOT编译可行，但需要评估运行时依赖。

### 2.4 其他框架

| 框架 | 预编译支持 | 运行时依赖 | 推荐优先级 |
|------|-----------|-----------|-----------|
| **Vue 3** | ✅ 完整支持 | 仅虚拟DOM.diff | P0 |
| **Svelte** | ✅ 最彻底 | 几乎无 | P0 |
| **React** | ⚠️ JSX转换 | 需要React运行时 | P1 |
| **Angular** | ⚠️ AOT编译 | 需要Angular运行时 | P2 |

---

## 3. Servo/SpiderMonkey能力评估

### 3.1 SpiderMonkey技术能力

SpiderMonkey是Mozilla的JavaScript引擎（Firefox的JS引擎），具备以下能力：

| 能力 | 支持情况 | 说明 |
|------|---------|------|
| JavaScript解析 | ✅ 完整支持 | ES2020+ |
| JIT编译 | ✅ IonMonkey | 高性能 |
| DOM API | ✅ 通过Servo绑定 | 完整的DOM操作 |
| 垃圾回收 | ✅ GC | 内存管理 |
| WebAssembly | ✅ 支持 | 可选 |

### 3.2 SpiderMonkey作为预执行引擎的可行性

**可行性分析**：

```rust
// SpiderMonkey可以完成的任务
pub fn spider_monkey_pre_execute(js_code: &str) -> Result<DomSnapshot> {
    // 1. 创建SpiderMonkey运行时
    // 2. 设置DOM环境（Window, Document等）
    // 3. 执行初始化JS代码
    // 4. 序列化DOM结构
    // 5. 返回序列化结果
}
```

**优点**：
- ✅ SpiderMonkey是成熟的JavaScript引擎，稳定可靠
- ✅ 可以正确执行所有JavaScript代码（包括React.createElement等）
- ✅ Servo已经集成了SpiderMonkey，可以复用
- ✅ 生态成熟，过程可观测（DevTools协议支持）

**缺点**：
- ❌ 需要完整的SpiderMonkey运行时（约1-2MB）
- ❌ 初始化时间长于纯Rust方案
- ❌ FFI调用开销

**结论**：SpiderMonkey可以承担预执行任务，特别是对于复杂框架（React、Angular）。

---

## 4. 初始化代码与事件监听代码分离方案

### 4.1 分离的可行性分析

**关键洞察**：Web应用的JavaScript代码通常分为两类：

1. **初始化代码**（Initialization Code）
   - 创建DOM结构
   - 设置初始状态
   - 执行时机：应用启动时一次性执行
   - 特点：同步执行，结果可序列化

2. **事件监听代码**（Event Listener Code）
   - 注册事件处理器
   - 处理用户交互
   - 执行时机：用户触发事件时执行
   - 特点：异步执行，难以序列化

**分离示例**：

```javascript
// 初始化代码（可预执行并序列化）
const root = document.getElementById('root');
root.innerHTML = '<div>Hello World</div>';
const button = document.createElement('button');
button.textContent = 'Click me';
root.appendChild(button);

// 事件监听代码（需要转译为Rust）
button.addEventListener('click', () => {
    console.log('Button clicked!');
});
```

### 4.2 分离方法

#### 方法一：基于代码分析（静态分离）

```rust
pub struct CodeSeparator {
    init_patterns: Vec<Regex>,
    event_patterns: Vec<Regex>,
}

impl CodeSeparator {
    pub fn separate(&self, js_code: &str) -> (String, String) {
        let mut init_code = String::new();
        let mut event_code = String::new();

        for stmt in parse_statements(js_code) {
            if self.is_initialization(&stmt) {
                init_code.push_str(&stmt);
            } else if self.is_event_listener(&stmt) {
                event_code.push_str(&stmt);
            }
        }

        (init_code, event_code)
    }

    fn is_initialization(&self, stmt: &Statement) -> bool {
        // 匹配: document.createElement, element.innerHTML =, etc.
        self.init_patterns.iter().any(|p| p.is_match(stmt))
    }

    fn is_event_listener(&self, stmt: &Statement) -> bool {
        // 匹配: addEventListener, onClick =, etc.
        self.event_patterns.iter().any(|p| p.is_match(stmt))
    }
}
```

#### 方法二：基于AST分析（精确分离）

```rust
pub fn separate_by_ast(js_code: &str) -> (String, String) {
    let ast = parse_ast(js_code);

    let init_stmts: Vec<Stmt> = ast.body.iter()
        .filter(|stmt| !is_event_registration(stmt))
        .collect();

    let event_stmts: Vec<Stmt> = ast.body.iter()
        .filter(|stmt| is_event_registration(stmt))
        .collect();

    (
        generate_code(&init_stmts),
        generate_code(&event_stmts),
    )
}

fn is_event_registration(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(ExprStmt { expr, .. }) => {
            matches!(expr, Expr::Call(CallExpr { callee, .. })
                if matches!(callee, Expr::Member(MemberExpr { prop, .. })
                    if prop == "addEventListener"))
        }
        _ => false
    }
}
```

### 4.3 分离边界判断规则

**初始化代码特征**：
- `document.createElement()`
- `element.innerHTML =`
- `element.setAttribute()`
- `element.appendChild()` / `insertBefore()`
- `document.createTextNode()`
- 直接的属性赋值（非回调）

**事件监听代码特征**：
- `addEventListener()`
- `onclick =` / `onfocus =` / etc.
- 回调函数定义
- `removeEventListener()`

**特殊情况**：
- `eval()` - 动态代码，需要特殊处理
- `new Function()` - 动态函数，需要特殊处理
- `setTimeout` / `setInterval` - 异步执行，需要保留

---

## 5. 方案A vs 方案B 对比分析

### 5.1 方案A：JS引擎 + 序列化

**架构图**：
```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Vue/Vite   │ ──▶ │  SpiderMonkey│ ──▶ │ 序列化DOM   │
│  (预编译)   │     │ (预执行JS)  │     │ (bincode)   │
└─────────────┘     └─────────────┘     └─────────────┘
                                              │
                                              ▼
                    ┌─────────────┐     ┌─────────────┐
                    │  反序列化   │ ◀── │   二进制    │
                    │  恢复DOM   │     │   文件      │
                    └─────────────┘     └─────────────┘
```

**优点**：
- ✅ 生态成熟，过程可观测（SpiderMonkey DevTools）
- ✅ 可以处理所有JavaScript代码（包括动态代码）
- ✅ 对框架友好（React、Angular等）
- ✅ 可以提前验证JS代码正确性

**缺点**：
- ❌ 需要完整的SpiderMonkey运行时（约1-2MB）
- ❌ FFI调用开销
- ❌ 增加二进制体积
- ❌ 初始化时间长于纯Rust方案

**适用场景**：
- React、Angular等复杂框架
- 需要支持动态代码（eval、new Function）
- 快速原型开发

### 5.2 方案B：jrust1预执行 + 序列化

**架构图**：
```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Vue/Vite   │ ──▶ │ jrust-trans │ ──▶ │  jrust1     │
│  (预编译)   │     │   lator     │     │ (Rust代码)  │
└─────────────┘     └─────────────┘     └─────────────┘
                                              │
                                              ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  反序列化   │ ◀── │   二进制    │ ◀── │ 执行初始化  │
│  恢复DOM   │     │   文件      │     │ 序列化DOM   │
└─────────────┘     └─────────────┘     └─────────────┘
```

**优点**：
- ✅ 架构更简洁，无JS引擎依赖
- ✅ 序列化和反序列化更简单（纯Rust结构）
- ✅ 性能更好（无FFI开销）
- ✅ 调试更容易（纯Rust堆栈）
- ✅ 二进制体积更小

**缺点**：
- ❌ jrust1需要完整实现DOM初始化能力
- ❌ 复杂动态代码处理困难
- ❌ 闭包和回调处理需要特殊设计

**适用场景**：
- Vue、Svelte等框架
- 简单的原生JavaScript应用
- 对二进制体积敏感的场景

### 5.3 方案对比矩阵

| 评估维度 | 方案A (SpiderMonkey) | 方案B (jrust1) |
|---------|---------------------|----------------|
| **生态成熟度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **过程可观测性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **性能** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **二进制体积** | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **框架支持** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **动态代码** | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **实现复杂度** | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **调试难度** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 6. 序列化工具选型分析

### 6.1 序列化工具对比

| 工具 | 序列化格式 | 性能 | 体积 | 跨语言 | Rust专属 |
|------|-----------|------|------|--------|---------|
| **rkyv** | 二进制 | ⭐⭐⭐⭐⭐ | 最小 | ❌ | ✅ 零拷贝 |
| **bincode** | 二进制 | ⭐⭐⭐ | 小 | ❌ | ✅ |
| **prost** | Protobuf | ⭐⭐⭐⭐ | 中 | ✅ | ❌ |
| **serde_json** | JSON | ⭐⭐ | 大 | ✅ | ❌ |
| **serde_toml** | TOML | ⭐⭐ | 中 | ❌ | ✅ 可读性好 |

### 6.2 推荐方案

**启动性能优先** → `rkyv`（零拷贝，极快）
**跨语言通信** → `prost`（Protobuf）
**简单场景/调试** → `bincode`
**配置文件** → `serde_toml`

### 6.3 DOM结构序列化分析

当前jrust-runtime的DOM结构：

```rust
// Element 结构的序列化分析
pub struct Element {
    pub tag_name: String,              // ✅ 直接序列化
    pub id: Option<String>,             // ✅ 直接序列化
    pub class_list: Vec<String>,       // ✅ 直接序列化
    pub attributes: HashMap<String, String>, // ✅ 可序列化
    pub children: Vec<Element>,        // ✅ 递归序列化
    pub text_content: String,           // ✅ 直接序列化
    pub inner_html: String,            // ✅ 直接序列化
    event_listeners: HashMap<EventType, Vec<Box<dyn Fn(&Event) -> JsValue>>>, // ❌ 闭包无法序列化
}
```

**关键问题**：事件监听器包含`Box<dyn Fn>`闭包，无法直接序列化。

**解决方案**：
1. **方案1**：不序列化事件监听器，反序列化后重新绑定
2. **方案2**：设计可序列化的"事件处理器描述符"
3. **方案3**：将事件处理逻辑单独处理，仅序列化DOM结构

---

## 7. 综合技术方案

### 7.1 推荐方案：渐进式混合策略

**核心思想**：根据框架特性选择最适合的方案，优先使用方案B，必要时引入方案A。

**架构图**：
```
┌─────────────────────────────────────────────────────────────┐
│                         Director                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ 代码分离器   │  │ SpiderMonkey │  │  jrust1     │      │
│  │ (可选)       │  │ (预执行引擎) │  │ (Rust执行)  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      序列化层                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │    rkyv      │  │   bincode    │  │    prost     │      │
│  │ (零拷贝)     │  │  (通用)      │  │ (跨语言)     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 框架支持矩阵

| 框架 | 预编译 | 预执行引擎 | 事件监听处理 | 推荐方案 |
|------|--------|-----------|-------------|---------|
| **Vue 3** | ✅ vite-plugin-vue-precompile | jrust1 | jrust-translator | 方案B |
| **Svelte** | ✅ 内置预编译 | jrust1 | jrust-translator | 方案B |
| **React** | ✅ swc JSX转换 | SpiderMonkey | jrust-translator | 方案A+B |
| **Angular** | ✅ AOT编译 | SpiderMonkey | jrust-translator | 方案A+B |
| **Vanilla JS** | ❌ 无需 | jrust1 | jrust-translator | 方案B |

### 7.3 实施路径

#### Phase 1：完善基础能力（P0）
1. 完善DOM API（querySelector、CSS选择器）
2. 设计事件序列化机制（EventDescriptor）
3. 集成rkyv序列化

#### Phase 2：方案B深化（P1）
1. jrust1完整DOM初始化能力
2. 闭包序列化支持
3. Vue 3完整支持

#### Phase 3：方案A集成（P2）
1. SpiderMonkey预执行引擎集成
2. React/Angular支持
3. 代码分离器实现

---

## 8. 关键设计：事件描述符机制

### 8.1 EventDescriptor结构

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDescriptor {
    pub event_type: String,           // "click", "focus", etc.
    pub handler_id: String,           // 处理器的唯一标识
    pub target_selector: String,      // 目标元素选择器
    pub handler_code: String,          // 处理器代码（用于动态重建）
    pub captured_values: serde_json::Value, // 捕获的变量值
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventHandlerRegistry {
    pub handlers: HashMap<String, EventHandlerInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventHandlerInfo {
    pub id: String,
    pub code: String,
    pub captured_types: Vec<String>,  // 捕获变量的类型信息
}
```

### 8.2 事件监听器序列化流程

```rust
pub fn serialize_event_listeners(element: &Element) -> Vec<EventDescriptor> {
    let mut descriptors = Vec::new();

    for (event_type, handlers) in &element.event_listeners {
        for (idx, handler) in handlers.iter().enumerate() {
            descriptors.push(EventDescriptor {
                event_type: event_type.to_string(),
                handler_id: format!("{:p}_{}", handler, idx),
                target_selector: element.get_selector(),
                handler_code: extract_handler_code(handler),
                captured_values: extract_captured_values(handler),
            });
        }
    }

    descriptors
}

pub fn deserialize_and_rebind(
    elements: &mut [Element],
    descriptors: &[EventDescriptor],
    registry: &EventHandlerRegistry,
) -> Result<()> {
    for desc in descriptors {
        // 1. 查找目标元素
        let element = find_element(elements, &desc.target_selector)?;

        // 2. 从注册表重建处理器
        let handler_info = registry.handlers.get(&desc.handler_id)
            .ok_or_else(|| Error::HandlerNotFound(desc.handler_id.clone()))?;

        // 3. 重建处理器（注入捕获的值）
        let handler = rebuild_handler(handler_info, &desc.captured_values)?;

        // 4. 重新绑定
        element.add_event_listener(
            EventType::from_str(&desc.event_type),
            handler,
        );
    }

    Ok(())
}
```

---

## 9. 下一步行动计划

| 优先级 | 任务 | 方案 | 预计工作项 | 依赖 |
|--------|------|------|-----------|------|
| **P0** | 完善DOM API | 方案B | querySelector、CSS选择器、Element操作 | - |
| **P0** | 设计事件序列化机制 | 方案B | EventDescriptor、EventHandlerRegistry | DOM API |
| **P0** | 集成rkyv序列化 | 方案B | 序列化框架集成、性能测试 | 事件序列化 |
| **P1** | 实现代码分离器 | 方案A+B | 静态分析、AST分析 | - |
| **P1** | SpiderMonkey预执行集成 | 方案A | Servo集成、DOM快照 | 代码分离器 |
| **P1** | Vue 3完整支持 | 方案B | 模板预编译、DOM序列化 | rkyv集成 |
| **P2** | React支持 | 方案A+B | swc JSX转换、SpiderMonkey执行 | SpiderMonkey |
| **P2** | Angular支持 | 方案A+B | AOT编译、运行时适配 | SpiderMonkey |

---

## 10. 风险与缓解措施

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| SpiderMonkey集成复杂度 | 中 | 中 | 使用Servo已有的集成方案 |
| 事件监听器序列化不完整 | 高 | 中 | 设计描述符机制，分阶段实现 |
| 性能达不到预期 | 中 | 高 | 持续benchmark，rkyv优化 |
| 框架兼容性问题 | 中 | 高 | 渐进式支持，优先Vue/React |

---

## 11. 结论

### 11.1 核心建议

1. **采用渐进式混合策略**：以方案B为主，根据框架特性引入方案A
2. **优先支持Vue 3和Svelte**：这两个框架的预编译方案最成熟
3. **利用SpiderMonkey处理复杂框架**：React、Angular等需要JS引擎支持
4. **设计事件描述符机制**：解决事件监听器序列化问题
5. **使用rkyv序列化**：追求最佳启动性能

### 11.2 关键技术点

1. **代码分离**：识别初始化代码与事件监听代码的边界
2. **事件序列化**：将闭包转换为可序列化的描述符
3. **渐进式支持**：按优先级支持不同框架

### 11.3 架构优势

- **灵活性**：根据需求选择方案A或B
- **可扩展性**：容易添加新的框架支持
- **性能优化**：通过序列化减少启动时间
- **可维护性**：纯Rust架构更清晰

---

**文档版本历史**：
- v0.1：初稿，整合技术评估讨论结果
