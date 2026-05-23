// 用户登录 Vue 工程
// 完整的登录、注册、表单验证功能

// 响应式状态
function reactive(state) {
  return state;
}

function ref(value) {
  return { value: value };
}

// 登录表单组件
function LoginForm() {
  const username = ref('');
  const password = ref('');
  const remember = ref(false);
  const loading = ref(false);
  const error = ref('');
  
  function validateEmail(email) {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email);
  }
  
  function handleLogin() {
    loading.value = true;
    error.value = '';
    
    // 表单验证
    if (!username.value) {
      error.value = '请输入用户名';
      loading.value = false;
      return;
    }
    
    if (!password.value) {
      error.value = '请输入密码';
      loading.value = false;
      return;
    }
    
    if (password.value.length < 6) {
      error.value = '密码至少 6 位';
      loading.value = false;
      return;
    }
    
    // 模拟登录请求
    setTimeout(function() {
      if (username.value === 'admin' && password.value === '123456') {
        alert('登录成功！欢迎 ' + username.value);
        loading.value = false;
      } else {
        error.value = '用户名或密码错误';
        loading.value = false;
      }
    }, 1000);
  }
  
  function handleReset() {
    username.value = '';
    password.value = '';
    remember.value = false;
    error.value = '';
  }
  
  return {
    username: username,
    password: password,
    remember: remember,
    loading: loading,
    error: error,
    handleLogin: handleLogin,
    handleReset: handleReset
  };
}

// 注册表单组件
function RegisterForm() {
  const username = ref('');
  const email = ref('');
  const password = ref('');
  const confirmPassword = ref('');
  const agree = ref(false);
  const loading = ref(false);
  const error = ref('');
  
  function handleRegister() {
    loading.value = true;
    error.value = '';
    
    // 验证
    if (!username.value || username.value.length < 3) {
      error.value = '用户名至少 3 个字符';
      loading.value = false;
      return;
    }
    
    if (!email.value) {
      error.value = '请输入邮箱';
      loading.value = false;
      return;
    }
    
    if (!password.value || password.value.length < 6) {
      error.value = '密码至少 6 位';
      loading.value = false;
      return;
    }
    
    if (password.value !== confirmPassword.value) {
      error.value = '两次密码不一致';
      loading.value = false;
      return;
    }
    
    if (!agree.value) {
      error.value = '请同意用户协议';
      loading.value = false;
      return;
    }
    
    // 模拟注册
    setTimeout(function() {
      alert('注册成功！请登录');
      loading.value = false;
    }, 1000);
  }
  
  return {
    username: username,
    email: email,
    password: password,
    confirmPassword: confirmPassword,
    agree: agree,
    loading: loading,
    error: error,
    handleRegister: handleRegister
  };
}

// 密码强度检测
function checkPasswordStrength(password) {
  let strength = 0;
  
  if (password.length >= 6) strength = strength + 1;
  if (password.length >= 10) strength = strength + 1;
  if (/[A-Z]/.test(password)) strength = strength + 1;
  if (/[a-z]/.test(password)) strength = strength + 1;
  if (/[0-9]/.test(password)) strength = strength + 1;
  if (/[^A-Za-z0-9]/.test(password)) strength = strength + 1;
  
  if (strength <= 2) return '弱';
  if (strength <= 4) return '中';
  return '强';
}

// 用户状态管理
function useUserStore() {
  const user = ref(null);
  const token = ref('');
  const isAuthenticated = ref(false);
  
  function login(userData, userToken) {
    user.value = userData;
    token.value = userToken;
    isAuthenticated.value = true;
  }
  
  function logout() {
    user.value = null;
    token.value = '';
    isAuthenticated.value = false;
  }
  
  function updateProfile(newData) {
    if (user.value) {
      user.value = Object.assign({}, user.value, newData);
    }
  }
  
  return {
    user: user,
    token: token,
    isAuthenticated: isAuthenticated,
    login: login,
    logout: logout,
    updateProfile: updateProfile
  };
}

// 导出
module.exports = {
  LoginForm: LoginForm,
  RegisterForm: RegisterForm,
  useUserStore: useUserStore,
  checkPasswordStrength: checkPasswordStrength
};
