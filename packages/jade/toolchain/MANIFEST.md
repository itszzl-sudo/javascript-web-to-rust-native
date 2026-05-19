# MSVC 工具链文件清单

## 当前文件 (packages/jade/toolchain/win32-x64-msvc/)

| 文件 | 大小 | 用途 | 必需 |
|------|------|------|------|
| link.exe | 3.6 MB | MSVC 链接器 | ✅ 是 |
| lib.exe | 23 KB | 静态库打包工具 | ⚠️ 可选 |
| mspdbcore.dll | 1.4 MB | PDB 核心库 | ✅ 是 |
| mspdb140.dll | 378 KB | PDB 支持 | ✅ 是 |
| msobj140.dll | - | MSVC 对象支持 | ✅ 是 |
| mspdbst.dll | - | PDB 存储 | ⚠️ 推荐 |
| mspdbsrv.exe | 188 KB | PDB 服务器 | ⚠️ 推荐 |
| ucrtbase.dll | 1.1 MB | UCRT 运行时 | ⚠️ 推荐 |

**总计**: ~6.6 MB

## 依赖说明

### 已拷贝（工具链内）
- mspdbcore.dll, mspdb140.dll - PDB 调试信息支持
- msobj140.dll - 对象文件支持
- ucrtbase.dll - Universal C Runtime

### 系统提供（不拷贝）
- **.NET Framework 4.0+** - 必需，由系统安装
  - 检测路径: `C:\Windows\Microsoft.NET\Framework\v4.0.30319\clr.dll`
  - 支持版本: 4.0, 4.5, 4.5.1, 4.5.2, 4.6, 4.6.1, 4.6.2, 4.7, 4.7.1, 4.7.2, 4.8
  
- **Windows API** - 系统自带
  - kernel32.dll, user32.dll 等

## 是否有多余文件？

### 必需文件（不可删除）
```
link.exe          # 核心链接器
mspdbcore.dll     # PDB 核心
mspdb140.dll      # PDB 支持
msobj140.dll      # 对象支持
```

### 可选文件（可删除但影响功能）
```
lib.exe           # 用于打包静态库 (.lib)
mspdbsrv.exe      # PDB 服务器（多进程调试）
mspdbst.dll       # PDB 存储（增强调试）
ucrtbase.dll      # UCRT（可能系统已有）
```

### 建议保留
**全部保留**（仅 6.6 MB），完整功能支持。

## .NET Framework 支持列表

| 版本 | Release 值 | 检测方式 |
|------|-----------|---------|
| 4.8 | 528040+ | 注册表 |
| 4.7.2 | 461808+ | 注册表 |
| 4.7.1 | 461308+ | 注册表 |
| 4.7 | 460798+ | 注册表 |
| 4.6.2 | 394802+ | 注册表 |
| 4.6.1 | 394254+ | 注册表 |
| 4.6 | 393295+ | 注册表 |
| 4.5.2 | 379893+ | 注册表 |
| 4.5.1 | 378675+ | 注册表 |
| 4.5 | 378389+ | 注册表 |
| 4.0 | - | 文件系统/注册表 |

**推荐**: .NET Framework 4.8（最新，性能最佳）

## 工作原理

```
用户运行 jade link hello.obj
        ↓
检查工具链目录
        ↓
检查 .NET Framework (4.0-4.8)
        ↓
    [已安装] → 使用工具链 link.exe
        ↓
    [未安装] → 报错 + 打开下载页面
```

## 无需 Visual Studio

工具链 + .NET Framework 的组合：
- ✅ 不需要 Visual Studio
- ✅ 不需要 VS Build Tools
- ✅ 仅需 .NET Framework 4.0+（Windows 自带或自动安装）
- ✅ 工具链已包含所有必需文件

## 最小化配置（如果需要）

仅保留核心文件（~5.4 MB）：
```bash
# 删除可选文件
rm packages/jade/toolchain/win32-x64-msvc/lib.exe
rm packages/jade/toolchain/win32-x64-msvc/mspdbsrv.exe
rm packages/jade/toolchain/win32-x64-msvc/mspdbst.dll
```

但**不推荐**，会影响调试和静态库功能。
