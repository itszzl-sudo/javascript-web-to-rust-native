# 移动端支持方案 (iOS & Android)

**创建日期**: 2026-05-18  
**状态**: 规划中

---

## 1. 概述

将 JavaScript-Web-to-Rust-Native 扩展到移动端，实现 "Write Once, Run Anywhere"。

### 1.1 核心思路

```
Web 项目 (Vue/React/Svelte)
    ↓
jrust-translator (统一转译)
    ↓
Rust 代码 + jrust-runtime
    ↓
┌─────────────────────────────────────┐
│  平台适配层                          │
├──────────┬──────────┬───────────────┤
│ Desktop  │   iOS    │   Android     │
│ (现有)   │  (新增)  │   (新增)      │
├──────────┼──────────┼───────────────┤
│ rust-   │ UIKit/   │ Kotlin/       │
│ browser │ SwiftUI  │ Jetpack       │
│ (现有)  │          │ Compose       │
└──────────┴──────────┴───────────────┘
```

### 1.2 目标

| 平台 | 目标架构 | 渲染方案 | 优先级 |
|------|---------|---------|--------|
| iOS | aarch64-apple-ios | SwiftUI / UIKit | P0 |
| Android | aarch64-linux-android | Jetpack Compose | P0 |
| iOS Simulator | aarch64-apple-ios-sim | SwiftUI | P1 |
| Android x86 | x86_64-linux-android | Jetpack Compose | P1 |

---

## 2. 技术架构

### 2.1 编译目标

```toml
# Cargo.toml 跨平台配置

[workspace.metadata.cross]
targets = [
    "aarch64-apple-ios",        # iOS (ARM64)
    "aarch64-apple-ios-sim",    # iOS Simulator
    "aarch64-linux-android",    # Android (ARM64)
    "x86_64-linux-android",     # Android Emulator
]
```

### 2.2 平台抽象层

```
┌─────────────────────────────────────────────────────────────┐
│  jrust-runtime (核心运行时 - 平台无关)                      │
│  - JsValue, JsObject, JsArray                               │
│  - 事件系统核心                                             │
│  - 垃圾回收                                                 │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  jrust-platform (平台抽象层)                                │
│  - Platform trait (统一接口)                                │
│  - DOM → 原生 UI 映射                                       │
│  - 事件 → 平台事件桥接                                       │
└─────────────────────────────┬───────────────────────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ jrust-desktop   │ │ jrust-ios       │ │ jrust-android   │
│ - Servo 渲染    │ │ - SwiftUI 尲染  │ │ - Compose 渲染  │
│ - winit 窗口    │ │ - UIKit 桥接    │ │ - JNI 桥接      │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

### 2.3 Platform Trait 设计

```rust
// src/jrust-platform/src/lib.rs

/// 平台抽象 trait
pub trait Platform: Send + Sync {
    /// 创建视图
    fn create_view(&self, tag: &str) -> Result<ViewId, PlatformError>;
    
    /// 设置属性
    fn set_attribute(&self, view: ViewId, key: &str, value: &JsValue) -> Result<(), PlatformError>;
    
    /// 设置文本内容
    fn set_text_content(&self, view: ViewId, text: &str) -> Result<(), PlatformError>;
    
    /// 添加子视图
    fn append_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError>;
    
    /// 移除子视图
    fn remove_child(&self, parent: ViewId, child: ViewId) -> Result<(), PlatformError>;
    
    /// 添加事件监听
    fn add_event_listener(
        &self, 
        view: ViewId, 
        event: &str, 
        handler: EventHandler
    ) -> Result<(), PlatformError>;
    
    /// 移除事件监听
    fn remove_event_listener(
        &self, 
        view: ViewId, 
        event: &str
    ) -> Result<(), PlatformError>;
    
    /// 获取窗口尺寸
    fn window_size(&self) -> (f64, f64);
    
    /// 请求重绘
    fn request_redraw(&self);
}

/// 视图 ID (平台无关)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(usize);

