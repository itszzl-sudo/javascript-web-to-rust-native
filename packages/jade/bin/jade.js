#!/usr/bin/env node

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

// 获取平台对应的二进制文件
function getBinary() {
  const platform = process.platform;
  const arch = process.arch;
  
  const binaryName = {
    win32: { x64: 'director.exe', ia32: 'director.exe' },
    darwin: { x64: 'director', arm64: 'director' },
    linux: { x64: 'director', arm64: 'director' }
  }[platform]?.[arch];
  
  if (!binaryName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }
  
  return path.join(__dirname, '..', 'binaries', platform, arch, binaryName);
}

// 主入口
function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    printHelp();
    return;
  }
  
  const binary = getBinary();
  
  if (!fs.existsSync(binary)) {
    console.error(`Binary not found: ${binary}`);
    console.error('Please run "npm install" to download the binary');
    process.exit(1);
  }
  
  try {
    execSync(`"${binary}" ${args.join(' ')}`, {
      stdio: 'inherit',
      env: { ...process.env }
    });
  } catch (error) {
    if (error.status !== 0) {
      process.exit(error.status);
    }
  }
}

function printHelp() {
  console.log(`
@irisverse/jade - JavaScript to Native compiler

Usage:
  jade <input.js> [options]
  jade build <input.js> [options]
  jade --help

Commands:
  build    Compile JavaScript to native executable (default)

Options:
  -o, --output <path>    Output directory (default: dist)
  -n, --name <name>      Output name (default: app)
  -e, --embed            Embed mode (generate library, no window)
  -h, --help             Show this help

Examples:
  # Compile Vue app to native executable
  jade build dist/assets/index.js -o ./output -n my-app

  # Generate embed library (for embedding in other programs)
  jade build dist/assets/index.js --embed -n my-lib

  # Quick compile
  jade dist/assets/index.js

For more information: https://github.com/irisverse/javascript-web-to-rust-native
`);
}

main();
