// 测试组件嵌套和生命周期

// 子组件：按钮
function Button(text, onClick) {
  return {
    text: text,
    onClick: onClick,
    render: function() {
      console.log("Button render:", this.text);
    }
  };
}

// 子组件：输入框
function Input(placeholder, value) {
  return {
    placeholder: placeholder,
    value: value,
    render: function() {
      console.log("Input render:", this.placeholder);
    }
  };
}

// 父组件：表单
function Form() {
  // 嵌套子组件
  const submitBtn = Button("Submit", function() {
    console.log("Form submitted!");
  });
  
  const nameInput = Input("Enter name", "");
  const emailInput = Input("Enter email", "");
  
  return {
    children: [nameInput, emailInput, submitBtn],
    
    onMounted: function() {
      console.log("Form mounted");
    },
    
    onUpdated: function() {
      console.log("Form updated");
    },
    
    onUnmounted: function() {
      console.log("Form unmounted");
    },
    
    render: function() {
      console.log("Form render with", this.children.length, "children");
      for (let i = 0; i < this.children.length; i++) {
        this.children[i].render();
      }
    }
  };
}

// 父组件：应用
function App() {
  const form = Form();
  
  return {
    children: [form],
    
    onMounted: function() {
      console.log("App mounted");
    },
    
    onUpdated: function() {
      console.log("App updated");
    },
    
    onUnmounted: function() {
      console.log("App unmounted");
    },
    
    render: function() {
      console.log("App render");
      this.children[0].render();
    }
  };
}

module.exports = { Button, Input, Form, App };
