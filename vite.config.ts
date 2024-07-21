import path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    open: 'index.html',
  },
  root: 'src/panel',
  publicDir: 'assets',
  build: {
    outDir: '../../web',
  },
  resolve: {
    alias: {
      "/src/panel": path.resolve(__dirname, "./src/panel"),
    },
  },
});