/// 事件处理器
pub type EventHandler = Box<dyn Fn(JsValue) + Send + Sync>;
```

---

## 3. iOS 支持

### 3.1 架构

```
┌─────────────────────────────────────────────────────────────┐
│  iOS App (Swift/SwiftUI)                                    │
│  - ContentView                                              │
│  - JRustView (SwiftUI View)                                 │
└─────────────────────────────┬───────────────────────────────┘
                              │ Swift FFI
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  jrust-ios (Rust 静态库)                                    │
│  - Platform 实现 (SwiftUI 映射)                             │
│  - 事件桥接 (Combine → Rust)                                │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  jrust-runtime                                              │
│  - 核心运行时 (无修改)                                      │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 DOM → SwiftUI 映射

| DOM Element | SwiftUI View |
|-------------|--------------|
| `<div>` | `VStack` / `ZStack` |
| `<span>` | `Text` |
| `<button>` | `Button` |
| `<img>` | `Image` |
| `<input>` | `TextField` / `SecureField` |
| `<ul>/<li>` | `List` |
| `<a>` | `NavigationLink` |

### 3.3 项目结构

```
src/jrust-ios/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # FFI 导出
│   ├── platform.rs               # Platform trait 实现
│   ├── view_bridge.rs            # SwiftUI 视图桥接
│   ├── event_bridge.rs           # 事件桥接
│   └── bindings/
│       ├── swift.rs              # Swift 绑定生成
│       └── auto.rs               # 自动生成
└── swift/
    ├── JRustBridge.swift         # Swift 桥接层
    ├── JRustView.swift           # SwiftUI View 包装
    └── JRustEvent.swift          # 事件处理
```

### 3.4 FFI 接口设计

```rust
// src/jrust-ios/src/lib.rs

#[no_mangle]
pub extern "C" fn jrust_ios_create_view(tag: *const c_char) -> u64 {
    // ...
}

#[no_mangle]
pub extern "C" fn jrust_ios_set_text(view_id: u64, text: *const c_char) {
    // ...
}

#[no_mangle]
pub extern "C" fn jrust_ios_append_child(parent: u64, child: u64) {
    // ...
}

#[no_mangle]
pub extern "C" fn jrust_ios_add_event_listener(
    view_id: u64,
    event: *const c_char,
    callback: extern "C" fn(*const c_char),
) {
    // ...
}
```

### 3.5 Swift 端集成

```swift
// swift/JRustView.swift

import SwiftUI

public struct JRustView: UIViewRepresentable {
    let source: String
    
    public func makeUIView(context: Context) -> UIView {
        let bridge = JRustBridge()
        bridge.loadScript(source)
        return bridge.rootView
    }
    
    public func updateUIView(_ uiView: UIView, context: Context) {
        // 处理更新
    }
}

// 使用示例
struct ContentView: View {
    var body: some View {
        JRustView(source: "compiled_js.js")
    }
}
```

### 3.6 构建配置

```toml
# src/jrust-ios/Cargo.toml

[package]
name = "jrust-ios"
version = "0.1.0"

[lib]
crate-type = ["staticlib", "cdylib"]

[dependencies]
jrust-runtime = { path = "../jrust-runtime" }
jrust-platform = { path = "../jrust-platform" }

[target.'cfg(target_os = "ios")'.dependencies]
objc = "0.2"
objc_id = "0.1"
objc-foundation = "0.1"
```

---

## 4. Android 支持

### 4.1 架构

```
┌─────────────────────────────────────────────────────────────┐
│  Android App (Kotlin/Jetpack Compose)                       │
│  - MainActivity                                             │
│  - JRustComposeView                                         │
└─────────────────────────────┬───────────────────────────────┘
                              │ JNI
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  jrust-android (Rust 共享库)                                │
│  - Platform 实现 (Compose 映射)                             │
│  - JNI 桥接                                                 │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  jrust-runtime                                              │
│  - 核心运行时 (无修改)                                      │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 DOM → Compose 映射

| DOM Element | Compose Function |
|-------------|------------------|
| `<div>` | `Column` / `Box` |
| `<span>` | `Text` |
| `<button>` | `Button` |
| `<img>` | `Image` |
| `<input>` | `TextField` / `OutlinedTextField` |
| `<ul>/<li>` | `LazyColumn` |
| `<a>` | 自定义 Navigation |

### 4.3 项目结构

```
src/jrust-android/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # JNI 导出
│   ├── platform.rs               # Platform trait 实现
│   ├── jni_bridge.rs             # JNI 桥接
│   └── compose_bridge.rs         # Compose 组件桥接
└── kotlin/
    ├── JRustBridge.kt            # Kotlin 桥接层
    ├── JRustComposeView.kt       # Compose View 包装
    └── JRustEvent.kt             # 事件处理
