const native = require('./native/jade-native.win32-x64-msvc.node');
const { JadeNative } = native;
const fs = require('fs');

console.log('=== 性能测试 ===\n');

const compiler = new JadeNative();

const testCode = `
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let result = fibonacci(20);
    println!("Fibonacci(20) = {}", result);
}
`;

// 测试 1：编译时间
console.log('1. 编译时间测试');
const times = [];
for (let i = 0; i < 5; i++) {
    const start = Date.now();
    try {
        compiler.compileWithRustc(testCode, 'native');
        const elapsed = Date.now() - start;
        times.push(elapsed);
        console.log(`   第 ${i+1} 次: ${elapsed}ms`);
    } catch (e) {
        console.log(`   第 ${i+1} 次: 失败 - ${e.message.split('\n')[0]}`);
    }
}

if (times.length > 0) {
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    console.log(`   平均: ${avg.toFixed(0)}ms\n`);
}

// 测试 2：验证时间
console.log('2. 验证时间测试');
const validateTimes = [];
for (let i = 0; i < 10; i++) {
    const start = Date.now();
    try {
        compiler.checkWithRustc(testCode);
        const elapsed = Date.now() - start;
        validateTimes.push(elapsed);
    } catch (e) {}
}
const validateAvg = validateTimes.reduce((a, b) => a + b, 0) / validateTimes.length;
console.log(`   验证平均: ${validateAvg.toFixed(0)}ms (${validateTimes.length}次)\n`);

// 测试 3：代码复杂度
console.log('3. 代码复杂度测试');
const complexCode = `
struct Point { x: f64, y: f64 }
struct Circle { center: Point, radius: f64 }
struct Rectangle { x: f64, y: f64, w: f64, h: f64 }

impl Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

fn main() {
    let c = Circle { center: Point { x: 0.0, y: 0.0 }, radius: 5.0 };
    let r = Rectangle { x: 0.0, y: 0.0, w: 10.0, h: 5.0 };
    println!("Circle: {}, Rect: {}", c.area(), r.area());
}
`;

try {
    const start = Date.now();
    const exe = compiler.compileWithRustc(complexCode, 'native');
    const elapsed = Date.now() - start;
    console.log(`   ✅ 复杂代码编译成功: ${elapsed}ms`);
    console.log(`   体积: ${(exe.length / 1024).toFixed(1)} KB\n`);
} catch (e) {
    console.log(`   ❌ 失败: ${e.message.split('\n')[0]}\n`);
}

console.log('=== 测试完成 ===');
