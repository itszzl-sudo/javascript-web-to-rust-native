# Embed 模式 API 文档

**生成时间**: 自动生成  
**模式**: 嵌入模式 (`--embed`)

---

## 概述

本文档描述 `director --embed` 生成的嵌入库的 API 接口。

嵌入模式生成物：
- **Windows**: `app.dll` / `app.lib`
- **Linux**: `libapp.so` / `libapp.a`
- **macOS**: `libapp.dylib` / `libapp.a`

---

## Rust API

### 主要类型

#### `BrowserConfig`

浏览器配置。

```rust
pub struct BrowserConfig {
    pub width: u32,      // 视口宽度
    pub height: u32,     // 视口高度
    pub title: String,   // 窗口标题（嵌入模式忽略）
    pub headless: bool,  // 无头模式（始终 true）
}
```

#### `BrowserInstance`

浏览器实例，核心 API。

```rust
pub struct BrowserInstance { ... }

impl BrowserInstance {
    /// 创建浏览器实例
    pub fn new(config: BrowserConfig) -> Result<Self, String>;
}
```

#### `HttpResponse`

HTTP 响应。

```rust
pub struct HttpResponse {
    pub status: u16,                    // 状态码
    pub headers: HashMap<String, String>, // 响应头
    pub body: Vec<u8>,                  // 响应体
    pub final_url: String,              // 最终 URL（重定向后）
}
```

---

### DOM 操作

```rust
impl BrowserInstance {
    /// 设置页面 HTML
    pub fn set_html(&mut self, html: &str);
    
    /// 按 CSS 选择器查找元素
    pub fn query(&self, selector: &str) -> Option<usize>;
    
    /// 查找所有匹配元素
    pub fn query_all(&self, selector: &str) -> Vec<usize>;
    
    /// 获取元素属性
    pub fn get_attr(&self, node_id: usize, name: &str) -> Option<String>;
    
    /// 设置元素属性
    pub fn set_attr(&mut self, node_id: usize, name: &str, value: &str);
    
    /// 获取元素文本
    pub fn text(&self, node_id: usize) -> Option<String>;
    
    /// 获取元素标签名
    pub fn tag_name(&self, node_id: usize) -> Option<String>;
}
```

### 布局与渲染

```rust
impl BrowserInstance {
    /// 获取元素位置
    pub fn get_rect(&self, selector: &str) -> Option<(f32, f32, f32, f32)>;
    
    /// 渲染为 PNG 字节
    pub fn render(&mut self) -> Vec<u8>;
    
    /// 设置视口尺寸
    pub fn set_viewport(&mut self, width: u32, height: u32);
    
    /// 获取视口尺寸
    pub fn viewport(&self) -> (u32, u32);
}
```

### CSS 操作

```rust
impl BrowserInstance {
    /// 添加 CSS 规则
    pub fn set_css(&mut self, css: &str);
    
    /// 设置元素内联样式
    pub fn set_style(&mut self, selector: &str, property: &str, value: &str);
    
    /// 清除自定义 CSS
    pub fn clear_css(&mut self);
}
```

### 事件系统

```rust
impl BrowserInstance {
    /// 绑定点击事件
    pub fn on_click<F>(&mut self, selector: &str, handler: F)
    where F: FnMut(f32, f32) + Send + 'static;
    
    /// 绑定表单提交事件
    pub fn on_form_submit<F>(&mut self, selector: &str, handler: F)
    where F: FnMut(HashMap<String, String>) + Send + 'static;
    
    /// 绑定 window.open 事件
    pub fn on_window_open<F>(&mut self, handler: F)
    where F: FnMut(&str) -> bool + Send + 'static;
    
    /// 处理鼠标点击
    pub fn handle_click(&mut self, x: f32, y: f32) -> bool;
    
    /// 处理表单提交
    pub fn handle_form_submit(&mut self, form_selector: &str);
    
    /// 处理 window.open
    pub fn handle_window_open(&mut self, url: &str) -> bool;
}
```

### 网络请求

