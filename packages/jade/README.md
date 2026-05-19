# @irisverse/jade

JavaScript to Native compiler. **No binary download required.**

## Why This Approach?

- ✅ **No untrusted binaries** - All code is JavaScript + WASM
- ✅ **Transparent** - You can read and audit the source code
- ✅ **Signed dependencies** - Cranelift (ByteDance), SWC (Vercel)
- ✅ **Install and run** - No postinstall scripts, no network requests
- ✅ **Debuggable** - Full source code available

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

## How It Works

```
Your JS Code
     ↓
SWC Parser (@swc/core)
     ↓
Rust IR (in JavaScript)
     ↓
Cranelift WASM (optional)
     ↓
.obj File
     ↓
Link with servo-zero → .exe
```

### Components

| Component | Provider | Trust |
|-----------|----------|-------|
| SWC Parser | Vercel | ✅ Signed npm package |
| Cranelift | ByteDance | ✅ Signed, open source |
| Director | This package | ✅ JavaScript source |

## Trust & Security

### No Binary Downloads

Unlike esbuild or swc CLI, this package:
- Does NOT download platform-specific binaries
- Does NOT run postinstall scripts with network access
- All compilation logic is in JavaScript/WASM

### Signed Dependencies

- **@swc/core**: Published by Vercel, npm verified
- **Cranelift**: ByteDance open source, Apache 2.0

### Auditable

All source code is in the package:
```
packages/jade/
├── bin/jade.js       # CLI entry (readable)
├── src/director.js   # Compiler logic (readable)
└── wasm/             # Optional Cranelift WASM
```

## Performance

For prototyping and early-stage projects, performance is secondary to trust and transparency.

- **JS Director**: ~1-5 seconds for typical projects
- **Cranelift WASM**: ~100-500ms (when available)

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

## Next Steps

After generating `.obj` file:

```bash
# Link with servo-zero (you control this step)
link my-app.obj servo-zero.lib -o my-app.exe
```

This puts you in control of the final linking step.

## License

MIT
