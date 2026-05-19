// Example: Simple Vue app
function createApp() {
  const state = reactive({
    count: 0,
    message: 'Hello World'
  });

  function increment() {
    state.count++;
  }

  return {
    state,
    increment
  };
}

function render() {
  return {
    tag: 'div',
    children: [
      { tag: 'h1', text: state.message },
      { tag: 'button', text: `Count: ${state.count}`, onClick: increment }
    ]
  };
}

module.exports = { createApp, render };