```rust
impl BrowserInstance {
    /// 导航到 URL
    pub fn navigate(&mut self, url: &str) -> Result<(), String>;
    
    /// 获取当前 URL
    pub fn current_url(&self) -> String;
    
    /// HTTP GET 请求
    pub fn http_get(&mut self, url: &str) -> Result<HttpResponse, String>;
    
    /// HTTP POST 请求
    pub fn http_post(&mut self, url: &str, body: &[u8], content_type: &str) 
        -> Result<HttpResponse, String>;
}
```

### 文件操作

```rust
impl BrowserInstance {
    /// 下载文件并保存
    pub fn download_file(&mut self, url: &str, path: &str) -> Result<u64, String>;
    
    /// 写入文件
    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), String>;
    
    /// 读取文件
    pub fn read_file(&mut self, path: &str) -> Result<Vec<u8>, String>;
}
```

### JS 执行

```rust
impl BrowserInstance {
    /// 执行 JavaScript 代码
    pub fn eval_js(&mut self, code: &str) -> String;
}
```

---

## C FFI API

用于 C/C++ 调用的 C 接口。

### 头文件 (`app.h`)

```c
#ifndef APP_H
#define APP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ── 生命周期 ──

/// 创建浏览器实例
void* browser_new(uint32_t width, uint32_t height);

/// 销毁浏览器实例
void browser_free(void* browser);

// ── DOM 操作 ──

/// 设置 HTML
void browser_set_html(void* browser, const char* html);

/// 查找元素（返回节点 ID，0 表示未找到）
uint64_t browser_query(void* browser, const char* selector);

/// 获取元素属性（返回 JSON 字符串，需调用者释放）
char* browser_get_attr(void* browser, uint64_t node_id, const char* name);

/// 设置元素属性
void browser_set_attr(void* browser, uint64_t node_id, const char* name, const char* value);

/// 获取元素文本（需调用者释放）
char* browser_get_text(void* browser, uint64_t node_id);

// ── 渲染 ──

/// 渲染为 PNG（返回字节指针，通过 out_size 返回大小）
uint8_t* browser_render(void* browser, uint64_t* out_size);

/// 释放渲染结果
void browser_render_free(uint8_t* data);

// ── 网络 ──

/// 导航到 URL（返回是否成功）
int browser_navigate(void* browser, const char* url);

/// 获取当前 URL（需调用者释放）
char* browser_current_url(void* browser);

/// HTTP GET（返回 JSON: {status, body_base64, final_url}）
char* browser_http_get(void* browser, const char* url);

/// HTTP POST（返回 JSON）
char* browser_http_post(void* browser, const char* url, const uint8_t* body, uint64_t body_size, const char* content_type);

// ── 文件操作 ──

/// 下载文件（返回字节数，失败返回 -1）
int64_t browser_download_file(void* browser, const char* url, const char* path);

/// 写入文件（返回是否成功）
int browser_write_file(void* browser, const char* path, const uint8_t* data, uint64_t size);

/// 读取文件（通过 out_size 返回大小）
uint8_t* browser_read_file(void* browser, const char* path, uint64_t* out_size);

// ── 事件 ──

/// 处理点击（返回是否命中元素）
int browser_handle_click(void* browser, float x, float y);

/// 处理 window.open（返回是否处理）
int browser_handle_window_open(void* browser, const char* url);

// ── JS 执行 ──

/// 执行 JS 代码（需调用者释放）
char* browser_eval_js(void* browser, const char* code);

// ── 内存管理 ──

/// 释放字符串
void string_free(char* s);

/// 释放字节 buffer
void buffer_free(uint8_t* buf);

#ifdef __cplusplus
}
#endif

#endif // APP_H
```

### C++ 示例

