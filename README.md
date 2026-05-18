# JavaScript-Web-to-Rust-Native

将任意打包后的 JavaScript 代码编译为原生 Rust 二进制程序。

> **核心理念**: "浏览器不关心你的源码是什么框架，只关心最终能否执行 JavaScript"

## 特性

- **通用性**: 支持所有打包后的 JavaScript 代码
- **高性能**: Rust 零成本抽象 + Cranelift JIT
- **内存安全**: Rust 所有权系统消除内存漏洞
- **跨平台**: Windows / macOS / Linux / iOS / Android

## 项目状态

**总体完成度**: ~90% | **当前阶段**: Phase 3 完成

| 组件 | 状态 | 测试 |
|------|------|------|
| jrust-translator | ✅ 完成 | 65 passed |
| jrust-runtime | ✅ 完成 | 54 passed |
| cranelift-compiler | ✅ 完成 | - |
| director | ✅ 完成 | - |
| jrust-browser | ✅ 完成 | 3 passed |

## 项目结构

```
src/
├── jrust-translator/     # JavaScript → Rust 转译器
├── jrust-runtime/        # 运行时 (DOM/BOM/GC)
├── cranelift-compiler/   # Cranelift 原生编译器
├── binding-generator/    # 过程宏绑定生成器
├── jrust-browser/        # rust-browser 集成
├── director/             # CLI 编排工具
└── vue-demo/             # Vue 应用演示
```

## 快速开始

```bash
# 构建
cargo build --release

# 运行示例
cargo run --example basic_usage -p jrust-runtime
cargo run --example e2e_integration -p jrust-runtime
cargo run -p director

# 转译 JavaScript
.\target\release\jrust-translator.exe input.js -o output.rs

# 测试
cargo test
```

## 文档

- [快速入门](docs/QUICK_START.md) - 快速上手指南
- [架构设计](docs/ARCHITECTURE.md) - 技术架构
- [项目状态](docs/STATUS.md) - 当前进度
- [路线图](docs/ROADMAP.md) - 开发计划
- [已知问题](docs/KNOWN_ISSUES.md) - 限制说明

## 工作原理

```
任意前端项目 → 打包工具 → 标准 JavaScript 
    → jrust-translator → Rust 代码 
    → jrust-runtime → 原生二进制
```

## 许可证

MIT OR Apache-2.0

## 鸣谢

本项目开发过程中得到了以下工具的支持：

- **[华为云码道（CodeArts）](https://www.huaweicloud.com/product/codearts.html)** - 华为云代码智能体，提供代码生成、重构和文档编写支持
- **[Trae](https://trae.ai/)** - AI 代码助手，提供技术咨询和代码审查支持