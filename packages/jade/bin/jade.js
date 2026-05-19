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

  const command = args[0];
  
  if (command === 'link') {
    await handleLink(args.slice(1));
    return;
  }
  
  if (command === 'pack') {
    await handlePack(args.slice(1));
    return;
  }

  // 编译模式
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

async function handleLink(args) {
  let objPath = null;
  let libPath = null;
  let outputPath = 'app.exe';
  
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '-o' || arg === '--output') {
      outputPath = args[++i];
    } else if (arg === '-l' || arg === '--lib') {
      libPath = args[++i];
    } else if (!arg.startsWith('-')) {
      if (!objPath) {
        objPath = arg;
      } else if (!libPath) {
        libPath = arg;
      }
    }
  }
  
  if (!objPath) {
    console.error('❌ No object file specified');
    console.error('Usage: jade link <obj> [lib] -o <output>');
    process.exit(1);
  }
  
  const director = new Director();
  
  try {
    const result = await director.link(objPath, libPath || '', outputPath);
    console.log(`\n✅ Success!`);
    console.log(`Output: ${result}`);
  } catch (error) {
    console.error(`\n❌ Link failed:`);
    console.error(error.message);
    process.exit(1);
  }
}

async function handlePack(args) {
  let objPath = null;
  let libName = 'mylib';
  
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    if (arg === '-n' || arg === '--name') {
      libName = args[++i];
    } else if (!arg.startsWith('-')) {
      objPath = arg;
    }
  }
  
  if (!objPath) {
    console.error('❌ No object file specified');
    console.error('Usage: jade pack <obj> -n <name>');
    process.exit(1);
  }
  
  const director = new Director();
  
  try {
    const result = await director.packLib(objPath, libName);
    console.log(`\n✅ Success!`);
    console.log(`Output: ${result}`);
  } catch (error) {
    console.error(`\n❌ Pack failed:`);
    console.error(error.message);
    process.exit(1);
  }
}

function printHelp() {
  console.log(`
@irisverse/jade - JavaScript to Native compiler

Usage:
  jade <input.js> [options]    Compile JS to .obj
  jade link <obj> [lib] -o <output>   Link .obj to .exe
  jade pack <obj> -n <name>    Pack .obj to .lib/.a
  jade --help                  Show this help

Compile Options:
  -i, --input <file>    Input JavaScript file
  -n, --name <name>     Output name (default: app)
  -e, --embed           Embed mode (library, no window)

Link Options:
  -o, --output <file>   Output executable
  -l, --lib <file>      Library to link

Pack Options:
  -n, --name <name>     Library name

Examples:
  jade dist/assets/index.js -n my-app
  jade link my-app.obj servo-zero.lib -o my-app.exe
  jade pack my-app.obj -n mylib

No binary download required!
Uses Cranelift for code generation.
`);
}

main();
