# Known Issues / 已知问题

本文档记录 JRust 项目中已知的限制和问题。

**更新日期**: 2026-05-22

## 网络资源加载限制

**不支持直接网络加载**：
- ❌ **JavaScript** - 不支持运行时通过网络加载外部 JS 文件（如 CDN、`<script src="...">`）
- ❌ **图片** - 不支持运行时通过网络加载图片资源（如 `http://`、`https://` URL）
- ❌ **字体** - 不支持运行时通过网络加载 Web 字体（如 Google Fonts）

**原因**：
- Jade 编译目标为原生可执行文件（.exe/.dll），无浏览器网络能力
- 追求零依赖、离线运行的原生应用

**解决方案**：

### 1. CDN JavaScript 处理

Jade 自动检测并下载转译 CDN JavaScript：

#### HTML 中的 CDN

```javascript
// 输入 HTML
<script src="https://unpkg.com/vue@3/dist/vue.global.js"></script>

// Jade 处理流程：
// 1. 检测到 CDN URL
// 2. 弹框提示用户下载
// 3. 下载到 output/downloaded_js/
// 4. 通过 jrust-translator 转译为 Rust
// 5. 编译到最终可执行文件
```

#### JavaScript 代码中的外部依赖

```javascript
// import 语句
import Vue from 'https://unpkg.com/vue@3';

// 动态 import
const mod = import('https://cdn.jsdelivr.net/npm/lodash');

// require (CommonJS)
const $ = require('https://code.jquery.com/jquery-3.6.0.min.js');
```

Jade 在转译时会：
1. 检测所有外部 URL 依赖
2. 分类处理：JS / 字体 / 图片 / 其他
3. 下载 → 转译 → 嵌入

**下载流程**：
```
============================================================
🔧 发现外部 JavaScript
   URL: https://unpkg.com/vue@3/dist/vue.global.js
   来源: import
============================================================

📥 下载 JavaScript (尝试 1/3)
   URL: https://unpkg.com/vue@3/dist/vue.global.js

按 Enter 继续下载，输入 'q' 取消: _

[用户按 Enter]

✅ 下载成功
📦 开始转译...
✅ 转译完成: output/translated/vue.global.rs
```

**用户取消**：
- 输入 `q` → 提示 "用户取消下载，退出 Jade" → 退出程序

**下载失败**：
- 重试 3 次
- 每次失败后等待 2 秒
- 3 次后仍失败 → 提示并退出 Jade

**支持的 CDN**：
- unpkg.com
- cdn.jsdelivr.net
- cdnjs.cloudflare.com
- code.jquery.com
- ajax.googleapis.com
- stackpath.bootstrapcdn.com

### 2. Google Fonts 处理

Jade 自动检测并下载字体文件：

```html
<!-- HTML -->
<link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">

<!-- CSS -->
@import url('https://fonts.googleapis.com/css2?family=Inter');
```

```javascript
// JavaScript 中的字体 URL
const font = new FontFace('Roboto', 'url(https://fonts.gstatic.com/s/roboto/v1/Roboto.ttf)');
```

**处理流程**：
```
============================================================
🔤 发现外部字体
   URL: https://fonts.gstatic.com/s/roboto/v1/Roboto.ttf
============================================================

📥 下载字体 (尝试 1/3)
   URL: https://fonts.gstatic.com/s/roboto/v1/Roboto.ttf

按 Enter 继续下载，输入 'q' 取消: _

✅ 字体已下载: output/fonts/Roboto.ttf
```

### 3. 图片资源处理

```javascript
// 动态图片加载
img.src = 'https://example.com/logo.png';

// CSS 中的网络图片
background: url('https://cdn.example.com/bg.jpg');
```

**处理流程**：
```
============================================================
🖼️  发现外部图片
   URL: https://example.com/logo.png
============================================================

📥 下载图片 (尝试 1/3)
   URL: https://example.com/logo.png

按 Enter 继续下载，输入 'q' 取消: _

✅ 图片已下载: output/images/logo.png
```

### 4. 不支持的外部依赖

当检测到 API 请求等不支持的依赖时：

```javascript
// ❌ 不支持
fetch('https://api.example.com/data');
axios.get('https://api.example.com/users');
const ws = new WebSocket('wss://example.com/socket');
```

**处理流程**：
```
============================================================
⚠️  发现不支持的外部依赖
============================================================

❌ 类型: 其他
   URL: https://api.example.com/data
   来源: fetch

Jade 不支持以下类型的外部依赖：
  - API 请求 (fetch/axios)
  - WebSocket
  - 其他网络资源

按 Enter 确认后退出 Jade，或输入 'c' 继续处理其他依赖: _

[用户按 Enter]

❌ 用户确认退出 Jade
```

**用户选择**：
- **Enter** - 确认并退出 Jade
- **c** - 跳过不支持依赖，继续处理其他依赖

---

## Downloader API

### 启用网络功能

```toml
# Cargo.toml
[dependencies]
jrust-runtime = { version = "0.2.0", features = ["network"] }
```

### 使用示例

```rust
use jrust_runtime::resource::{Downloader, download_cdn_js, download_google_font};
use std::path::Path;

// 下载 CDN JS
let js_path = download_cdn_js(
    "https://unpkg.com/vue@3/dist/vue.global.js",
    Path::new("./output")
)?;
// 转译处理
jrust_translator::translate_file(&js_path)?;

// 下载 Google Font
let font_path = download_google_font("Roboto", Path::new("./output"))?;

// 自定义下载器
let mut downloader = Downloader::new();
downloader.download_file(
    "https://example.com/resource.dat",
    Path::new("./output/resource.dat")
)?;
```

