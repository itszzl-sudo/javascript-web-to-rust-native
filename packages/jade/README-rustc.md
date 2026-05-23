# jade 编译器 - rustc Native 方案

## 架构

```
JavaScript
    ↓
SWC Parser → AST
    ↓
IR Generator → Rust 源码
    ↓
cargo check → 类型验证
    ↓
cargo build --release → Native 可执行文件
```

## 核心组件

### 1. AST 解析
- 使用 SWC (`@swc/core`) 解析 JavaScript
- 支持所有 ES6+ 语法
- 速度：~1-5ms

### 2. IR 生成
- 遍历 AST 生成 Rust 源码
- 支持函数、类、组件
- 速度：~1-2ms

### 3. 类型验证
- 使用 `cargo check`
- 完整的类型检查、借用检查
- 速度：~100-500ms（有缓存）

### 4. 代码生成
- 使用 `cargo build --release`
- LLVM 优化
- 输出：Native 可执行文件

## 测试结果

| 示例 | AST 节点 | 函数数 | 验证 | 编译 | 输出大小 |
|-----|---------|--------|------|------|---------|
| simple.js | 136 | 2 | ✅ | ✅ | 4.0 MB |
| web-app.js | 274 | 4 | ✅ | ✅ | 4.0 MB |
| vue-component.js | 550 | 7 | ✅ | ✅ | - |
| nested-components.js | 491 | 4 | ✅ | ✅ | - |

## 性能

- **AST 解析**：1-5ms
- **IR 生成**：1-2ms
- **类型验证**：100-500ms（缓存）
- **编译**：10-30s（首次），1-5s（增量）

## 使用

### CLI
```bash
jade build -i input.js -o output
```

### API
```javascript
const director = new Director();
const exePath = await director.compile(jsCode, {
    outputName: 'app',
    useRustc: true
});
```

## 对比 Cranelift

| 方案 | IR 开发 | 验证 | 性能 | 体积 | 推荐度 |
|-----|---------|------|------|------|--------|
| **rustc** | ✅ 无需 | ✅ 完整 | ⭐⭐⭐⭐ | 4MB | ⭐⭐⭐⭐⭐ |
| **Cranelift** | ❌ 必须 | ⚠️ 部分 | ⭐⭐⭐ | 200B | ⭐⭐⭐ |

## 下一步

- [ ] 减小输出体积（去除 GUI 依赖）
- [ ] 性能优化（增量编译）
- [ ] 更多测试用例
- [ ] 错误处理改进
- [ ] 文档完善
