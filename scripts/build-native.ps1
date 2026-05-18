# Vue 项目打包优化 + Director 编译脚本 (PowerShell)
# 用法: .\build-native.ps1 -VueProject <path> [-OutputName <name>]

param(
    [Parameter(Mandatory=$true)]
    [string]$VueProject,
    
    [string]$OutputName = "app"
)

$ErrorActionPreference = "Stop"
$WorkDir = $PWD

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  Vue → Native 构建流程" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "Vue 项目: $VueProject"
Write-Host "输出名称: $OutputName"
Write-Host ""

# Step 1: Vue 项目打包优化
Write-Host "=== Step 1: Vue 项目打包优化 ===" -ForegroundColor Yellow
Write-Host ""

Set-Location $VueProject

# 检查 package.json
if (-not (Test-Path "package.json")) {
    Write-Host "❌ 未找到 package.json" -ForegroundColor Red
    exit 1
}

# 生成优化配置
Write-Host "生成 Vite 优化配置（禁用 eval、new Function）..."

$viteOptimizeConfig = @"
import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        target: 'es2015',
        minify: 'terser',
        terserOptions: {
            compress: {
                evaluate: false,
                negate_iife: false,
            },
            output: {
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
        legalComments: 'none',
    }
})
"@

Set-Content -Path "vite.optimize.config.js" -Value $viteOptimizeConfig -Encoding UTF8
Write-Host "✅ 已生成 vite.optimize.config.js" -ForegroundColor Green

# 安装依赖
Write-Host "检查依赖..."
if (-not (Test-Path "node_modules")) {
    npm install
}

# 执行构建
Write-Host "执行打包..."
npm run build -- --config vite.optimize.config.js

Write-Host "✅ Vue 项目打包完成" -ForegroundColor Green
Write-Host ""

# Step 2: 查找生成的 JS 文件
Write-Host "=== Step 2: 查找打包结果 ===" -ForegroundColor Yellow
Write-Host ""

$DistJs = $null
if (Test-Path "dist/assets") {
    $DistJs = Get-ChildItem "dist/assets/*.js" | Select-Object -First 1
}

if ($null -eq $DistJs) {
    if (Test-Path "dist") {
        $DistJs = Get-ChildItem "dist/*.js" | Select-Object -First 1
    }
}

if ($null -eq $DistJs) {
    Write-Host "❌ 未找到打包后的 JS 文件" -ForegroundColor Red
    exit 1
}

Write-Host "找到 JS 文件: $($DistJs.FullName)"

# 检查禁用特性
Write-Host ""
Write-Host "检查禁用特性..."
$JsContent = Get-Content $DistJs.FullName -Raw

if ($JsContent -match "eval\(") {
    Write-Host "⚠️  警告: 发现 eval 调用" -ForegroundColor Yellow
}
if ($JsContent -match "new Function\(") {
    Write-Host "⚠️  警告: 发现 new Function 调用" -ForegroundColor Yellow
}

Write-Host "✅ JS 文件准备完成" -ForegroundColor Green
Write-Host ""

# Step 3: 调用 Director 编译
Write-Host "=== Step 3: Director 编译为 Native ===" -ForegroundColor Yellow
Write-Host ""

Set-Location $WorkDir

# 检查 director CLI
$DirectorCli = ".\target\release\director.exe"
if (-not (Test-Path $DirectorCli)) {
    Write-Host "编译 Director..."
    cargo build --release -p director
}

# 创建临时 JS 文件
$TempJs = Join-Path $WorkDir "temp_input.js"
Copy-Item $DistJs.FullName $TempJs

# 调用 director
Write-Host "调用 Director 编译..."
& $DirectorCli --input $TempJs --name $OutputName --output "dist\$OutputName"

# 清理临时文件
Remove-Item $TempJs -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  ✅ 构建完成" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "输出文件: dist\$OutputName\$OutputName.exe"
Write-Host ""
