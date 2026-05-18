# 构建脚本使用指南

## 概述

`build-native.sh` (Linux/macOS) 和 `build-native.ps1` (Windows) 提供一键构建流程：

```
Vue 项目 → 打包优化 → Director 编译 → Native 可执行文件
```

## 前置条件

### Vue 项目要求

1. **已优化的 Vite 配置**：
   - 禁用 `eval`
   - 禁用 `new Function`
   - 目标: ES2015+

2. **项目结构**：
   ```
   my-vue-app/
   ├── package.json
   ├── vite.config.js (或 .ts)
   └── src/
   ```

### 系统要求

- **Node.js**: >= 16.x
- **Rust**: >= 1.70 (仅首次编译需要)
- **系统**: Windows / Linux / macOS

---

## 使用方式

### Windows (PowerShell)

```powershell
# 基础用法
.\scripts\build-native.ps1 -VueProject C:\projects\my-vue-app

# 指定输出名称
.\scripts\build-native.ps1 -VueProject C:\projects\my-vue-app -OutputName my-app

# 完整示例
.\scripts\build-native.ps1 `
    -VueProject "C:\Users\dev\projects\vue-dashboard" `
    -OutputName "dashboard"
```

### Linux / macOS (Bash)

```bash
# 基础用法
./scripts/build-native.sh /path/to/vue-project

# 指定输出名称
./scripts/build-native.sh /path/to/vue-project my-app

# 完整示例
./scripts/build-native.sh ~/projects/vue-dashboard dashboard
```

---

## 构建流程

### Step 1: Vue 项目打包优化

**自动操作**：

1. 生成 `vite.optimize.config.js`:
   ```js
   import { defineConfig } from 'vite'
   
   export default defineConfig({
       build: {
           target: 'es2015',
           minify: 'terser',
           terserOptions: {
               compress: {
                   evaluate: false,      // 禁用 eval
                   negate_iife: false,
               },
               output: {
                   beautify: false,
               }
           },
           rollupOptions: {
               output: {
                   format: 'es',
                   manualChunks: undefined,
               }
           }
       },
       esbuild: {
           legalComments: 'none',
       }
   })
   ```

2. 执行构建：
   ```bash
   npm run build -- --config vite.optimize.config.js
   ```

**输出检查**：

- ✅ 扫描 `dist/assets/*.js`
- ⚠️ 检测 `eval(` 和 `new Function(`
- ❌ 发现则发出警告

### Step 2: Director 编译

**自动操作**：

1. 读取打包后的 JS 文件
2. 调用 `director` CLI:
   ```
   director --input <js-file> --name <output-name> --output <output-dir>
   ```

**编译流程**：

```
JS → jrust-translator → Cranelift IR → cranelift-compiler → .obj
```

### Step 3: 输出结果

**输出目录结构**：

```
dist/
└── <output-name>/
    ├── <output-name>.obj     # 目标文件
    └── <output-name>.exe     # 最终可执行文件 (待链接)
```

---

## 手动调用 Director CLI

如果已完成 Vue 打包，可单独调用 Director：

```bash
# 编译单个 JS 文件
director --input ./dist/assets/index.js --name my-app

# 指定输出目录
director --input ./app.js --name my-app --output ./output

# 查看帮助
director --help
```

**Director CLI 选项**：

| 选项 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--input` | `-i` | 输入 JS 文件 | 必需 |
| `--name` | `-n` | 输出名称 | `app` |
| `--output` | `-o` | 输出目录 | `dist` |
| `--help` | `-h` | 显示帮助 | - |

---

## 完整示例

### 示例 1: Vue Dashboard 项目

```powershell
# 1. 进入 javascript-web-to-rust-native 目录
cd C:\projects\javascript-web-to-rust-native

# 2. 执行构建
.\scripts\build-native.ps1 `
    -VueProject "C:\projects\vue-dashboard" `
    -OutputName "dashboard"

# 输出:
# =========================================
#   Vue → Native 构建流程
# =========================================
# Vue 项目: C:\projects\vue-dashboard
# 输出名称: dashboard
# 
# === Step 1: Vue 项目打包优化 ===
# 生成 Vite 优化配置（禁用 eval、new Function）...
# ✅ 已生成 vite.optimize.config.js
# 检查依赖...
# 执行打包...
# ✅ Vue 项目打包完成
# 
# === Step 2: 查找打包结果 ===
# 找到 JS 文件: dist\assets\index.abc123.js
# 
# 检查禁用特性...
# ✅ JS 文件准备完成
# 
# === Step 3: Director 编译为 Native ===
# JS 代码大小: 123456 字节
# 调用 Director 编译...
# 
# =========================================
#   ✅ 构建完成
# =========================================
# 输出文件: dist\dashboard\dashboard.exe
```

### 示例 2: 从已打包的 Vue 项目构建

```bash
# 假设 Vue 项目已打包，dist/assets/index.js 存在

