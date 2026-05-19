# MSVC 工具链 vs LLD

## 方案对比

### MSVC link.exe（不推荐）

**需要拷贝的文件** (~100 MB):
- link.exe
- lib.exe
- mspdb*.dll (多个)
- msobj140.dll
- vcruntime*.dll
- ucrtbase.dll
- 其他依赖...

**问题**:
- 依赖太多
- 版权问题
- 仅 Windows

### LLD（推荐）

**需要拷贝的文件** (~5 MB):
- lld.exe (或 ld.lld)

**优点**:
- 单文件
- 开源 (LLVM)
- 跨平台
- 无依赖

## 如何获取 LLD

### 方法 1: 从 LLVM 官方下载

```bash
# Windows
choco install llvm
# 或下载: https://releases.llvm.org/download.html

# macOS
brew install llvm

# Linux
apt install lld
```

### 方法 2: 从 Rustup 获取

```bash
rustup component add llvm-tools-preview
# LLD 在: ~/.rustup/toolchains/stable-*/bin/lld
```

### 方法 3: 拷贝到项目

```bash
# 从系统拷贝
cp /usr/bin/lld packages/jade/toolchain/win32-x64-msvc/

# 或从 Rustup 拷贝
cp ~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/lld.exe packages/jade/toolchain/win32-x64-msvc/
```

## 当前支持

jade 已支持以下链接器（按优先级）:

1. 项目内工具链: `packages/jade/toolchain/win32-x64-msvc/link.exe`
2. 系统 LLD: `lld` 或 `ld.lld`
3. Rustup LLD: `~/.rustup/toolchains/stable-*/bin/lld`
4. 系统链接器: `link.exe` (Windows) / `ld` (Linux/macOS)
