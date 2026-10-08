import { defineConfig } from 'vite';

export default defineConfig({
  base: process.env.GITHUB_PAGES === 'true' ? '/AI-Game-Maker/' : '/',
  server: { host: '0.0.0.0', port: 1420, strictPort: true },
  build: { target: 'es2022', sourcemap: true },
});
