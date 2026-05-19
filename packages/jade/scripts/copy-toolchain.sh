#!/bin/bash

# 拷贝 MSVC 工具链文件

set -e

# 查找 MSVC 安装路径
MSVC_PATH=$(find "/c/Program Files/Microsoft Visual Studio" -path "*/VC/Tools/MSVC/*/bin/Hostx64/x64" 2>/dev/null | head -1)

if [ -z "$MSVC_PATH" ]; then
    echo "❌ MSVC not found"
    echo "Please install Visual Studio Build Tools"
    exit 1
fi

echo "Found MSVC: $MSVC_PATH"

TARGET_DIR="packages/jade/toolchain/win32-x64-msvc"
mkdir -p "$TARGET_DIR"

# 核心文件
FILES=(
    "link.exe"
    "link.exe.config"
    "lib.exe"
    "mspdb140.dll"
    "mspdbcore.dll"
    "mspdbst.dll"
    "mspdbsrv.exe"
    "mspdbcmf.exe"
)

echo "Copying files..."
for file in "${FILES[@]}"; do
    if [ -f "$MSVC_PATH/$file" ]; then
        cp "$MSVC_PATH/$file" "$TARGET_DIR/"
        echo "  ✅ $file"
    else
        echo "  ⚠️  $file not found"
    fi
done

# 拷贝必要的 DLL
DLLS=(
    "vcruntime140.dll"
    "vcruntime140_1.dll"
    "msvcp140.dll"
    "concrt140.dll"
)

# 查找 UCRT (Universal C Runtime)
UCRT_PATH=$(find "/c/Program Files (x86)/Windows Kits/10" -path "*/bin/*/x64/ucrtbase.dll" 2>/dev/null | head -1 | xargs dirname)

if [ -n "$UCRT_PATH" ]; then
    echo "Found UCRT: $UCRT_PATH"
    
    cp "$UCRT_PATH/ucrtbase.dll" "$TARGET_DIR/" 2>/dev/null && echo "  ✅ ucrtbase.dll" || true
    
    for dll in "${DLLS[@]}"; do
        if [ -f "$UCRT_PATH/$dll" ]; then
            cp "$UCRT_PATH/$dll" "$TARGET_DIR/" 2>/dev/null && echo "  ✅ $dll" || true
        fi
    done
fi

echo ""
echo "✅ Toolchain copied to: $TARGET_DIR"
echo ""
echo "Files:"
ls -lh "$TARGET_DIR"
