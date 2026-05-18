# 开发指南

## 环境设置

### 必需依赖

- **Rust**: 1.94+ (通过 rustup 安装)
- **Node.js**: 20+ (用于 Vue 项目构建)

### 推荐工具

- **rust-analyzer**: VS Code Rust 插件
- **cargo-watch**: 自动重新编译
- **cargo-expand**: 宏展开调试

```bash
# 安装推荐工具
rustup component add rust-analyzer
cargo install cargo-watch cargo-expand
```

---

## 开发工作流

### 1. 代码修改

```bash
# 监听模式自动编译
cargo watch -x "build --lib"

# 运行测试
cargo test --all

# 检查代码
cargo clippy --all
cargo fmt --all --check
```

### 2. 添加新功能

```bash
# 1. 在对应 crate 下添加代码
# 2. 添加测试
# 3. 运行测试验证
cargo test -p <crate-name>

# 4. 更新文档
```

### 3. 调试

```bash
# Debug 构建
cargo build

# 运行示例并打印日志
RUST_LOG=debug cargo run --example basic_usage -p jrust-runtime

# 宏展开
cargo expand -p jrust-translator
```

---

## 测试指南

### 单元测试

```bash
# 运行所有测试
cargo test --all

# 运行特定 crate
cargo test -p jrust-translator
cargo test -p jrust-runtime

# 运行特定测试
cargo test -p jrust-translator test_function_declaration

# 显示输出
cargo test -- --nocapture
```

### 集成测试

```bash
# 运行端到端示例
cargo run --example e2e_integration -p jrust-runtime
cargo run --example binding_demo -p jrust-runtime
```

### 测试覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

---

## 示例程序

### jrust-runtime 示例

| 示例 | 说明 |
|------|------|
| basic_usage | 基础运行时使用 |
| e2e_integration | 端到端集成演示 |
| binding_demo | 绑定系统演示 |
| benchmark | 性能基准测试 |

```bash
# 运行示例
cargo run --example <name> -p jrust-runtime
```

### director 示例

```bash
# 完整工作流程
cargo run -p director
```

---

## 常见问题

### 编译问题

**Q: Windows SDK 缺失**

```bash
# 解决方案 1: 安装 Windows SDK
# 通过 Visual Studio Installer 安装

# 解决方案 2: 使用 GNU 工具链
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**Q: SWC 解析错误**

部分 JavaScript 语法可能无法解析，参见 [KNOWN_ISSUES.md](./KNOWN_ISSUES.md)

### 运行时问题

**Q: 事件监听器无法序列化**

事件监听器包含闭包，无法直接序列化。需要使用 EventDescriptor 机制。

**Q: DOM 操作性能问题**

考虑使用批量操作 API，参见 [TECH_EVALUATION.md](./TECH_EVALUATION.md)

---

## 贡献指南

### 代码风格

```bash
# 格式化
cargo fmt --all

# Lint
cargo clippy --all -- -D warnings
```

### 提交规范

```
feat: 添加新功能
fix: 修复 bug
docs: 文档更新
test: 测试相关
refactor: 重构
chore: 构建/工具相关
```

### PR 流程

1. Fork 仓库
2. 创建分支: `git checkout -b feature/xxx`
3. 提交更改: `git commit -m "feat: xxx"`
4. 推送分支: `git push origin feature/xxx`
5. 创建 Pull Request

---

## 项目配置

### Cargo.toml 配置

```toml
[workspace]
members = [
    "src/jrust-translator",
    "src/jrust-runtime",
    "src/cranelift-compiler",
    ...
]

[workspace.dependencies]
serde = "1.0"
anyhow = "1.0"
# ...
```

### 示例配置

在 crate 的 `Cargo.toml` 中添加:

```toml
[[example]]
name = "my_example"
path = "examples/my_example.rs"
```

---

## 性能优化

### 编译优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### 运行时优化

- 使用 `rkyv` 零拷贝序列化
- 避免不必要的克隆
- 使用 `Rc<RefCell<T>>` 而非 `Arc<Mutex<T>>` (单线程)

---

## 调试技巧

### 日志配置

```rust
use tracing::{info, debug, trace};

debug!("Processing node: {:?}", node);
```

```bash
RUST_LOG=debug cargo run
```

### 内存调试

```bash
# 使用 valgrind (Linux)
valgrind --leak-check=full ./target/debug/my-program

# 使用 Instruments (macOS)
instruments -t Leaks ./target/debug/my-program
```

### 性能分析

```bash
# 使用 cargo flamegraph
cargo install flamegraph
cargo flamegraph --root
```
