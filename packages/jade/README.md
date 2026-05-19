# @irisverse/jade

JavaScript to Native compiler - compile Vue/React/Svelte apps to native executables.

## Installation

```bash
npm install -D @irisverse/jade
```

## Usage

### CLI

```bash
# Compile Vue app
jade build dist/assets/index.js -o ./output -n my-app

# Generate embed library
jade build dist/assets/index.js --embed -n my-lib

# Quick compile
jade dist/assets/index.js
```

### Programmatic API

```typescript
import { compile } from '@irisverse/jade';

const result = await compile({
  input: 'dist/assets/index.js',
  output: './output',
  name: 'my-app',
  embed: false
});

if (result.success) {
  console.log('Compiled successfully!');
  console.log(result.output);
} else {
  console.error('Error:', result.error);
}
```

## Workflow

```
Vue/React/Svelte Project
        ↓
npm run build (Vite/Webpack)
        ↓
dist/assets/index.js
        ↓
jade build dist/assets/index.js
        ↓
Native Executable (my-app.exe / my-app)
```

## Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--output` | `-o` | `dist` | Output directory |
| `--name` | `-n` | `app` | Output name |
| `--embed` | `-e` | `false` | Embed mode (library, no window) |
| `--help` | `-h` | | Show help |

## Modes

### Window Mode (default)

Generate standalone desktop application:

```bash
jade build input.js -n my-app
# → my-app.exe (Windows)
# → my-app (Linux/macOS)
```

### Embed Mode

Generate library for embedding:

```bash
jade build input.js --embed -n my-lib
# → my-lib.dll / my-lib.lib (Windows)
# → libmy-lib.so / libmy-lib.a (Linux)
```

Use in other programs:
- C/C++: Link with `.lib` / `.a`
- Node.js: Load with `ffi-napi`
- Python: Load with `ctypes`

## Supported Frameworks

- ✅ Vue 3 (with pre-compilation)
- ✅ React (with Hooks)
- ✅ Svelte
- ✅ Preact
- ✅ SolidJS
- ✅ Angular (with Ivy)
- ✅ Lit
- ✅ Qwik

## Requirements

- Node.js >= 18.0.0
- Supported platforms: Windows x64, macOS x64/arm64, Linux x64/arm64

## How it works

1. **Parse**: Parse JavaScript AST with SWC
2. **Transform**: Transform JS to Rust code
3. **Compile**: Compile Rust to native with Cranelift
4. **Link**: Link with servo-zero runtime
5. **Output**: Generate executable/library

## Related

- [javascript-web-to-rust-native](https://github.com/irisverse/javascript-web-to-rust-native) - Main project
- [servo-zero](https://github.com/irisverse/servo-zero) - Browser engine

## License

MIT
