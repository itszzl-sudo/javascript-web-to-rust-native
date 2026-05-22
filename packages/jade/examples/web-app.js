// 完整的 Web 应用示例

// 导航栏组件
function Navbar() {
  return {
    title: "My Web App",
    links: ["Home", "About", "Contact"],
    render: function() {
      console.log("Navbar rendered");
    }
  };
}

// 侧边栏组件
function Sidebar() {
  return {
    menu: ["Dashboard", "Settings", "Profile"],
    collapsed: false,
    render: function() {
      console.log("Sidebar rendered");
    }
  };
}

// 内容区组件
function Content() {
  return {
    title: "Welcome",
    text: "This is a compiled web application",
    render: function() {
      console.log("Content rendered");
    }
  };
}

// 主应用组件
function WebApp() {
  const navbar = Navbar();
  const sidebar = Sidebar();
  const content = Content();
  
  return {
    children: [navbar, sidebar, content],
    
    onMounted: function() {
      console.log("WebApp mounted");
    },
    
    onUpdated: function() {
      console.log("WebApp updated");
    },
    
    render: function() {
      console.log("WebApp rendered");
    }
  };
}

module.exports = { Navbar, Sidebar, Content, WebApp };