```cpp
#include "app.h"
#include <iostream>
#include <fstream>

int main() {
    // 创建浏览器
    void* browser = browser_new(1280, 720);
    
    // 导航
    if (browser_navigate(browser, "https://example.com")) {
        std::cout << "页面加载成功\n";
    }
    
    // 查询元素
    uint64_t title = browser_query(browser, "h1");
    if (title != 0) {
        char* text = browser_get_text(browser, title);
        std::cout << "标题: " << text << "\n";
        string_free(text);
    }
    
    // 渲染截图
    uint64_t size;
    uint8_t* png = browser_render(browser, &size);
    std::ofstream out("screenshot.png", std::ios::binary);
    out.write((char*)png, size);
    out.close();
    browser_render_free(png);
    
    // 下载文件
    int64_t downloaded = browser_download_file(
        browser,
        "https://example.com/file.pdf",
        "output.pdf"
    );
    std::cout << "下载: " << downloaded << " bytes\n";
    
    // 清理
    browser_free(browser);
    return 0;
}
```

---

## Python 绑定示例

使用 `ctypes` 调用：

```python
import ctypes
from ctypes import c_void_p, c_uint32, c_char_p, POINTER, c_uint8, c_uint64

# 加载库
lib = ctypes.CDLL("./app.dll")

# 类型定义
lib.browser_new.argtypes = [c_uint32, c_uint32]
lib.browser_new.restype = c_void_p

lib.browser_navigate.argtypes = [c_void_p, c_char_p]
lib.browser_navigate.restype = ctypes.c_int

# 使用
browser = lib.browser_new(1280, 720)
lib.browser_navigate(browser, b"https://example.com")

# 渲染
size = c_uint64()
png = lib.browser_render(browser, ctypes.byref(size))
with open("screenshot.png", "wb") as f:
    f.write(ctypes.string_at(png, size.value))
```

---

## Node.js 绑定示例

使用 `ffi-napi`:

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

const lib = ffi.Library('./app', {
  'browser_new': ['pointer', ['uint32', 'uint32']],
  'browser_navigate': ['int', ['pointer', 'string']],
  'browser_render': ['pointer', ['pointer', 'pointer']],
  'browser_free': ['void', ['pointer']],
});

const browser = lib.browser_new(1280, 720);
lib.browser_navigate(browser, 'https://example.com');

const sizePtr = ref.alloc('uint64');
const png = lib.browser_render(browser, sizePtr);
// ... 保存 PNG
```

---

## 内存管理规则

1. **创建/销毁**: 
   - `browser_new()` 返回的指针必须用 `browser_free()` 释放

2. **字符串**:
   - 返回 `char*` 的函数需要调用者用 `string_free()` 释放

3. **字节 buffer**:
   - 返回 `uint8_t*` 的函数需要用 `buffer_free()` 或特定释放函数

4. **渲染结果**:
   - `browser_render()` 返回的 buffer 用 `browser_render_free()` 释放

---

## 错误处理

- 返回 `int` 的函数：`0` 表示失败，非 `0` 表示成功
- 返回 `char*` 的函数：返回 JSON 字符串，包含 `error` 字段表示错误
- 返回指针的函数：`NULL` 表示失败

---

## 线程安全

- `BrowserInstance` 不是线程安全的
- 每个线程应创建独立的 `BrowserInstance`
- 可并行创建多个实例

---

## 性能建议

1. **重用实例**: 避免频繁创建/销毁浏览器实例
2. **批量操作**: 批量执行 DOM 操作而非单个操作
3. **缓存渲染**: 仅在需要时调用 `render()`
4. **异步网络**: 考虑使用后台线程处理网络请求

---

## 完整示例：Rust 嵌入

```rust
use app::{BrowserConfig, BrowserInstance};

fn main() {
    // 创建浏览器
    let config = BrowserConfig {
        width: 1280,
        height: 720,
        title: String::new(),
        headless: true,
    };
    let mut browser = BrowserInstance::new(config).unwrap();
    
    // 导航并渲染
    browser.navigate("https://example.com").unwrap();
    let png = browser.render();
    std::fs::write("screenshot.png", &png).unwrap();
    
    // 下载文件
    browser.download_file(
        "https://example.com/data.json",
        "data.json"
    ).unwrap();
    
    // 监听 window.open
    browser.on_window_open(|url| {
        if url.contains("attachment") {
            println!("下载附件: {}", url);
            true
        } else {
            false
        }
    });
}
```
