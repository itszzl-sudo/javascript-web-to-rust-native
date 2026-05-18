#!/bin/bash
# Vue 项目打包优化 + Director 编译脚本
# 用法: ./build-native.sh <vue-project-path> [output-name]

set -e

if [ -z "$1" ]; then
    echo "用法: $0 <vue-project-path> [output-name]"
    echo "示例: $0 ./my-vue-app my-app"
    exit 1
fi

VUE_PROJECT="$1"
OUTPUT_NAME="${2:-app}"
WORK_DIR="$(pwd)"

echo "========================================="
echo "  Vue → Native 构建流程"
echo "========================================="
echo "Vue 项目: $VUE_PROJECT"
echo "输出名称: $OUTPUT_NAME"
echo ""

# Step 1: Vue 项目打包优化
echo "=== Step 1: Vue 项目打包优化 ==="
echo ""

cd "$VUE_PROJECT"

# 检查 package.json
if [ ! -f "package.json" ]; then
    echo "❌ 未找到 package.json"
    exit 1
fi

# 优化 vite.config.js/ts
echo "优化 Vite 配置（禁用 eval、new Function）..."
if [ -f "vite.config.js" ]; then
    VITE_CONFIG="vite.config.js"
elif [ -f "vite.config.ts" ]; then
    VITE_CONFIG="vite.config.ts"
else
    echo "⚠️  未找到 vite.config，使用默认配置"
    VITE_CONFIG=""
fi

# 注入优化配置
if [ -n "$VITE_CONFIG" ]; then
    cat > vite.optimize.config.js << 'EOF'
import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        target: 'es2015',
        minify: 'terser',
        terserOptions: {
            compress: {
                // 禁用 eval
                evaluate: false,
                // 禁用 new Function
                negate_iife: false,
            },
            output: {
                // 移除 eval 和 new Function
                beautify: false,
            }
        },
        rollupOptions: {
            output: {
                format: 'es',
                manualChunks: undefined,
            }
        }
    },
    esbuild: {
        // 禁用 eval 优化
        legalComments: 'none',
    }
})
EOF
    echo "✅ 已生成 vite.optimize.config.js"
fi

# 安装依赖
echo "检查依赖..."
if [ ! -d "node_modules" ]; then
    npm install
fi

# 执行构建
echo "执行打包..."
if [ -f "vite.optimize.config.js" ]; then
    npm run build -- --config vite.optimize.config.js
else
    npm run build
fi

echo "✅ Vue 项目打包完成"
echo ""

# Step 2: 查找生成的 JS 文件
echo "=== Step 2: 查找打包结果 ==="
echo ""

DIST_JS=""
if [ -d "dist/assets" ]; then
    for file in dist/assets/*.js; do
        if [ -f "$file" ]; then
            DIST_JS="$file"
            echo "找到 JS 文件: $DIST_JS"
            break
        fi
    done
fi

if [ -z "$DIST_JS" ]; then
    # 尝试直接 dist 目录
    for file in dist/*.js; do
        if [ -f "$file" ]; then
            DIST_JS="$file"
            echo "找到 JS 文件: $DIST_JS"
            break
        fi
    done
fi

if [ -z "$DIST_JS" ]; then
    echo "❌ 未找到打包后的 JS 文件"
    exit 1
fi

# 检查是否包含 eval/new Function
echo ""
echo "检查禁用特性..."
if grep -q "eval(" "$DIST_JS"; then
    echo "⚠️  警告: 发现 eval 调用"
fi
if grep -q "new Function(" "$DIST_JS"; then
    echo "⚠️  警告: 发现 new Function 调用"
fi

echo "✅ JS 文件准备完成"
echo ""

# Step 3: 调用 Director 编译
echo "=== Step 3: Director 编译为 Native ==="
echo ""

cd "$WORK_DIR"

# 读取 JS 内容
JS_CONTENT=$(cat "$VUE_PROJECT/$DIST_JS")
echo "JS 代码大小: $(echo "$JS_CONTENT" | wc -c) 字节"

# 检查 director CLI 是否存在
DIRECTOR_CLI="./target/release/director"
if [ ! -f "$DIRECTOR_CLI" ]; then
    echo "编译 Director..."
    cargo build --release -p director
fi

# 调用 director
echo "调用 Director 编译..."
echo "$JS_CONTENT" | "$DIRECTOR_CLI" --name "$OUTPUT_NAME" --output "dist/$OUTPUT_NAME"

echo ""
echo "========================================="
echo "  ✅ 构建完成"
echo "========================================="
echo "输出文件: dist/$OUTPUT_NAME/$OUTPUT_NAME.exe"
echo ""
