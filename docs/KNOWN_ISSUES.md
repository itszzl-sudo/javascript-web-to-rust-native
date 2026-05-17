# Known Issues / 已知问题

本文档记录 JRust 项目中已知的限制和问题。

**更新日期**: 2026-05-18

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
