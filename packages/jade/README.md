# @irisverse/jade

JavaScript to Native compiler. **No binary download required.**

## Architecture

```
Director (JS)          - 仅调度，无编译逻辑
     ↓
SWC Parser (@swc/core) - JS AST 解析 (Vercel 签名)
     ↓
IR Generator (Rust)    - AST → IR (Node native)
     ↓
Code Generator (Rust)  - IR → .obj (Cranelift)
     ↓
Linker (可选)          - .obj → .exe
```

### Components

| Component | Language | Provider | Trust |
|-----------|----------|----------|-------|
| Director | JavaScript | This package | ✅ 源码透明 |
| SWC Parser | Rust/WASM | Vercel | ✅ npm 签名 |
| IR Generator | Rust | This package | ✅ 可审计 |
| Code Generator | Rust | Cranelift | ✅ ByteDance 开源 |

## Why This Approach?

- ✅ **单一职责** - Director 仅调度，编译逻辑在 Rust
- ✅ **类型安全** - Rust IR/Code Generator 编译时类型检查
- ✅ **易维护** - 统一用 Rust 实现编译逻辑
- ✅ **透明可审计** - 所有源码可见，无隐藏二进制
- ✅ **签名依赖** - Cranelift (ByteDance), SWC (Vercel)

## Installation

```bash
npm install -D @irisverse/jade
```

**That's it!** No binary downloads, no network requests during install.

## Usage

```bash
# Compile Vue app
jade dist/assets/index.js -n my-app

# Embed mode (library)
jade input.js --embed -n my-lib
```

## Building Native Module (Optional)

If you want to rebuild the native module:

```bash
# Build Rust native module
npm run build:native

# Or manually:
cargo build --release -p jade-native
node scripts/copy-native.js
```

**Note**: Pre-built native module is included in the package. You only need to rebuild if modifying the Rust code.

## Example

```javascript
// Use programmatically
const { Director } = require('@irisverse/jade');

const director = new Director();
const outputPath = await director.compile(jsCode, {
  outputName: 'my-app',
  embed: false
});
```

## Requirements

### Windows

- **.NET Framework 4.0+** (required for MSVC linker)
  - Automatically detected at runtime
  - If missing, download page opens automatically
  - Download: https://dotnet.microsoft.com/download/dotnet-framework

### Linking Options

1. **Project toolchain** (recommended)
   - Copy link.exe + dependencies to `toolchain/win32-x64-msvc/`
   - See: `toolchain/README.md`

2. **LLD** (cross-platform, ~5 MB)
   - Install: `choco install llvm` (Windows)
   - Or copy `lld.exe` to `toolchain/win32-x64-msvc/`

3. **System linker**
   - Windows: Visual Studio Build Tools
   - macOS: Xcode Command Line Tools
   - Linux: build-essential

## Next Steps

After generating `.obj` file, you can:

### Option 1: Link with system linker (Windows)

```bash
# Requires Visual Studio Build Tools
jade link my-app.obj servo-zero.lib -o my-app.exe
```

### Option 2: Link with LLD (cross-platform)

```bash
# Install LLD
# Windows: choco install llvm
# macOS: brew install llvm
# Linux: apt install lld

jade link my-app.obj servo-zero.lib -o my-app.exe
```

### Option 3: Pack as static library

```bash
jade pack my-app.obj -n mylib
# Output: mylib.lib (Windows) or libmylib.a (Linux/macOS)
```

**Note**: The linker requires either:
- Visual Studio Build Tools (Windows)
- LLD (LLVM linker) 
- System linker (ld on Linux/macOS)

## License

MIT
