const native = require('./native/jade-native.win32-x64-msvc.node');
const { JadeNative } = native;
const fs = require('fs');

console.log('=== rustc 编译方案测试 ===\n');

const compiler = new JadeNative();

// 测试 1：验证
console.log('1. 类型验证');
const validCode = `
fn add(a: i64, b: i64) -> i64 {
    a + b
}
`;

try {
    compiler.checkWithRustc(validCode);
    console.log('   ✅ 验证通过\n');
} catch (e) {
    console.log('   ❌ 验证失败:', e.message, '\n');
}

// 测试 2：错误代码验证
console.log('2. 错误代码验证');
const invalidCode = `
fn foo(x: i64) -> i64 {
    x + "string"
}
`;

try {
    compiler.checkWithRustc(invalidCode);
    console.log('   ✅ 验证通过\n');
} catch (e) {
    console.log('   ❌ 验证失败（预期）:', e.message.split('\n')[0], '\n');
}

// 测试 3：编译可执行文件
console.log('3. 编译 Native 可执行文件');
const mainCode = `
fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let result = add(1, 2);
    println!("Result: {}", result);
}
`;

try {
    const exe = compiler.compileWithRustc(mainCode, 'native');
    console.log('   ✅ 编译成功');
    console.log('   体积:', exe.length, '字节\n');
    
    // 保存可执行文件
    fs.writeFileSync('test_rustc.exe', exe);
    console.log('   ✅ 已保存: test_rustc.exe\n');
} catch (e) {
    console.log('   ❌ 编译失败:', e.message, '\n');
}

// 测试 4：性能对比
console.log('4. 性能测试 (编译 10 次)');
const start = Date.now();
for (let i = 0; i < 10; i++) {
    compiler.compileWithRustc(validCode + '\nfn main() {}', 'native');
}
const elapsed = Date.now() - start;
console.log('   总耗时:', elapsed, 'ms');
console.log('   平均:', (elapsed / 10).toFixed(1), 'ms/次\n');

console.log('=== 测试完成 ===');
