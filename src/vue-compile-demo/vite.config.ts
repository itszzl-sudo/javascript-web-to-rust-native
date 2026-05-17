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
    minify: false,
    rollupOptions: {
      output: {
        format: 'es'
      }
    }
  },
  define: {
    'process.env.NODE_ENV': JSON.stringify('production')
  }
})