# Director Embed 模式设计

## 概念区分

### rust-browser embed
rust-browser **始终**使用 embed 模式（w2n 场景）：
- 无 GUI（无 egui/eframe）
- WebNativeBridge API
- 渲染到内存

### director embed
director 的 **生成物**形态选择：
- `--embed`: 生成嵌入库/CLI（无 egui 依赖）
- 默认: 生成独立窗口应用（有 egui 依赖）

## 架构

```
rust-browser (embed 模式)
     ↓
jrust-browser (使用 rust-browser embed)
     ↓
director 编译流程:
     ↓
生成物选择:
├── --embed 模式
│   ├── 输出: app.lib / app.dll
│   ├── 入口: 无 main()，提供 API
│   ├── 依赖: 无 egui
│   └── 用途: 嵌入其他程序、CLI 工具
│
└── 默认窗口模式
    ├── 输出: app.exe
    ├── 入口: main() 启动 egui 窗口
    ├── 依赖: eframe, egui
    └── 用途: 独立桌面应用
```

## 命令行参数

```bash
# 嵌入模式（生成库）
director input.js --embed -o ./lib

# 窗口模式（生成独立应用）
director input.js -o ./app
```

## 生成物对比

| 特性 | --embed | 默认 |
|------|---------|------|
| 输出格式 | .lib / .dll | .exe |
| 入口函数 | 无 / api() | main() |
| egui 依赖 | ❌ | ✅ |
| 窗口支持 | ❌ | ✅ |
| WebNativeBridge | ✅ | ✅ |
| 渲染方式 | 内存 → API | 内存 → 窗口 |
| 适用场景 | 嵌入、CLI | 桌面应用 |

## 实现方案

### 1. 代码生成模板

**嵌入模式** (`--embed`):
```rust
// 不生成 main()，只导出 API
pub use jrust_browser::*;

pub fn render(url: &str) -> Vec<u8> {
    let mut browser = BrowserInstance::new(...);
    browser.navigate(url).unwrap();
    browser.render()
}
```

**窗口模式** (默认):
```rust
// 生成 main() 启动 egui
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("App", options, Box::new(|cc| Box::new(App::new(cc))));
}

struct App {
    browser: BrowserInstance,
}

impl eframe::App for App {
    fn ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 渲染 browser 到 egui 窗口
        let pixmap = self.browser.render();
        egui::Painter::image(ctx, pixmap);
    }
}
```

### 2. Cargo.toml 模板

**嵌入模式**:
```toml
[package]
name = "app"
version = "0.1.0"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
jrust-browser = { path = "..." }
# 无 egui
```

**窗口模式**:
```toml
[package]
name = "app"
version = "0.1.0"

[[bin]]
name = "app"

[dependencies]
jrust-browser = { path = "..." }
eframe = "0.34"
egui = "0.34"
```

### 3. 链接方式

**嵌入模式**:
```bash
# 生成 lib
cargo build --lib
# 输出: target/debug/app.dll / app.lib
```

**窗口模式**:
```bash
# 生成 exe
cargo build
# 输出: target/debug/app.exe
```

## 使用场景

### 嵌入模式示例

**嵌入到 C++ 应用**:
```cpp
// C++ 调用 Rust lib
extern "C" {
    uint8_t* render(const char* url);
}

int main() {
    auto data = render("https://example.com");
    // 使用渲染数据
}
```

**CLI 工具**:
```bash
# 无窗口渲染
app-cli render input.html -o output.png
```

### 窗口模式示例

**独立桌面应用**:
```bash
# 启动窗口应用
./app.exe
```

## 文件结构

```
output/
├── embed 模式
│   ├── app.lib       # Windows 静态库
│   ├── app.dll       # Windows 动态库
│   ├── app.a         # Linux 静态库
│   └── app.h         # C API 头文件
│
└── 窗口模式
    ├── app.exe       # Windows
    └── app           # Linux/macOS
```

## 下一步

1. 修改 director/main.rs 添加 `--embed` 参数解析
2. 创建代码生成模板（embed_template.rs, window_template.rs）
3. 创建 Cargo.toml 生成逻辑
4. 测试两种模式的编译和运行
