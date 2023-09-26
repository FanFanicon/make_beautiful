import { defineConfig } from "umi";

export default defineConfig({
  routes: [
    { path: "/", component: "index" },
   
  ],
  npmClient: 'pnpm',
  proxy: {
    '/api': {
      'target': 'localhost://127.0.0.1:8080',
      'changeOrigin': true,
      'pathRewrite': { '^/api': '' },
    },
  },
});
