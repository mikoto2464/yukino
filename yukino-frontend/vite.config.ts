import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";

// Vite 配置：开发环境通过 proxy 与 Rust 后端通信，生产打包由 Rust 后端托管
export default defineConfig({
  plugins: [
    vue({
      compilerOptions: {
        // 允许 <md-*> 自定义标签（@material/web 的 Web Components）
        isCustomElement: (tag) => tag.startsWith("md-"),
      },
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    port: 3000,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8088",
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
});
