# 资源加载指南

**更新日期**: 2026-05-22

## 概述

Jade 编译的原生应用**不支持运行时网络资源加载**。但支持：
1. **编译时下载** - CDN JS 和 Google Fonts 自动下载
2. **打包嵌入** - 本地资源嵌入到可执行文件（推荐）
3. **本地文件** - 从 exe 所在目录读取

---

## CDN JavaScript 下载

### 自动下载流程

当 Jade 检测到 CDN URL 时：

```
<script src="https://unpkg.com/vue@3/dist/vue.global.js"></script>
```

处理步骤：
1. 检测到 CDN URL
2. 弹框提示用户确认下载
3. 下载到 `output/downloaded_js/`
4. 通过 jrust-translator 转译为 Rust
5. 编译到最终可执行文件

### 下载提示

```
============================================================
📥 下载资源 (尝试 1/3)
   URL: https://unpkg.com/vue@3/dist/vue.global.js
============================================================

按 Enter 继续下载，输入 'q' 取消: _
```

**用户操作**：
- **Enter** - 继续下载
- **q** - 取消下载，退出 Jade

**下载失败处理**：
```
⚠️  下载失败 (尝试 1/3): 网络错误: Connection timeout
   URL: https://unpkg.com/vue@3/dist/vue.global.js
   2 秒后重试...

[重试 3 次]

============================================================
❌ 下载失败
   URL: https://unpkg.com/vue@3/dist/vue.global.js
   重试 3 次后仍失败，退出 Jade
============================================================
```

### 支持的 CDN

| CDN | 示例 URL |
|-----|----------|
| unpkg | `https://unpkg.com/vue@3/dist/vue.global.js` |
| jsdelivr | `https://cdn.jsdelivr.net/npm/vue@3/dist/vue.global.js` |
| cdnjs | `https://cdnjs.cloudflare.com/ajax/libs/vue/3.0.0/vue.global.js` |
| jQuery | `https://code.jquery.com/jquery-3.6.0.min.js` |
| Google APIs | `https://ajax.googleapis.com/ajax/libs/jquery/3.6.0/jquery.min.js` |

### 手动下载

```rust
use jrust_runtime::resource::{download_cdn_js, is_cdn_url};

if is_cdn_url(url) {
    let js_path = download_cdn_js(url, Path::new("./output"))?;
    jrust_translator::translate_file(&js_path)?;
}
```

---

## Google Fonts 下载

### 自动下载流程

```html
<link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">
```

处理步骤：
1. 检测到 Google Fonts URL
2. 弹框提示用户确认下载
3. 下载 `.ttf` 字体文件到 `output/fonts/`
4. 编译时嵌入或运行时从本地读取

### 下载提示

```
============================================================
📥 下载资源 (尝试 1/3)
   URL: https://fonts.gstatic.com/s/roboto/v1/Roboto.ttf
============================================================

按 Enter 继续下载，输入 'q' 取消: _
```

### 手动下载

```rust
use jrust_runtime::resource::download_google_font;

let font_path = download_google_font("Roboto", Path::new("./output"))?;
// 输出: output/fonts/Roboto.ttf
```

### 支持的字体格式

- `.ttf` - TrueType Font
- `.otf` - OpenType Font
- `.woff` - Web Open Font Format
- `.woff2` - Web Open Font Format 2

---

## Downloader API

### 启用网络功能

```toml
# Cargo.toml
[dependencies]
jrust-runtime = { version = "0.2.0", features = ["network"] }
```

### API 参考

```rust
use jrust_runtime::resource::{
    Downloader,
    download_cdn_js,
    download_google_font,
    is_cdn_url,
    is_google_fonts_url,
};

// 检测 URL 类型
if is_cdn_url(url) {
    let js_path = download_cdn_js(url, output_dir)?;
}

if is_google_fonts_url(url) {
    let font_path = download_google_font("Roboto", output_dir)?;
}

// 自定义下载
let mut downloader = Downloader::new();
downloader.download_file(url, dest_path)?;
```

### 错误处理

```rust
use jrust_runtime::resource::DownloadError;

match download_cdn_js(url, output_dir) {
    Ok(path) => println!("下载成功: {}", path),
    Err(DownloadError::Cancelled) => {
        println!("用户取消");
        std::process::exit(1);
    }
    Err(DownloadError::MaxRetriesExceeded) => {
        println!("下载失败，退出");
        std::process::exit(1);
    }
    Err(DownloadError::Network(msg)) => eprintln!("网络错误: {}", msg),
    Err(DownloadError::Io(e)) => eprintln!("IO 错误: {}", e),
}
```

