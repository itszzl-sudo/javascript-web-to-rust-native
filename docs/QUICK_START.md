# 快速入门指南

## 环境要求

- Rust 1.94+ (rustup 推荐)
- Node.js 20+ (用于 Vue 项目构建)

## 快速开始

### 1. 构建项目

```bash
# 克隆项目
git clone https://github.com/itszzl-sudo/javascript-web-to-rust-native.git
cd javascript-web-to-rust-native

# 构建所有组件
cargo build --release
```

### 2. 运行示例

```bash
# 基础运行时示例
cargo run --example basic_usage -p jrust-runtime

# 端到端集成示例
cargo run --example e2e_integration -p jrust-runtime

# 绑定系统演示
cargo run --example binding_demo -p jrust-runtime

# Director 工作流程演示
cargo run -p director
```

### 3. 转译 JavaScript 代码

```bash
# 使用 jrust-translator CLI
.\target\release\jrust-translator.exe input.js -o output.rs
```

### 4. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定 crate 测试
cargo test -p jrust-translator  # 65+ 测试
cargo test -p jrust-runtime     # 36 测试
```

## 核心组件

| Crate | 功能 | 快速测试 |
|-------|------|---------|
| jrust-translator | JS → Rust 转译器 | `cargo test -p jrust-translator` |
| jrust-runtime | 运行时环境 | `cargo run --example basic_usage -p jrust-runtime` |
| cranelift-compiler | 原生编译器 | `cargo build -p cranelift-compiler` |
| director | CLI 编排工具 | `cargo run -p director` |

## 项目结构

```
src/
├── jrust-translator/     # JavaScript → Rust 转译器
├── jrust-runtime/        # 运行时环境 (DOM/BOM/GC)
├── cranelift-compiler/   # Cranelift 原生编译器
├── binding-generator/    # 过程宏绑定生成器
├── jrust-servo/          # Servo 浏览器集成
├── jrust-browser/        # rust-browser 集成
├── director/             # CLI 编排工具
└── vue-demo/             # Vue 应用演示
```

## 下一步

- 查看 [架构文档](./ARCHITECTURE.md) 了解设计原理
- 查看 [状态文档](./STATUS.md) 了解当前进度
- 查看 [已知问题](./KNOWN_ISSUES.md) 了解限制
