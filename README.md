# JavaScript-Web-to-Rust-Native (Jade)

将任意打包后的 JavaScript 代码编译为原生 Rust 二进制程序。

> **核心理念**: "浏览器不关心你的源码是什么框架，只关心最终能否执行 JavaScript"

---

## 编译路径

Jade 提供**两条编译路径**，灵活选择：

### 路径 1：jrust 路径（完整 Web 应用）

```
JavaScript → SWC → jrust-translator → jrust-runtime → .exe (~2MB)
```

**适用**：完整 Web 应用（Vue/React/Angular + DOM + 事件）

**状态**：~95% 完成 ✅

### 路径 2：ts-native 路径（纯计算 + 可选 DOM）

```
TypeScript → ts-native → Extensions → .exe (10-14KB + extensions)
```

**适用**：纯计算、命令行工具、简单 DOM 渲染

**状态**：核心完成 ✅
- 编译器：ts-native v0.1.6
- 运行时：ts-native-runtime v0.1.0
- 扩展包：stdlib + dom (事件系统)
- 链接器：jade/link.exe 集成

**核心特性**：
- 插件机制：Cargo metadata 识别
- 事件系统：分发器 + 联动事件
- DOM API：与 jrust-browser 集成
- 体积优势：10KB vs 2MB

**体积优势**：

| 场景 | jrust 路径 | ts-native 路径 |
|------|-----------|---------------|
| 纯计算 | ~2MB | 10-14KB |
| DOM 渲染 | ~2MB | ~500KB |

详见 [ts-native 技术路径](../TS-NATIVE-PATH.md)

---

## Jade - 诗咏
君子温如玉，名器号 Jade。  
一器御四域，持剑下云阶。  
八派横剑气，诸锋各自裂。  
谁言碎难补？一碗盛残月。  
炉火煅新璧，端上琉璃碟。  
把盏临窗坐，此世已无缺。

---

## 特性

- **通用性**: 支持所有打包后的 JavaScript 代码
- **高性能**: Rust 零成本抽象 + Cranelift JIT
- **内存安全**: Rust 所有权系统消除内存漏洞
- **跨平台**: Windows / macOS / Linux / iOS / Android
- **离线运行**: 自动下载并嵌入 CDN JS、字体、图片资源

## 项目状态

**总体完成度**: ~95% | **当前阶段**: Phase 3 完成

| 组件 | 状态 | 测试 |
|------|------|------|
| jrust-translator | ✅ 完成 | 65 passed |
| jrust-runtime | ✅ 完成 | 54 passed |
| cranelift-compiler | ✅ 完成 | - |
| director | ✅ 完成 | - |
| jrust-browser | ✅ 完成 | 3 passed |
| jade-native | ✅ 完成 | 已验证 |

## 资源处理

Jade 自动处理外部依赖：

- **CDN JavaScript** - 自动下载并转译为 Rust
- **Google Fonts** - 自动下载字体文件
- **网络图片** - 自动下载并嵌入
- **不支持依赖** (fetch/WebSocket) - 弹框提示后退出

详见 [资源加载指南](docs/RESOURCE_LOADING.md) 和 [已知限制](docs/KNOWN_ISSUES.md)

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
JavaScript → SWC 解析 → IR → Cranelift 编译 → .obj → link.exe (.NET 4) → .exe/.dll
                ↓
            Rust 源码 + Cargo.toml (配套交付)
```

### 编译流程

```bash
# 1. Jade 编译 (Cranelift + .NET Framework)
jade compile app.js --output my-app

# 生成文件:
#   my-app.rs       - Rust 源码 (egui GUI)
#   Cargo.toml      - Cargo 配置
#   my-app.obj      - COFF 目标文件
#   my-app.exe      - 原生可执行文件
```

### 工具链

- **Cranelift** - JIT 编译器，生成 COFF 目标文件
- **link.exe** - MSVC 链接器 (项目内 toolchain/)
- **.NET Framework 4.0-4.8** - Windows 系统库链接器依赖

## 许可证

MIT OR Apache-2.0

## 鸣谢

本项目开发过程中得到了以下工具的支持：

- **[华为云码道（CodeArts）](https://www.huaweicloud.com/product/codearts.html)** - 华为云代码智能体，提供代码生成、重构和文档编写支持
- **[Trae](https://trae.ai/)** - AI 代码助手，提供技术咨询和代码审查支持