```

### 4.4 JNI 接口设计

```rust
// src/jrust-android/src/lib.rs

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};

#[no_mangle]
pub extern "system" fn Java_com_jrust_JRustBridge_createView(
    mut env: JNIEnv,
    _class: JClass,
    tag: JString,
) -> jlong {
    let tag: String = env.get_string(&tag).unwrap().into();
    // 创建视图并返回 ID
    0
}

#[no_mangle]
pub extern "system" fn Java_com_jrust_JRustBridge_setText(
    mut env: JNIEnv,
    _class: JClass,
    view_id: jlong,
    text: JString,
) {
    let text: String = env.get_string(&text).unwrap().into();
    // 设置文本
}

#[no_mangle]
pub extern "system" fn Java_com_jrust_JRustBridge_appendChild(
    env: JNIEnv,
    _class: JClass,
    parent: jlong,
    child: jlong,
) {
    // 添加子视图
}
```

### 4.5 Kotlin 端集成

```kotlin
// kotlin/JRustComposeView.kt

package com.jrust

import androidx.compose.runtime.Composable
import androidx.compose.ui.viewinterop.AndroidView

class JRustBridge {
    init {
        System.loadLibrary("jrust_android")
    }
    
    external fun createView(tag: String): Long
    external fun setText(viewId: Long, text: String)
    external fun appendChild(parent: Long, child: Long)
    external fun loadScript(source: String)
}

@Composable
fun JRustComposeView(source: String) {
    val bridge = remember { JRustBridge() }
    
    AndroidView(
        factory = { context ->
            bridge.loadScript(source)
            // 返回根视图
        },
        update = { view ->
            // 处理更新
        }
    )
}

// 使用示例
@Composable
fun MainActivity() {
    JRustComposeView(source = "compiled_js.js")
}
```

### 4.6 构建配置

```toml
# src/jrust-android/Cargo.toml

[package]
name = "jrust-android"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
jrust-runtime = { path = "../jrust-runtime" }
jrust-platform = { path = "../jrust-platform" }
jni = "0.21"

[target.'cfg(target_os = "android")'.dependencies]
ndk = "0.8"
ndk-sys = "0.5"
```

---

## 5. 统一构建流程

### 5.1 构建脚本

```bash
# scripts/build-mobile.sh

#!/bin/bash

# iOS 构建
build_ios() {
    echo "Building for iOS..."
    cargo build --target aarch64-apple-ios --release
    cargo build --target aarch64-apple-ios-sim --release
    
    # 生成 XCFramework
    lipo -create \
        target/aarch64-apple-ios/release/libjrust_ios.a \
        target/aarch64-apple-ios-sim/release/libjrust_ios.a \
        -output target/ios/libjrust_ios.a
}

# Android 构建
build_android() {
    echo "Building for Android..."
    cargo build --target aarch64-linux-android --release
    cargo build --target x86_64-linux-android --release
    
    # 生成 AAR
    # ... (使用 cargo-ndk 或 gradle)
}

# 根据参数选择构建目标
case "$1" in
    ios) build_ios ;;
    android) build_android ;;
    all) build_ios; build_android ;;
    *) echo "Usage: $0 {ios|android|all}" ;;
esac
```

### 5.2 Cross 工具链配置

```toml
# Cross.toml

[target.aarch64-linux-android]
image = "ghcr.io/cross-rs/aarch64-linux-android:0.2"

