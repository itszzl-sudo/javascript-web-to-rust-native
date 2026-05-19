/**
 * Jade Director - JS Implementation
 * 
 * 编译流程：JS → SWC 解析 → Rust IR → Cranelift WASM → .obj
 */

const { parse, transform } = require('@swc/core');
const path = require('path');
const fs = require('fs');

class Director {
  constructor() {
    this.workdir = process.cwd();
  }

  /**
   * 编译 JS 到原生代码
   */
  async compile(jsCode, options = {}) {
    const {
      outputName = 'app',
      embed = false,
      target = this.detectTarget()
    } = options;

    console.log('=== Jade Director: JS → Native ===\n');
    console.log(`Target: ${target}`);
    console.log(`Mode: ${embed ? 'embed' : 'window'}`);

    // 1. 解析 JS AST
    console.log('\n1. Parsing JavaScript...');
    const ast = await this.parseJS(jsCode);
    console.log(`✅ AST nodes: ${this.countNodes(ast)}`);

    // 2. 转换为 Rust IR
    console.log('\n2. Transforming to Rust IR...');
    const rustIR = await this.transformToRust(ast);
    console.log(`✅ IR functions: ${rustIR.functions.length}`);

    // 3. Cranelift 编译（WASM）
    console.log('\n3. Compiling with Cranelift...');
    const objBytes = await this.compileWithCranelift(rustIR, target);
    console.log(`✅ Object file: ${objBytes.length} bytes`);

    // 4. 保存输出
    const outputPath = path.join(this.workdir, `${outputName}.obj`);
    fs.writeFileSync(outputPath, objBytes);
    console.log(`\n✅ Output: ${outputPath}`);

    if (!embed) {
      console.log('\nNext steps:');
      console.log(`  Link with servo-zero to create ${outputName}.exe`);
    }

    return outputPath;
  }

  /**
   * 解析 JS 为 AST
   */
  async parseJS(code) {
    return parse(code, {
      syntax: 'ecmascript',
      target: 'es2020',
      module: true
    });
  }

  /**
   * 转换为 Rust IR
   */
  async transformToRust(ast) {
    const ir = {
      functions: [],
      globals: [],
      imports: []
    };

    // 遍历 AST 提取函数
    this.traverseAST(ast, (node) => {
      if (node.type === 'FunctionDeclaration') {
        ir.functions.push({
          name: node.identifier?.value || 'anonymous',
          params: node.params?.map(p => p.pat?.value) || [],
          body: this.extractBody(node.body)
        });
      }
      
      if (node.type === 'VariableDeclaration') {
        node.declarations?.forEach(decl => {
          ir.globals.push({
            name: decl.id?.value,
            init: this.extractValue(decl.init)
          });
        });
      }

      if (node.type === 'ImportDeclaration') {
        ir.imports.push({
          source: node.source?.value,
            specifiers: node.specifiers?.map(s => s.local?.value)
          });
        }
      });

    return ir;
  }

  /**
   * Cranelift 编译（调用 WASM）
   */
  async compileWithCranelift(ir, target) {
    // 加载 Cranelift WASM
    const cranelift = await this.loadCranelift();

    // 编译选项
    const options = {
      target: target,
      opt_level: 'speed',
      is_pic: true
    };

    // 编译每个函数
    const objParts = [];
    for (const func of ir.functions) {
      const bytes = await cranelift.compile_function(func, options);
      objParts.push(bytes);
    }

    // 合并为完整 .obj
    return this.linkObjectParts(objParts, target);
  }

  /**
   * 加载 Cranelift WASM 模块
   */
  async loadCranelift() {
    const wasmPath = path.join(__dirname, '..', 'wasm', 'cranelift.wasm');
    
    if (!fs.existsSync(wasmPath)) {
      // 如果 WASM 不存在，使用纯 JS 回退
      console.log('  ⚠️  Cranelift WASM not found, using JS fallback');
      return this.createJSFallback();
    }

    const wasmBuffer = fs.readFileSync(wasmPath);
    const wasmModule = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(wasmModule, {
      env: {
        memory: new WebAssembly.Memory({ initial: 256 })
      }
    });

    return instance.exports;
  }

  /**
   * 纯 JS 回退编译器（简化版）
   */
  createJSFallback() {
    return {
      compile_function: async (func, options) => {
        // 生成简化的 .obj 格式
        return Buffer.from(this.generateSimpleObject(func));
      }
    };
  }

