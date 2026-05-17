# Vue 预编译方案

**更新日期**: 2026-05-17

## 概述

Vue 3 使用 `@vitejs/plugin-vue` 插件在构建时预编译模板，生成纯 JavaScript render 函数，实现：
- ✅ 无运行时模板编译
- ✅ 无 `eval()` 或 `new Function()`
- ✅ 可在 QuickJS/轻量引擎运行
- ✅ 支持浏览器扩展 CSP

## 验证结果 (2026-05-17)

**Vue 预编译成功验证**：
- 编译产物：`src/vue-compile-demo/dist/assets/index-B8iAt2nN.js`
- ✅ 无 `eval()` 出现
- ✅ 无 `new Function()` 出现
- ✅ 模板完全预编译为 render 函数

**编译后的关键代码**：
```javascript
// 预编译后的 render 函数示例
return (_ctx, _cache) => {
  return openBlock(), createElementBlock("div", {
    id: "app",
    class: normalizeClass({ active: isActive.value })
  }, [...], 2);
};
```

## 集成方式

### 1. Vite 配置 (`vite.config.ts`)

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          whitespace: 'condense'
        }
      }
    })
  ],
  build: {
    target: 'es2020',
    minify: 'esbuild'
  }
})
```

### 2. Vue 组件 (`App.vue`)

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'

const title = ref('Vue Compile Demo')
const count = ref(0)
const isActive = ref(true)

const doubledCount = computed(() => count.value * 2)

function increment() {
  count.value++
}
</script>

<template>
  <div id="app" :class="{ active: isActive }">
    <h1>{{ title }}</h1>
    <button @click="increment">Click me ({{ count }})</button>
    <div class="card" v-if="count > 0">
      <h2>Card #{{ count }}</h2>
    </div>
  </div>
</template>
```

## 预编译结果

### 输入：Vue SFC (Single File Component)
```vue
<template>
  <div id="app">
    <h1>{{ title }}</h1>
    <button @click="increment">Click me</button>
  </div>
</template>
```

### 输出：预编译的 Render 函数
```javascript
// 预编译后的 render 函数
return (_ctx, _cache) => {
  return openBlock(), createElementBlock("div", {
    id: "app"
  }, [
    createBaseVNode("h1", null, toDisplayString(title.value), 1),
    createBaseVNode("button", {
      onClick: increment
    }, " Click me (" + toDisplayString(count.value) + ") ", 3)
  ], 2);
};
```

## 关键编译产物

### 1. createElementBlock / createBaseVNode
Vue 3 的虚拟 DOM 创建函数：
- `createElementBlock("div", props, children, patchFlag)` - 创建块级元素
- `createBaseVNode("h1", props, text, patchFlag)` - 创建基本元素

### 2. PatchFlag (性能优化)
数字标识符，标记动态内容：
- `1` - TEXT (文本内容)
- `2` - CLASS (class 绑定)
- `3` - STYLE (style 绑定)
- `40` - MODEL (v-model)

### 3. 静态提升 (Static Hoisting)
```javascript
// 静态内容被提升到 render 函数外部
const _hoisted_1 = { class: "btn btn-primary" }
const _hoisted_2 = ["onClick", "onInput"]
```

## jrust-runtime 集成路径

### 步骤 1：Vite 构建
```
App.vue → @vitejs/plugin-vue → 预编译 JS (render函数)
```

### 步骤 2：jrust-translator 转译
```
预编译 JS → AST → Rust 代码 → jrust1
```

### 步骤 3：jrust-runtime 运行
```
jrust1 → DOM 序列化 → 事件绑定 → 运行
```

## 方案优势

| 特性 | 运行时编译 | 预编译方案 |
|------|-----------|-----------|
| 模板编译 | 需要运行时 | ✅ 不需要 |
| eval() | 可能需要 | ✅ 不需要 |
| new Function | 可能需要 | ✅ 不需要 |
| CSP 兼容 | ❌ 不兼容 | ✅ 兼容 |
| 启动性能 | 较慢 | ✅ 更快 |
| 包体积 | 包含编译器 | ✅ 更小 |

## 文件结构

```
vue-compile-demo/
├── package.json
├── vite.config.ts      # Vite 配置
├── tsconfig.json
├── index.html
└── src/
    ├── main.ts         # Vue 入口
    ├── App.vue         # Vue 组件
    └── env.d.ts
```

## 下一步

1. **解析预编译 JS**：扩展 jrust-translator 支持解析 Vue render 函数
2. **映射到 Rust DOM**：将 `createElementBlock` 映射到 `jrust-runtime` 的 `Element` API
3. **事件绑定**：实现 `@click` 等事件到 Rust 闭包的转换
