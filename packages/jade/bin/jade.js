#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { Director } = require('../src/director');

async function main() {
  const args = process.argv.slice(2);
  
  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    printHelp();
    return;
  }

  // 解析参数
  let inputFile = null;
  let outputName = 'app';
  let embed = false;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '--input' || arg === '-i') {
      inputFile = args[++i];
    } else if (arg === '--name' || arg === '-n') {
      outputName = args[++i];
    } else if (arg === '--embed' || arg === '-e') {
      embed = true;
    } else if (!arg.startsWith('-')) {
      inputFile = arg;
    }
  }

  if (!inputFile) {
    console.error('❌ No input file specified');
    console.error('Usage: jade <input.js> [options]');
    process.exit(1);
  }

  // 读取 JS 文件
  const jsCode = fs.readFileSync(inputFile, 'utf-8');
  console.log(`Input: ${inputFile} (${jsCode.length} bytes)\n`);

  // 编译
  const director = new Director();
  
  try {
    const outputPath = await director.compile(jsCode, {
      outputName,
      embed
    });
    
    console.log(`\n✅ Success!`);
    console.log(`Output: ${outputPath}`);
    
  } catch (error) {
    console.error(`\n❌ Compilation failed:`);
    console.error(error.message);
    process.exit(1);
  }
}

function printHelp() {
  console.log(`
@irisverse/jade - JavaScript to Native compiler

Usage:
  jade <input.js> [options]
  jade --help

Options:
  -i, --input <file>    Input JavaScript file
  -n, --name <name>     Output name (default: app)
  -e, --embed           Embed mode (library, no window)
  -h, --help            Show this help

Examples:
  jade dist/assets/index.js -n my-app
  jade input.js --embed -n my-lib

No binary download required!
Uses Cranelift WASM for compilation.
`);
}

main();
