// 编译 user-login.js 为 Native
const { Director } = require('../src/director');

async function main() {
  const director = new Director();
  
  const code = `
function LoginForm() {
  const username = ref('');
  const password = ref('');
  const remember = ref(false);
  return { username, password, remember };
}

function RegisterForm() {
  const username = ref('');
  const email = ref('');
  const password = ref('');
  const confirmPassword = ref('');
  const agree = ref(false);
  return { username, email, password, confirmPassword, agree };
}

function useUserStore() {
  const user = ref(null);
  const token = ref('');
  const isAuthenticated = ref(false);
  return { user, token, isAuthenticated };
}

function checkPasswordStrength(password) {
  let strength = 0;
  if (password.length >= 6) strength = strength + 1;
  if (password.length >= 10) strength = strength + 1;
  return strength;
}
`;
  
  await director.compile(code, { outputName: 'app' });
}

main().catch(console.error);