[target.x86_64-linux-android]
image = "ghcr.io/cross-rs/x86_64-linux-android:0.2"

[target.aarch64-apple-ios]
image = "ghcr.io/cross-rs/aarch64-apple-ios:0.2"
```

---

## 6. 事件系统桥接

### 6.1 事件映射

| Web 事件 | iOS (UIKit) | Android (View) |
|----------|-------------|----------------|
| `click` | `UITapGestureRecognizer` | `OnClickListener` |
| `input` | `UIControlEventEditingChanged` | `TextWatcher` |
| `focus` | `UIControlEventEditingDidBegin` | `OnFocusChangeListener` |
| `blur` | `UIControlEventEditingDidEnd` | `OnFocusChangeListener` |
| `scroll` | `UIScrollViewDelegate` | `OnScrollListener` |
| `touchstart` | `touchesBegan` | `OnTouchListener` |
| `touchmove` | `touchesMoved` | `OnTouchListener` |
| `touchend` | `touchesEnded` | `OnTouchListener` |

### 6.2 事件桥接实现

```rust
// src/jrust-platform/src/event.rs

/// 平台事件 → Web 事件转换
pub trait EventBridge {
    fn to_web_event(&self) -> WebEvent;
}

#[derive(Debug, Clone)]
pub struct WebEvent {
    pub event_type: String,
    pub target: ViewId,
    pub data: JsValue,
    pub timestamp: f64,
}

// iOS 实现
#[cfg(target_os = "ios")]
impl EventBridge for IosTouchEvent {
    fn to_web_event(&self) -> WebEvent {
        WebEvent {
            event_type: "touchstart".to_string(),
            target: self.view_id,
            data: JsValue::Object(/* touches */),
            timestamp: self.timestamp,
        }
    }
}

// Android 实现
#[cfg(target_os = "android")]
impl EventBridge for AndroidMotionEvent {
    fn to_web_event(&self) -> WebEvent {
        WebEvent {
            event_type: match self.action {
                0 => "touchstart",
                1 => "touchend",
                2 => "touchmove",
                _ => "unknown",
            }.to_string(),
            target: self.view_id,
            data: JsValue::Object(/* touches */),
            timestamp: self.event_time as f64,
        }
    }
}
```

---

## 7. 样式系统

### 7.1 CSS → 平台样式映射

```rust
// src/jrust-platform/src/style.rs

/// CSS 属性 → 平台样式转换
pub trait StyleMapper {
    fn apply(&self, view: ViewId, property: &str, value: &JsValue);
}

#[derive(Debug)]
pub enum PlatformStyle {
    Color(Color),
    Dimension(Dimension),
    Flex(FlexStyle),
    Text(TextStyle),
}

// iOS 实现
#[cfg(target_os = "ios")]
impl StyleMapper for IosStyleMapper {
    fn apply(&self, view: ViewId, property: &str, value: &JsValue) {
        match property {
            "color" => {
                let color = parse_color(value);
                // 设置 UILabel.textColor 或 SwiftUI.foregroundColor
            }
            "background-color" => {
                let color = parse_color(value);
                // 设置 UIView.backgroundColor
            }
            "font-size" => {
                let size = parse_dimension(value);
                // 设置 UIFont size
            }
            // ... 更多属性
        }
    }
}

// Android 实现
#[cfg(target_os = "android")]
impl StyleMapper for AndroidStyleMapper {
    fn apply(&self, view: ViewId, property: &str, value: &JsValue) {
        match property {
            "color" => {
                let color = parse_color(value);
                // 设置 Text color
            }
            "background-color" => {
                let color = parse_color(value);
                // 设置 Modifier.background
            }
            // ... 更多属性
        }
    }
}
```

### 7.2 Flexbox 支持

使用现有 Rust Flexbox 实现 (如 `stretch` 或 `taffy`)：

```rust
use taffy::{Taffy, Layout, Style};