  /**
   * 生成简化的对象文件
   */
  generateSimpleObject(func) {
    // COFF 格式（Windows）或 ELF 格式（Linux/macOS）
    const isWindows = process.platform === 'win32';
    
    if (isWindows) {
      return this.generateCOFF(func);
    } else {
      return this.generateELF(func);
    }
  }

  /**
   * 生成 COFF 格式
   */
  generateCOFF(func) {
    // 简化的 COFF 头
    const header = Buffer.alloc(20);
    header.writeUInt16LE(0x14c, 0);  // Machine: i386
    header.writeUInt16LE(1, 2);      // NumberOfSections
    header.writeUInt32LE(0, 4);      // TimeDateStamp
    header.writeUInt32LE(0, 8);      // PointerToSymbolTable
    header.writeUInt32LE(1, 12);     // NumberOfSymbols
    header.writeUInt16LE(0, 16);     // SizeOfOptionalHeader
    header.writeUInt16LE(0, 18);     // Characteristics

    // 代码段
    const code = this.generateCode(func);
    const section = Buffer.alloc(40 + code.length);
    section.write('.text\0\0\0', 0);
    section.writeUInt32LE(code.length, 16);  // SizeOfRawData
    section.writeUInt32LE(40, 20);           // PointerToRawData
    code.copy(section, 40);

    return Buffer.concat([header, section]);
  }

  /**
   * 生成 ELF 格式
   */
  generateELF(func) {
    // 简化的 ELF 头
    const header = Buffer.alloc(52);
    header.writeUInt8(0x7f, 0);
    header.write('ELF', 1);
    header.writeUInt8(2, 4);   // 64-bit
    header.writeUInt8(1, 5);   // Little endian
    header.writeUInt8(1, 6);   // ELF version
    header.writeUInt16LE(2, 16);  // Executable
    header.writeUInt16LE(0x3e, 18);  // x86-64

    const code = this.generateCode(func);
    return Buffer.concat([header, code]);
  }

  /**
   * 生成机器码（简化版）
   */
  generateCode(func) {
    // 简化的 x86-64 代码
    const code = [];
    
    // 函数序言
    code.push(0x55);  // push rbp
    code.push(0x48, 0x89, 0xe5);  // mov rbp, rsp
    
    // 函数体（简化：返回 0）
    code.push(0x48, 0x31, 0xc0);  // xor rax, rax
    
    // 函数尾声
    code.push(0x5d);  // pop rbp
    code.push(0xc3);  // ret

    return Buffer.from(code);
  }

  /**
   * 链接对象文件部分
   */
  linkObjectParts(parts, target) {
    return Buffer.concat(parts);
  }

  /**
   * 检测目标平台
   */
  detectTarget() {
    const platform = process.platform;
    const arch = process.arch;

    const targetMap = {
      'win32-x64': 'x86_64-pc-windows-msvc',
      'darwin-x64': 'x86_64-apple-darwin',
      'darwin-arm64': 'aarch64-apple-darwin',
      'linux-x64': 'x86_64-unknown-linux-gnu'
    };

    return targetMap[`${platform}-${arch}`] || 'x86_64-unknown-linux-gnu';
  }

  /**
   * AST 遍历
   */
  traverseAST(node, callback) {
    callback(node);
    
    for (const key in node) {
      if (node[key] && typeof node[key] === 'object') {
        if (Array.isArray(node[key])) {
          node[key].forEach(child => this.traverseAST(child, callback));
        } else {
          this.traverseAST(node[key], callback);
        }
      }
    }
  }

  /**
   * 统计 AST 节点数
   */
  countNodes(node) {
    let count = 0;
    this.traverseAST(node, () => count++);
    return count;
  }

  /**
   * 提取函数体
   */
  extractBody(body) {
    if (!body) return [];
    if (body.stmts) {
      return body.stmts.map(stmt => this.extractStatement(stmt));
    }
    return [];
  }

  /**
   * 提取语句
   */
  extractStatement(stmt) {
    return {
      type: stmt.type,
      ...stmt
    };
  }

  /**
   * 提取值
   */
  extractValue(node) {
    if (!node) return null;
    
    if (node.type === 'NumericLiteral') return node.value;
    if (node.type === 'StringLiteral') return node.value;
    if (node.type === 'BooleanLiteral') return node.value;
    
    return node;
  }
}

module.exports = { Director };