---

## 外部依赖检测

### 自动检测

Jade 在转译时自动检测代码中的外部依赖：

```rust
use jrust_translator::{detect_external_deps, ExternalDepType};

let source = r#"
    import Vue from 'https://unpkg.com/vue@3';
    const font = new FontFace('Roboto', 'url(https://fonts.gstatic.com/s/roboto/Roboto.ttf)');
    img.src = 'https://example.com/logo.png';
"#;

let deps = detect_external_deps(source);

for dep in deps {
    match dep.dep_type {
        ExternalDepType::JavaScript => println!("JS: {}", dep.url),
        ExternalDepType::Font => println!("Font: {}", dep.url),
        ExternalDepType::Image => println!("Image: {}", dep.url),
        ExternalDepType::Other => println!("Other: {}", dep.url),
    }
}
```

### 检测范围

| 类型 | 检测模式 |
|------|----------|
| JavaScript | `import ... from 'https://...'`<br>`require('https://...')`<br>`import('https://...')` |
| 字体 | `fonts.googleapis.com`<br>`fonts.gstatic.com`<br>`@import url(...)` |
| 图片 | `.src = 'https://...*.png'`<br>`url('https://...*.jpg')` |
| 其他 | `fetch('https://...')`<br>`axios.get('https://...')` |

### 处理流程

```rust
use jrust_translator::process_external_deps;

let source = std::fs::read_to_string("app.js")?;

// 检测并处理所有外部依赖
let processed_files = process_external_deps(&source, "./output")?;

for path in processed_files {
    println!("已处理: {}", path.display());
}
```

---

## ResourceLoader API

### 基础用法

```rust
use jrust_runtime::resource::{ResourceLoader, load_font, load_image};

// 设置资源目录（可选，默认为 exe 所在目录）
ResourceLoader::set_resource_dir("./assets");
ResourceLoader::set_font_dir("./assets/fonts");
ResourceLoader::set_image_dir("./assets/images");

// 加载字体
let font_data = load_font("Roboto-Regular.ttf");

// 加载图片
let img_data = load_image("logo.png");

// 或使用 data URL（base64）
let img_data = load_image("data:image/png;base64,iVBORw0KGgo...");
```

### 调试 vs 发布模式

```rust
// 自动切换：
// - debug: 从文件系统读取（./output/assets）
// - release: 使用嵌入资源

#[cfg(debug_assertions)]
{
    ResourceLoader::set_resource_dir("./output/assets");
    ResourceLoader::use_filesystem();
}

#[cfg(not(debug_assertions))]
{
    ResourceLoader::use_embedded();
}
```

---

## 编译时嵌入资源

### 方式 1：宏嵌入

```rust
use jrust_runtime::{embed_font, embed_image};

// 编译时嵌入
embed_font!("Roboto-Regular.ttf", "../assets/fonts/Roboto-Regular.ttf");
embed_image!("logo.png", "../assets/images/logo.png");

// 运行时自动从嵌入资源读取
let font = load_font("Roboto-Regular.ttf");  // 从嵌入读取
let logo = load_image("logo.png");            // 从嵌入读取
```

### 方式 2：手动嵌入

```rust
use jrust_runtime::resource::embed_resource;

embed_resource("fonts/MyFont.ttf", include_bytes!("../assets/MyFont.ttf").to_vec());
embed_resource("images/icon.png", include_bytes!("../assets/icon.png").to_vec());
```

---

## 图片加载策略

### 1. Data URL（推荐）

JavaScript 端：
```javascript
// Vite 自动转为 base64
import logo from './assets/logo.png';  // logo = "data:image/png;base64,..."

const img = new Image();
img.src = logo;  // ✅ 支持
```

Rust 端：
```rust
let img_data = load_image("data:image/png;base64,iVBORw0KGgo...");
```

### 2. 本地文件

```javascript
img.src = "./images/logo.png";  // 相对于 exe 所在目录
```

```rust
// ResourceLoader 自动解析：
// 1. ./images/logo.png (相对于 image_dir)
// 2. ./assets/images/logo.png (相对于 resource_dir)
// 3. 绝对路径
```