pub fn compute_layout(root: ViewId, tree: &ViewTree) -> Layout {
    let mut taffy = Taffy::new();
    
    // 构建样式树
    let node = build_taffy_node(root, tree);
    
    // 计算布局
    taffy.compute_layout(node, tree.size()).unwrap()
}
```

---

## 8. 开发路线图

### Phase 1: 基础架构 (2 周)

| 任务 | 预计时间 | 产出 |
|------|---------|------|
| 创建 jrust-platform crate | 2 天 | Platform trait 定义 |
| 设置交叉编译工具链 | 1 天 | Cross.toml, 构建脚本 |
| 实现 iOS FFI 基础 | 3 天 | jrust-ios 静态库 |
| 实现 Android JNI 基础 | 3 天 | jrust-android 共享库 |
| 基础视图创建/销毁 | 2 天 | create_view, destroy_view |

### Phase 2: 视图系统 (2 周)

| 任务 | 预计时间 | 产出 |
|------|---------|------|
| DOM → SwiftUI 映射 | 3 天 | iOS 视图创建 |
| DOM → Compose 映射 | 3 天 | Android 视图创建 |
| 属性设置桥接 | 2 天 | set_attribute |
| 文本内容处理 | 1 天 | set_text_content |
| 子视图管理 | 2 天 | append_child, remove_child |

### Phase 3: 事件系统 (1 周)

| 任务 | 预计时间 | 产出 |
|------|---------|------|
| iOS 事件桥接 | 2 天 | 点击、触摸事件 |
| Android 事件桥接 | 2 天 | 点击、触摸事件 |
| 事件冒泡/捕获 | 1 天 | 完整事件流 |

### Phase 4: 样式系统 (1 周)

| 任务 | 预计时间 | 产出 |
|------|---------|------|
| CSS 解析 | 1 天 | 解析基础 CSS |
| iOS 样式映射 | 2 天 | 颜色、字体、布局 |
| Android 样式映射 | 2 天 | 颜色、字体、布局 |
| Flexbox 布局 | 1 天 | 使用 taffy |

### Phase 5: 示例与文档 (1 周)

| 任务 | 预计时间 | 产出 |
|------|---------|------|
| iOS 示例应用 | 2 天 | SwiftUI Demo |
| Android 示例应用 | 2 天 | Compose Demo |
| 文档编写 | 1 天 | 集成指南 |

---

## 9. 技术挑战与解决方案

| 挑战 | 解决方案 |
|------|---------|
| **交叉编译** | 使用 `cross` 工具，配置 Docker 镜像 |
| **FFI 安全性** | 使用 `#[no_mangle]` + `extern "C"`，避免 panic 跨 FFI |
| **内存管理** | Rust 侧统一管理，通过 ID 引用 |
| **事件线程** | 平台主线程 → Rust 事件队列 → 回调执行 |
| **样式差异** | 统一 CSS 子集，平台特定扩展 |
| **性能** | 避免频繁 FFI 调用，批量更新 |

---

## 10. 依赖项

### Rust 依赖

```toml
# jrust-platform/Cargo.toml
[dependencies]
jrust-runtime = { path = "../jrust-runtime" }
taffy = "0.3"              # Flexbox 布局
cssparser = "0.31"         # CSS 解析

[target.'cfg(target_os = "ios")'.dependencies]
objc = "0.2"
block = "0.1"

[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
ndk = "0.8"
```

### iOS 依赖

- Xcode 15+
- iOS 15+ deployment target
- Swift 5.9+

### Android 依赖

- Android SDK 24+
- NDK r25+
- Kotlin 1.9+
- Jetpack Compose 1.5+

---

## 11. 总结

### 优势

1. **代码复用**: jrust-translator 和 jrust-runtime 100% 复用
2. **统一 API**: Platform trait 提供统一抽象
3. **原生性能**: Rust 编译为原生代码，无 JS 引擎开销
4. **类型安全**: Rust 类型系统保证安全性

### 下一步行动

1. 创建 `jrust-platform` crate
2. 实现 iOS 基础 FFI
3. 实现 Android 基础 JNI
4. 构建示例应用验证

---

**相关文档**:
- [项目状态](./STATUS.md)
- [架构设计](./ARCHITECTURE.md)
- [路线图](./ROADMAP.md)
