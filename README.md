# JavaScript-Web-to-Rust-Native

将任意打包后的 JavaScript 代码编译为原生 Rust 二进制程序的创新技术方案。

## 🌟 核心理念

> **"浏览器不关心你的源码是什么框架，只关心最终能否执行 JavaScript"**

| 浏览器关心的 | 浏览器不关心的 |
|-------------|----------------|
| 最终的 JavaScript 代码 | 源码框架 (Vue/React/Svelte) |
| DOM/BOM API 调用 | 打包工具 (Vite/Webpack/Rollup) |
| 事件系统 | 模块系统 (ESM/CJS/UMD) |
| 运行时语义 | JSX/TS/模板语法 |

## 🎯 特性

- **通用性**: 支持所有打包后的 JavaScript 代码
- **高性能**: 利用 Rust 的零成本抽象和 Servo 的并行渲染
- **内存安全**: Rust 的所有权系统消除内存安全漏洞
- **跨平台**: 支持 Windows、macOS、Linux、iOS、Android

## 📁 项目结构

```
javascript-web-to-rust-native/
├── docs/                  # 文档
│   ├── ARCHITECTURE.md        # 架构说明
│   └── ROADMAP.md             # 实现路线图
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
│       └── gc/                      # 垃圾回收
│
├── tests/                 # 测试
│   ├── unit/                  # 单元测试
│   ├── integration/            # 集成测试
│   └── e2e/                   # E2E 测试
│
└── examples/              # 示例
    └── vanilla-js/            # 原生 JavaScript 示例
```

## 🚀 快速开始

### 环境要求

- Rust 1.95+
- Node.js 20+

### 安装

```bash
# 克隆项目
git clone https://github.com/your-org/javascript-web-to-rust-native.git
cd javascript-web-to-rust-native

# 构建 javascript2rust
cargo build --release

# 构建运行时
cargo build --release -p javascript-web-runtime
```

### 使用

```bash
# 转译打包后的 JavaScript
./target/release/javascript2rust input.js -o output.rs

# 编译为原生二进制
cargo build --release
```

### 作为库使用

```rust
use javascript2rust::{compile, Optimize};

fn main() {
    let js_code = std::fs::read_to_string("bundle.js").unwrap();
    let rust_code = compile(&js_code)
        .optimize()
        .unwrap();
    println!("{}", rust_code);
}
```

## 📖 文档

- [架构方案](docs/ARCHITECTURE.md) - 完整的技术架构设计
- [实现路线图](docs/ROADMAP.md) - 详细的开发计划

## 🎨 工作原理

```
┌─────────────────────────────────────────────────────────┐
│  任意前端项目 (Vue / React / Svelte / 任意)              │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  任意打包工具 (Vite / Webpack / Rollup / esbuild)        │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  打包产物 = 标准 JavaScript (ESM / CJS / IIFE)           │
│                                                          │
│  // 示例：Vue 打包后                                     │
│  import { createApp } from 'vue'                         │
│  createApp(App).mount('#app')                            │
│                                                          │
│  // 示例：React 打包后                                   │
│  var React = require('react')                            │
│  React.createElement(App, null)                           │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  javascript2rust 转译器                                          │
│  - 解析标准 JavaScript                                   │
│  - 类型推断                                              │
│  - Rust 代码生成                                         │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  JavaScript-Web-Runtime (Servo 绑定)                     │
│  - DOM API                                              │
│  - BOM API                                              │
│  - 事件系统                                             │
│  - 框架运行时桥接                                        │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  原生二进制 (Windows / macOS / Linux / Mobile)           │
└─────────────────────────────────────────────────────────┘
```

## 🤝 贡献

欢迎贡献代码！请阅读 CONTRIBUTING.md（待编写）了解贡献指南。

## 📄 许可证

MIT License

## 🙋‍♂️ 联系方式

如有问题或建议，请创建 Issue。