### 3. 网络图片（不支持）

```javascript
img.src = "https://example.com/image.png";  // ❌ 返回 None
```

---

## 字体加载策略

### 1. 嵌入字体（推荐）

```rust
embed_font!("Inter-Regular.ttf", "../assets/fonts/Inter-Regular.ttf");
embed_font!("Inter-Bold.ttf", "../assets/fonts/Inter-Bold.ttf");
```

### 2. 系统字体

```rust
// 自动检测系统字体目录
let font = load_font("Arial.ttf");  // Windows: C:/Windows/Fonts/Arial.ttf
let font = load_font("SF Pro.ttf"); // macOS: /System/Library/Fonts/SF Pro.ttf
```

### 3. 本地字体目录

```
output/
├── my-app.exe
└── fonts/
    ├── Inter-Regular.ttf
    └── Inter-Bold.ttf
```

```rust
ResourceLoader::set_font_dir("./fonts");
let font = load_font("Inter-Regular.ttf");
```

---

## 完整示例

### 项目结构

```
my-app/
├── src/
│   └── main.rs          # Rust 入口
├── assets/
│   ├── fonts/
│   │   ├── Inter-Regular.ttf
│   │   └── Inter-Bold.ttf
│   └── images/
│       └── logo.png
├── dist/
│   └── bundle.js        # Vite 打包输出
└── output/              # Jade 编译输出
    ├── my-app.exe
    ├── fonts/           # 复制字体
    └── images/          # 复制图片
```

### main.rs

```rust
use jrust_runtime::resource::{ResourceLoader, embed_font, embed_image, load_font, load_image};

// 编译时嵌入资源
embed_font!("Inter-Regular.ttf", "../assets/fonts/Inter-Regular.ttf");
embed_image!("logo.png", "../assets/images/logo.png");

fn main() {
    // 初始化资源加载器
    #[cfg(debug_assertions)]
    ResourceLoader::set_resource_dir("./output");
    
    #[cfg(not(debug_assertions))]
    ResourceLoader::use_embedded();
    
    // 加载资源
    let font = load_font("Inter-Regular.ttf").expect("Font not found");
    let logo = load_image("logo.png").expect("Logo not found");
    
    // 渲染应用
    // ...
}
```

---

## 构建脚本

### 自动复制资源到输出目录

```rust
// build.rs
use std::fs;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    // 复制字体
    fs::create_dir_all(format!("{}/fonts", out_dir)).ok();
    for entry in fs::read_dir("assets/fonts").unwrap() {
        let entry = entry.unwrap();
        let dest = format!("{}/fonts/{}", out_dir, entry.file_name().to_string_lossy());
        fs::copy(entry.path(), dest).ok();
    }
    
    // 复制图片
    fs::create_dir_all(format!("{}/images", out_dir)).ok();
    for entry in fs::read_dir("assets/images").unwrap() {
        let entry = entry.unwrap();
        let dest = format!("{}/images/{}", out_dir, entry.file_name().to_string_lossy());
        fs::copy(entry.path(), dest).ok();
    }
    
    println!("cargo:rerun-if-changed=assets/");
}
```

---

## 不支持的功能

| 资源类型 | 支持 | 说明 |
|----------|------|------|
| 本地文件 | ✅ | 相对或绝对路径 |
| Data URL | ✅ | base64 / URL-encoded |
| 嵌入资源 | ✅ | include_bytes! |
| 系统字体 | ✅ | Windows/macOS/Linux |
| HTTP URL | ❌ | 不支持网络请求 |
| CDN | ❌ | 不支持网络请求 |
| Google Fonts | ❌ | 需预先下载 |

---

## 性能对比

| 方式 | 首次加载 | 后续访问 | 文件大小 |
|------|----------|----------|----------|
| 嵌入资源 | ~0ms | ~0ms | exe 增大 |
| 本地文件 | ~1-5ms | ~1-5ms | 不变 |
| Data URL | ~0.5ms | ~0.5ms | bundle 增大 |
| 网络请求 | ❌ | ❌ | ❌ |

---

## 相关文件

- `src/jrust-runtime/src/resource/loader.rs` - ResourceLoader 实现
- `src/jrust-runtime/src/resource/mod.rs` - 模块导出
- `docs/KNOWN_ISSUES.md` - 限制说明
