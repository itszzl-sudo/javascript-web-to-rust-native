const fs = require('fs');
const path = require('path');

const platform = process.platform;
const arch = process.arch;

let moduleName;
if (platform === 'win32') {
  moduleName = 'jade_native.dll';
} else if (platform === 'darwin') {
  moduleName = 'libjade_native.dylib';
} else {
  moduleName = 'libjade_native.so';
}

const sourcePath = path.join(__dirname, '..', '..', '..', 'target', 'release', moduleName);
const targetDir = path.join(__dirname, '..', 'native');
const targetPath = path.join(targetDir, `jade-native.${platform}-${arch}${platform === 'win32' ? '-msvc' : ''}.node`);

if (!fs.existsSync(sourcePath)) {
  console.error(`Native module not found: ${sourcePath}`);
  console.error('Please run: cargo build --release -p jade-native');
  process.exit(1);
}

fs.mkdirSync(targetDir, { recursive: true });
fs.copyFileSync(sourcePath, targetPath);

console.log(`✅ Copied native module to: ${targetPath}`);
console.log(`   Size: ${(fs.statSync(targetPath).size / 1024 / 1024).toFixed(2)} MB`);
