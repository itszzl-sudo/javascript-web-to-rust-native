// Vue 3 组件测试
// 这是一个简化的 Vue 组件，用于测试编译器

function reactive(state) {
  return state;
}

function computed(getter) {
  return getter();
}

function ref(value) {
  return { value: value };
}

function watch(source, callback) {
  // 简化实现
}

function onMounted(hook) {
  // 简化实现
}

// 计数器组件
function Counter() {
  const count = ref(0);
  
  function increment() {
    count.value = count.value + 1;
  }
  
  function decrement() {
    count.value = count.value - 1;
  }
  
  return {
    count: count,
    increment: increment,
    decrement: decrement
  };
}

// 待办事项组件
function TodoApp() {
  const todos = ref([]);
  const newTodo = ref('');
  
  function addTodo() {
    if (newTodo.value.length > 0) {
      todos.value.push({
        id: Date.now(),
        text: newTodo.value,
        done: false
      });
      newTodo.value = '';
    }
  }
  
  function toggleTodo(id) {
    for (let i = 0; i < todos.value.length; i++) {
      if (todos.value[i].id == id) {
        todos.value[i].done = !todos.value[i].done;
      }
    }
  }
  
  function removeTodo(id) {
    const newTodos = [];
    for (let i = 0; i < todos.value.length; i++) {
      if (todos.value[i].id != id) {
        newTodos.push(todos.value[i]);
      }
    }
    todos.value = newTodos;
  }
  
  return {
    todos: todos,
    newTodo: newTodo,
    addTodo: addTodo,
    toggleTodo: toggleTodo,
    removeTodo: removeTodo
  };
}

// 导出
module.exports = {
  reactive: reactive,
  computed: computed,
  ref: ref,
  Counter: Counter,
  TodoApp: TodoApp
};
