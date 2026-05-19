// 测试更多 JS 语法支持

// 函数声明
function add(a, b) {
  return a + b;
}

// 变量声明
const x = 10;
let y = 20;

// 条件语句
function testIf(n) {
  if (n > 0) {
    return "positive";
  } else {
    return "negative";
  }
}

// 循环
function sum(n) {
  let total = 0;
  for (let i = 0; i < n; i++) {
    total = total + i;
  }
  return total;
}

// 类
class Calculator {
  add(a, b) {
    return a + b;
  }
  
  multiply(a, b) {
    return a * b;
  }
}

// 导出
module.exports = { add, testIf, sum, Calculator };