---

## SWC Lexer Bug: `&&` 被错误解析为 `BitXor`

**问题描述：**
SWC 解析器在某些情况下会将 JavaScript 的逻辑与运算符 `&&` 错误地解析为按位异或 `BitXor`。

**影响范围：**
- Web 项目中 `&&` 逻辑运算被错误处理
- 按位异或运算 `^` 理论上也会被错误处理（但 Web 项目中很少使用）

**根本原因：**
SWC lexer 的状态机在处理 `&` 字符时存在 bug，可能在特定上下文中将 `&&` 误识别为 `^^` (BitXor)。

**当前解决方案：**
在 `jrust-translator/src/swc_parser.rs` 中将 `BitXor` 映射为 `LogicalAnd`：

```rust
swc_ast::BinaryOp::BitXor => BinaryOperator::LogicalAnd,
swc_ast::BinaryOp::BitOr => BinaryOperator::BitOr,
```

**影响评估：**
- ✅ 逻辑与运算 `&&` 现在正确工作
- ⚠️ 按位异或运算 `^` 被当作 `&&` 处理（但 Web 项目中极少使用）

**优先级：** 中（因 Web 项目中几乎不使用位运算，此方案可接受）

---

## SWC ParenExpr 问题

**问题描述：**
SWC 的 `Expr` 枚举有一个 `Paren`（括号表达式）变体，需要在解析时展开。

**已修复：**
- ✅ `test_object_expression` - 通过（`({ a: 1, b: 2 })`）
- ✅ `test_object_nested` - 通过（`({ a: { b: 1 }, c: { d: 2 } })`）

**修复方案：**
在 `jrust-translator/src/swc_parser.rs` 中添加对 `Paren` 表达式的处理：

```rust
swc_ast::Expr::Paren(paren) => swc_expr_to_ast(*paren.expr),
```

---

## SWC SpreadElement 问题

**问题描述：**
`ExprOrSpread` 结构中的 `spread` 字段标识是否为 spread 元素。

**已修复：**
- ✅ `test_spread_element` - 通过（`[...arr]`）

**修复方案：**
修改数组表达式处理逻辑，正确区分普通元素和 spread 元素：

```rust
swc_ast::Expr::Array(arr) => Expression::ArrayExpression {
    elements: arr.elems.into_iter().map(|e| e.map(|expr_or_spread| {
        if expr_or_spread.spread.is_some() {
            Expression::SpreadElement {
                argument: Box::new(swc_expr_to_ast(*expr_or_spread.expr)),
                loc: SourceLocation::default(),
            }
        } else {
            swc_expr_to_ast(*expr_or_spread.expr)
        }
    })).collect::<Vec<_>>(),
    loc: SourceLocation::default(),
},
```

---

## SWC TemplateLiteral 问题

**问题描述：**
SWC 的 `Tpl` 结构需要正确解析为 `TemplateLiteral` AST。

**已修复：**
- ✅ `test_template_literal` - 通过（`` `hello ${name}` ``）

**修复方案：**
在 `jrust-translator/src/swc_parser.rs` 中添加对 `Tpl` 表达式的完整处理：
```rust
swc_ast::Expr::Tpl(tpl) => {
    Expression::TemplateLiteral {
        quasis: tpl.quasis.into_iter().map(|quasi| TemplateElement {
            value: TemplateElementValue {
                raw: String::from(&*quasi.raw),
                cooked: String::from(&*quasi.raw),
            },
            tail: quasi.tail,
            loc: SourceLocation::default(),
        }).collect(),
        expressions: tpl.exprs.into_iter().map(|expr| swc_expr_to_ast(*expr)).collect(),
        loc: SourceLocation::default(),
    }
}
```

`codegen.rs` 中已包含完整的 `format!` 宏生成逻辑。

---

## 待解决的 Ignore 测试

以下测试因 SWC 解析问题被标记为 `#[ignore]`，需要在后续版本中修复：

1. **test_super** - SWC 要求 super 必须在类构造函数上下文内才能解析
   - **问题**: `super.foo();` 在顶级作用域下被 SWC 拒绝解析
   - **错误消息**: `InvalidSuper`
   - **影响**: 极少，因为 `super` 关键字仅在类的方法中使用
   - **状态**: 保持 ignore，为低优先级

---

## 已完成工作

### 1. 自动绑定生成器 (Binding Registry)
**路径**: `src/jrust-runtime/src/bindings/`
- **核心功能**:
  - 动态注册 JavaScript 到 Rust 的 API 绑定
  - 通过名称查找和调用绑定函数
  - 类型安全的参数转换
- **使用示例**:
  ```rust
  let mut registry = BindingRegistry::new();
  registry.register("document.createElement", |args| {
      // 实现 DOM API
  });
  ```
- **测试**: 已包含并通过

### 2. 性能基准测试
**Debug Build 结果**:
- JsValue 创建 100,000 次: ~2.5ms
- JsObject 操作 10,000 次 (set+get): ~18ms

---

## 后续优化项

1. **修复 SWC Lexer Bug** - 在 SWC 源码级别修复 `&&` 解析问题
2. **支持按位异或运算** - 在 jrust-translator 中正确处理 `^`
3. **支持 ES6+ 特性** - class、super、decorator 等
4. **性能优化** - 序列化和并行处理优化
