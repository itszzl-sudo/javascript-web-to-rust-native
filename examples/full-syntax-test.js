// 完整 JavaScript 语法测试

// ========== 基础语法 ==========

// 1. 函数声明
function add(a, b) {
    return a + b;
}

// 2. 函数表达式
const subtract = function(a, b) {
    return a - b;
};

// 3. 箭头函数
const multiply = (a, b) => a * b;
const divide = (a, b) => {
    return a / b;
};

// 4. Async 函数
async function fetchData(url) {
    const response = await fetch(url);
    return response.json();
}

// ========== 控制流 ==========

// 5. If-Else
function testIf(x) {
    if (x > 0) {
        return 'positive';
    } else if (x < 0) {
        return 'negative';
    } else {
        return 'zero';
    }
}

// 6. Switch
function getColor(type) {
    switch (type) {
        case 'primary':
            return 'blue';
        case 'secondary':
            return 'gray';
        case 'success':
            return 'green';
        default:
            return 'black';
    }
}

// 7. While
function countDown(n) {
    let result = [];
    while (n > 0) {
        result.push(n);
        n--;
    }
    return result;
}

// 8. Do-While
function repeatUntil(n) {
    let result = [];
    do {
        result.push(n);
        n--;
    } while (n > 0);
    return result;
}

// 9. For
function sumTo(n) {
    let sum = 0;
    for (let i = 1; i <= n; i++) {
        sum += i;
    }
    return sum;
}

// 10. For-In
function getKeys(obj) {
    let keys = [];
    for (let key in obj) {
        keys.push(key);
    }
    return keys;
}

// 11. For-Of
function doubleArray(arr) {
    let result = [];
    for (let item of arr) {
        result.push(item * 2);
    }
    return result;
}

// ========== 类和对象 ==========

// 12. Class
class Calculator {
    constructor(initialValue) {
        this.value = initialValue;
    }
    
    add(n) {
        this.value += n;
        return this;
    }
    
    subtract(n) {
        this.value -= n;
        return this;
    }
    
    getValue() {
        return this.value;
    }
}

// 13. 继承
class AdvancedCalculator extends Calculator {
    multiply(n) {
        this.value *= n;
        return this;
    }
    
    divide(n) {
        if (n !== 0) {
            this.value /= n;
        }
        return this;
    }
}

// ========== 错误处理 ==========

// 14. Try-Catch-Finally
function safeDivide(a, b) {
    try {
        if (b === 0) {
            throw new Error('Division by zero');
        }
        return a / b;
    } catch (error) {
        console.error(error.message);
        return null;
    } finally {
        console.log('Operation completed');
    }
}

// ========== 表达式 ==========

// 15. 三元运算符
function abs(x) {
    return x >= 0 ? x : -x;
}

// 16. 逻辑运算
function validate(input) {
    return input && input.length > 0 && input.trim() !== '';
}

// 17. 空值合并
function getOrDefault(value, defaultValue) {
    return value ?? defaultValue;
}

// 18. 可选链
function getNestedValue(obj) {
    return obj?.nested?.value;
}

// 19. 赋值运算符
function compoundAssignment(x) {
    x += 5;
    x -= 3;
    x *= 2;
    x /= 4;
    x %= 3;
    return x;
}

// 20. 更新表达式
function incrementDecrement(x) {
    x++;
    ++x;
    x--;
    --x;
    return x;
}

// ========== 数组和对象 ==========

// 21. 解构赋值
function destructuring() {
    const [a, b, c] = [1, 2, 3];
    const { x, y, z } = { x: 1, y: 2, z: 3 };
    return { a, b, c, x, y, z };
}

// 22. Spread 操作符
function spreadOperator() {
    const arr1 = [1, 2, 3];
    const arr2 = [4, 5, 6];
    const combined = [...arr1, ...arr2];
    
    const obj1 = { a: 1, b: 2 };
    const obj2 = { c: 3, d: 4 };
    const merged = { ...obj1, ...obj2 };
    
    return { combined, merged };
}

// 23. 模板字符串
function greet(name) {
    return `Hello, ${name}! Welcome to Jade.`;
}

// ========== 数组方法 ==========

// 24. Array Methods
function arrayMethods(arr) {
    const mapped = arr.map(x => x * 2);
    const filtered = arr.filter(x => x > 5);
    const reduced = arr.reduce((sum, x) => sum + x, 0);
    const found = arr.find(x => x === 10);
    const index = arr.findIndex(x => x === 10);
    
    return { mapped, filtered, reduced, found, index };
}

// ========== 其他语法 ==========

// 25. 标签语句
function labeledBreak() {
    let result = [];
    outer: for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
            if (i === 1 && j === 1) {
                break outer;
            }
            result.push([i, j]);
        }
    }
    return result;
}

// 26. New 表达式
function createInstance() {
    const calc = new Calculator(10);
    return calc.getValue();
}

// 27. 序列表达式
function sequence() {
    let x = 0;
    return (x++, x++, x++);
}

// 28. Generator 函数
function* numberGenerator() {
    yield 1;
    yield 2;
    yield 3;
}

console.log('✅ 所有语法特性已加载');
