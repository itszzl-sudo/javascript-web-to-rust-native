const https = require('https');
const fs = require('fs');
const path = require('path');
const zlib = require('zlib');
const { execSync } = require('child_process');

const PLATFORM = process.platform;
const ARCH = process.arch;

// 平台映射
const PLATFORM_MAP = {
  win32: 'windows',
  darwin: 'darwin',
  linux: 'linux'
};

const ARCH_MAP = {
  x64: 'x64',
  arm64: 'arm64',
  ia32: 'x86'
};

const TARGET = `${PLATFORM_MAP[PLATFORM]}-${ARCH_MAP[ARCH]}`;
const EXT = PLATFORM === 'win32' ? '.exe' : '';

// 获取版本
const packageJson = require('../package.json');
const VERSION = packageJson.version;

// 下载 URL
const BASE_URL = 'https://github.com/irisverse/javascript-web-to-rust-native/releases/download';
const URL = `${BASE_URL}/v${VERSION}/director-${TARGET}${EXT}.gz`;

const outputDir = path.join(__dirname, '..', 'binaries', PLATFORM, ARCH);
const outputPath = path.join(outputDir, `director${EXT}`);

console.log(`@irisverse/jade postinstall`);
console.log(`Platform: ${PLATFORM}-${ARCH}`);
console.log(`Target: ${TARGET}`);
console.log(`Downloading from: ${URL}`);

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function download() {
  ensureDir(outputDir);
  
  const file = fs.createWriteStream(outputPath);
  
  https.get(URL, (res) => {
    if (res.statusCode === 302 || res.statusCode === 301) {
      // 跟随重定向
      https.get(res.headers.location, downloadStream);
    } else if (res.statusCode === 200) {
      downloadStream(res);
    } else if (res.statusCode === 404) {
      console.error('');
      console.error('Binary not found for this version.');
      console.error('This might be a development version.');
      console.error('Please build from source:');
      console.error('  git clone https://github.com/irisverse/javascript-web-to-rust-native');
      console.error('  cd javascript-web-to-rust-native');
      console.error('  cargo build --release');
      process.exit(0);  // 不阻塞安装
    } else {
      console.error(`Download failed: HTTP ${res.statusCode}`);
      process.exit(1);
    }
  }).on('error', (err) => {
    console.error(`Download error: ${err.message}`);
    process.exit(1);
  });
}

function downloadStream(res) {
  const gunzip = zlib.createGunzip();
  
  res.pipe(gunzip).pipe(fs.createWriteStream(outputPath))
    .on('finish', () => {
      // 添加执行权限
      if (PLATFORM !== 'win32') {
        fs.chmodSync(outputPath, 0o755);
      }
      
      console.log('');
      console.log('✅ Binary downloaded successfully!');
      console.log(`Location: ${outputPath}`);
      console.log(`Size: ${(fs.statSync(outputPath).size / 1024 / 1024).toFixed(2)} MB`);
    })
    .on('error', (err) => {
      console.error(`Extract error: ${err.message}`);
      process.exit(1);
    });
}

download();
