# Jade 用户指南

**版本**: 0.1.0  
**更新**: 2026-05-23

---

## 快速开始

### 安装

```bash
npm install -g @irisverse/jade
```

### 创建项目

```bash
# 创建 Vue 项目
jade init my-vue-app -t vue

# 创建 React 项目
jade init my-react-app -t react

# 创建原生 JS 项目
jade init my-app -t vanilla
```

### 开发与构建

```bash
cd my-app

# 开发模式（监听文件变化）
jade dev

# 构建桌面应用
jade build

# 构建 iOS 应用
jade mobile ios

# 构建 Android 应用
jade mobile android
```

---

## 命令详解

### jade init

创建新项目。

```bash
jade init <project-name> [options]

选项:
  -t, --template <template>  项目模板 (vue/react/vanilla)
```

**示例**:
```bash
jade init my-app              # 默认 vue 模板
jade init my-app -t react     # React 模板
jade init my-app -t vanilla   # 原生 JS 模板
```

**生成结构**:
```
my-app/
├── src/
│   └── index.js        # 入口文件
├── dist/               # 构建输出
├── public/             # 静态资源
├── package.json
├── jade.config.json    # Jade 配置
└── README.md
```

---

### jade build

构建项目。

```bash
jade build [options]

选项:
  -i, --input <file>    输入文件 (默认: src/index.js)
  -o, --output <dir>    输出目录 (默认: dist)
  -n, --name <name>     输出名称 (默认: app)
  -t, --target <target> 目标平台 (desktop/ios/android)
  -e, --embed           嵌入模式（生成库，无窗口）
```

**示例**:
```bash
jade build                           # 使用 jade.config.json
jade build -i src/main.js            # 指定入口
jade build -t ios                    # iOS 目标
jade build -e -n mylib               # 嵌入模式
jade build -o ./output -n myapp      # 自定义输出
```

---

### jade dev

开发模式，监听文件变化自动重新构建。

```bash
jade dev [options]

选项:
  -i, --input <file>  监听的文件 (默认: src/index.js)
```

**示例**:
```bash
jade dev                  # 监听默认文件
jade dev -i src/main.js   # 监听指定文件
```

---

### jade mobile

构建移动端应用。

```bash
jade mobile <platform>

平台:
  ios      iOS (aarch64-apple-ios)
  android  Android (aarch64-linux-android)
  all      所有平台
```

**示例**:
```bash
jade mobile ios       # 构建 iOS
jade mobile android   # 构建 Android
jade mobile all       # 构建所有平台
```

**输出**:
```
dist/
├── ios/
│   └── app          # iOS 可执行文件
└── android/
    └── libapp.so    # Android 动态库
```

---

### jade serve

启动开发服务器。

```bash
jade serve [port]

参数:
  port  端口号 (默认: 3000)
```

**示例**:
```bash
jade serve           # 端口 3000
jade serve 8080      # 端口 8080
```

---

### jade link

链接目标文件为可执行文件。

```bash
jade link <obj> [lib] -o <output>

选项:
  -o, --output <file>  输出文件
  -l, --lib <file>     要链接的库
```

**示例**:
```bash
jade link app.obj -o app.exe
jade link app.obj servo-zero.lib -o app.exe
```

---

### jade pack

打包目标文件为静态库。

```bash
jade pack <obj> -n <name>

选项:
  -n, --name <name>  库名称
```

**示例**:
```bash
jade pack app.obj -n mylib    # 输出 mylib.lib / libmylib.a
```

---

## 配置文件

### jade.config.json

项目根目录下的配置文件：

```json
{
  "input": "src/index.js",
  "output": "dist",
  "name": "my-app",
  "target": "desktop",
  "embed": false,
  "minify": true,
  "sourceMap": true,
  "template": "vue"
}
```

**配置项说明**:

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| input | string | src/index.js | 入口文件 |
| output | string | dist | 输出目录 |
| name | string | app | 输出名称 |
| target | string | desktop | 目标平台 |
| embed | boolean | false | 嵌入模式 |
| minify | boolean | true | 压缩输出 |
| sourceMap | boolean | true | 生成 source map |
| template | string | vue | 模板类型 |

