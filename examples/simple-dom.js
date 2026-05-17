// Simple DOM manipulation example for JRust Translator

const counter = 0;

function increment() {
  const current = counter + 1;
  console.log('Counter:', current);
  return current;
}

function createDiv() {
  const div = document.createElement('div');
  div.id = 'test-div';
  div.textContent = 'Hello from JRust!';
  document.body.appendChild(div);
  console.log('Div created');
}

const result = increment();
console.log('Result:', result);
