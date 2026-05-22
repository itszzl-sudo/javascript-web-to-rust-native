// Test all JavaScript syntax features

// 1. Function Declaration
function add(a, b) {
    return a + b;
}

// 2. Arrow Function
const multiply = (a, b) => a * b;
const subtract = (a, b) => {
    return a - b;
};

// 3. Async Function
async function fetchData(url) {
    return fetch(url);
}

// 4. Class
class Calculator {
    constructor(value) {
        this.value = value;
    }
    
    add(n) {
        this.value += n;
        return this;
    }
    
    getValue() {
        return this.value;
    }
}

// 5. Try-Catch
function safeDivide(a, b) {
    try {
        if (b === 0) throw new Error('Division by zero');
        return a / b;
    } catch (error) {
        console.error(error.message);
        return null;
    }
}

// 6. Switch Statement
function getColor(type) {
    switch (type) {
        case 'primary':
            return 'blue';
        case 'secondary':
            return 'gray';
        case 'success':
            return 'green';
        case 'danger':
            return 'red';
        default:
            return 'black';
    }
}

// 7. Do-While Loop
function countdown(n) {
    let result = [];
    do {
        result.push(n);
        n--;
    } while (n > 0);
    return result;
}

// 8. For-In Loop
function getKeys(obj) {
    let keys = [];
    for (let key in obj) {
        keys.push(key);
    }
    return keys;
}

// 9. For-Of Loop
function sumArray(arr) {
    let sum = 0;
    for (let num of arr) {
        sum += num;
    }
    return sum;
}

// 10. Destructuring
function destructure(obj) {
    const { name, age } = obj;
    const [first, second] = [1, 2];
    return { name, age, first, second };
}

// 11. Spread Operator
function merge(obj1, obj2) {
    return { ...obj1, ...obj2 };
}

function concat(arr1, arr2) {
    return [...arr1, ...arr2];
}

// 12. Template Literal
function greet(name) {
    return `Hello, ${name}!`;
}

// 13. Array Methods
function arrayOperations() {
    const arr = [1, 2, 3, 4, 5];
    
    const doubled = arr.map(x => x * 2);
    const filtered = arr.filter(x => x > 2);
    const reduced = arr.reduce((sum, x) => sum + x, 0);
    
    return { doubled, filtered, reduced };
}

// 14. Object Methods
function objectMethods() {
    const obj = { a: 1, b: 2, c: 3 };
    
    const keys = Object.keys(obj);
    const values = Object.values(obj);
    const entries = Object.entries(obj);
    
    return { keys, values, entries };
}

// 15. Labeled Statement
function labeledBreak() {
    outer: for (let i = 0; i < 3; i++) {
        for (let j = 0; j < 3; j++) {
            if (i === 1 && j === 1) {
                break outer;
            }
        }
    }
}

// 16. Debugger Statement
function debug() {
    debugger;
    return 'debugged';
}

console.log('✅ All syntax features loaded');