---

## 目标平台

### Desktop (默认)

生成桌面应用可执行文件。

```bash
jade build
```

**输出**: `dist/app.exe` (Windows) 或 `dist/app` (Linux/macOS)

---

### iOS

生成 iOS 应用。

```bash
jade mobile ios
# 或
jade build -t ios
```

**要求**:
- macOS 系统
- Xcode 命令行工具
- Rust iOS target: `rustup target add aarch64-apple-ios`

**输出**: `dist/ios/app`

---

### Android

生成 Android 应用。

```bash
jade mobile android
# 或
jade build -t android
```

**要求**:
- Android NDK
- Rust Android target: `rustup target add aarch64-linux-android`

**输出**: `dist/android/libapp.so`

---

## 编程接口

### JavaScript API

```javascript
const { Director } = require('@irisverse/jade');

const director = new Director();

// 编译
const outputPath = await director.compile(jsCode, {
  outputName: 'my-app',
  embed: false,
  target: 'desktop'
});

// 链接
await director.link('app.obj', 'servo-zero.lib', 'app.exe');

// 打包
await director.packLib('app.obj', 'mylib');
```

### TypeScript API

```typescript
import { Director, CompileOptions } from '@irisverse/jade';

const director = new Director();

const options: CompileOptions = {
  outputName: 'my-app',
  embed: false,
  target: 'desktop',
  outputDir: 'dist'
};

const outputPath = await director.compile(jsCode, options);
```

---

## 工作流程

### 典型 Vue 项目

```bash
# 1. 创建项目
jade init my-vue-app -t vue
cd my-vue-app

# 2. 安装依赖
npm install

# 3. 开发
jade dev

# 4. 构建
jade build

# 5. 运行
./dist/my-vue-app.exe
```

### 嵌入模式（作为库使用）

```bash
# 构建
jade build -e -n mylib

# 输出
dist/mylib.lib    # Windows
dist/libmylib.a   # Linux/macOS
```

在 Rust 中使用：

```rust
// 链接生成的库
#[link(name = "mylib")]
extern "C" {
    fn run_app();
}

fn main() {
    unsafe { run_app() };
}
```

---

## 故障排除

### 常见问题

**1. 找不到输入文件**

```
❌ Input file not found: src/index.js
```

解决：检查文件路径或使用 `-i` 指定。

**2. 构建失败**

```
❌ Build failed: ...
```

解决：检查 JS 语法，确保支持 ES6+。

**3. iOS 构建失败**

```
error: can't find crate for `jrust-ios`
```

解决：
```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

**4. Android 构建失败**

```
error: can't find crate for `jrust-android`
```

解决：
```bash
rustup target add aarch64-linux-android
# 设置 ANDROID_NDK_HOME
```

---

## 高级用法

### 自定义构建脚本

在 package.json 中添加：

```json
{
  "scripts": {
    "build:release": "jade build --minify --sourceMap false",
    "build:debug": "jade build --minify false",
    "build:all": "jade build && jade mobile all"
  }
}
```

### 环境变量

```bash
# 指定 Rust 工具链
JADE_RUST_TOOLCHAIN=nightly jade build

# 指定 Android NDK
ANDROID_NDK_HOME=/path/to/ndk jade mobile android

# 详细输出
JADE_DEBUG=1 jade build
```

---

## 示例项目

查看 `packages/jade/examples/` 目录：

- `hello.js` - Hello World
- `vue-component.js` - Vue 3 组件
- `react-hooks.js` - React Hooks
- `full-features.js` - 完整特性演示

---

## 更新日志

### 0.1.0 (2026-05-23)

- ✅ 完整 CLI 命令体系
- ✅ jade init/build/dev/serve/mobile 命令
- ✅ 配置文件支持 (jade.config.json)
- ✅ iOS/Android 构建
- ✅ 开发模式（文件监听）
- ✅ Vue/React/Vanilla 模板

---

## 获取帮助

- 文档: https://github.com/itszzl-sudo/javascript-web-to-rust-native
- 问题: https://github.com/itszzl-sudo/javascript-web-to-rust-native/issues
- 命令帮助: `jade --help`