# 1. 直接调用 Director
./target/release/director \
    --input /path/to/vue-app/dist/assets/index.js \
    --name my-app \
    --output ./output

# 2. 输出
# =========================================
#   Director CLI - JS → Native
# =========================================
# 输入文件: /path/to/vue-app/dist/assets/index.js
# 输出名称: my-app
# 输出目录: ./output
# 
# JS 代码大小: 123456 字节
# 
# === Director: JS → Cranelift → Native ===
# 1. 翻译 JS 到 Cranelift IR...
# ✅ IR 生成完成
# 2. 编译 IR 到目标文件...
# ✅ 目标文件生成完成: 45678 字节
# 临时 obj 文件: "./temp.obj"
# 
# ✅ 编译成功！
# 目标文件: "./temp.obj"
# 输出位置: "./output/my-app/my-app.obj"
# 
# 下一步:
#   compiler.link_with_lib(&obj, "rust-browser.lib", "my-app.exe")
```

---

## 禁用特性检查

### 自动检测项

| 特性 | 正则 | 影响 |
|------|------|------|
| `eval` | `eval\(` | 动态代码执行 |
| `new Function` | `new Function\(` | 动态函数创建 |

### 处理方式

1. **发现 `eval`**:
   ```
   ⚠️  警告: 发现 eval 调用
   ```
   - 建议：在源码中移除或替换

2. **发现 `new Function`**:
   ```
   ⚠️  警告: 发现 new Function 调用
   ```
   - 建议：改用普通函数声明

### 手动检查

```bash
# Linux/macOS
grep -r "eval(" dist/assets/*.js
grep -r "new Function(" dist/assets/*.js

# Windows
Select-String -Path "dist\assets\*.js" -Pattern "eval\("
Select-String -Path "dist\assets\*.js" -Pattern "new Function\("
```

---

## 故障排除

### 问题 1: 未找到打包后的 JS 文件

**错误**:
```
❌ 未找到打包后的 JS 文件
```

**解决**:
1. 确认 Vue 项目已成功构建
2. 检查 `dist/assets/` 或 `dist/` 目录
3. 手动指定文件：
   ```bash
   director --input ./custom/path/app.js --name my-app
   ```

### 问题 2: Director 未编译

**错误**:
```
编译 Director...
cargo build --release -p director
```

**原因**: 首次运行需要编译 Director

**等待**: 编译时间约 1-2 分钟

### 问题 3: eval/new Function 警告

**警告**:
```
⚠️  警告: 发现 eval 调用
⚠️  警告: 发现 new Function 调用
```

**解决**:
1. 修改源码移除动态代码
2. 或接受限制（部分功能可能无法正常工作）

---

## 高级配置

### 自定义 Vite 优化

在 Vue 项目根目录创建 `vite.optimize.config.js`：

```js
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
    plugins: [vue()],
    
    build: {
        target: 'es2015',
        minify: 'terser',
        
        terserOptions: {
            compress: {
                evaluate: false,
                negate_iife: false,
                // 自定义压缩选项
                drop_console: true,
                drop_debugger: true,
            },
            output: {
                beautify: false,
                comments: false,
            }
        },
        
        rollupOptions: {
            output: {
                format: 'es',
                manualChunks: undefined,
                // 自定义输出
                entryFileNames: 'app.js',
                chunkFileNames: 'chunk-[hash].js',
            }
        }
    },
    
    esbuild: {
        legalComments: 'none',
    }
})
```

### 指定 rust-browser.lib 路径

构建完成后，手动链接：

```bash
# 假设已生成 app.obj
compiler.link_with_lib(
    &obj_bytes,
    "/path/to/rust-browser.lib",
    "output/app.exe"
)?;
```

---

## 性能参考

### 构建时间

| 项目规模 | Vue 打包 | Director 编译 | 总计 |
|----------|----------|---------------|------|
| 小型 (~50KB) | ~5s | ~1s | ~6s |
| 中型 (~200KB) | ~10s | ~3s | ~13s |
| 大型 (~1MB) | ~30s | ~10s | ~40s |

### 输出体积

| 输入 JS | rust-browser.lib | 最终 .exe |
|---------|------------------|-----------|
| 50KB | 2MB | ~2.1MB |
| 200KB | 2MB | ~2.3MB |
| 1MB | 2MB | ~3.2MB |

---

## 相关文档

- [Cranelift 编译器技术文档](./cranelift-native-compiler-zh-CN.md)
- [Director API 文档](../src/director/README.md)
- [Bridge API 参考](../../rust-browser/rust-browser/README.md)

---

**文档版本**: 1.0.0  
**最后更新**: 2026-05-18
