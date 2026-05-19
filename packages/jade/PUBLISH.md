# 发布指南

## 发布到 npm

### 前提条件
- npm 账号（需要 `@irisverse` 组织权限）
- 已登录：`npm login`

### 发布步骤

```bash
cd packages/jade

# 1. 检查包内容
npm pack --dry-run

# 2. 发布（需要权限）
npm publish --access public

# 或使用 beta 标签
npm publish --access public --tag beta
```

## 发布后验证

```bash
# 安装测试
npm install -g @irisverse/jade

# 运行测试
jade --help
jade examples/hello.js -n hello
```

## 版本更新

```bash
# 更新版本号
npm version patch  # 0.1.0 → 0.1.1
npm version minor  # 0.1.0 → 0.2.0
npm version major  # 0.1.0 → 1.0.0

# 发布新版本
npm publish --access public
```

## 包内容

```
@irisverse/jade@0.1.0 (5.0 MB)
├── bin/jade.js                    # CLI 入口
├── src/director.js                # Director
├── native/*.node                  # Rust native 模块 (2.8 MB)
├── toolchain/win32-x64-msvc/      # MSVC 工具链 (8.1 MB)
└── README.md
```

## 注意事项

1. **Scoped Package**: 需要 `@irisverse` 组织权限
2. **Binary Files**: 包含平台特定的 .node 和 .exe 文件
3. **Toolchain**: MSVC 工具链仅支持 Windows x64
4. **Access**: 必须使用 `--access public`（scoped package 默认 private）

## CI/CD 自动发布（可选）

创建 `.github/workflows/publish.yml`:

```yaml
name: Publish to npm

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      
      - run: npm ci
      - run: npm run build:native
      - run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```
