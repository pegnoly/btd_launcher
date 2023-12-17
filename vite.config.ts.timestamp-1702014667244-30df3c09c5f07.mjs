// vite.config.ts
import { defineConfig } from "file:///D:/Homm5Dev/btd_launcher/node_modules/vite/dist/node/index.js";
import react from "file:///D:/Homm5Dev/btd_launcher/node_modules/@vitejs/plugin-react/dist/index.mjs";
import { resolve } from "path";
var __vite_injected_original_dirname = "D:\\Homm5Dev\\btd_launcher";
var vite_config_default = defineConfig(async () => ({
  plugins: [react()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      input: {
        main: resolve(__vite_injected_original_dirname, "index.html")
      }
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxIb21tNURldlxcXFxidGRfbGF1bmNoZXJcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkQ6XFxcXEhvbW01RGV2XFxcXGJ0ZF9sYXVuY2hlclxcXFx2aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRDovSG9tbTVEZXYvYnRkX2xhdW5jaGVyL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCByZWFjdCBmcm9tIFwiQHZpdGVqcy9wbHVnaW4tcmVhY3RcIjtcbmltcG9ydCB7IHJlc29sdmUgfSBmcm9tIFwicGF0aFwiO1xuXG4vLyBodHRwczovL3ZpdGVqcy5kZXYvY29uZmlnL1xuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XG4gIHBsdWdpbnM6IFtyZWFjdCgpXSxcblxuICAvLyBWaXRlIG9wdGlvbnMgdGFpbG9yZWQgZm9yIFRhdXJpIGRldmVsb3BtZW50IGFuZCBvbmx5IGFwcGxpZWQgaW4gYHRhdXJpIGRldmAgb3IgYHRhdXJpIGJ1aWxkYFxuICAvLyBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICAvLyB0YXVyaSBleHBlY3RzIGEgZml4ZWQgcG9ydCwgZmFpbCBpZiB0aGF0IHBvcnQgaXMgbm90IGF2YWlsYWJsZVxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiAxNDIwLFxuICAgIHN0cmljdFBvcnQ6IHRydWUsXG4gIH0sXG4gIC8vIHRvIG1ha2UgdXNlIG9mIGBUQVVSSV9ERUJVR2AgYW5kIG90aGVyIGVudiB2YXJpYWJsZXNcbiAgLy8gaHR0cHM6Ly90YXVyaS5zdHVkaW8vdjEvYXBpL2NvbmZpZyNidWlsZGNvbmZpZy5iZWZvcmVkZXZjb21tYW5kXG4gIGVudlByZWZpeDogW1wiVklURV9cIiwgXCJUQVVSSV9cIl0sXG4gIGJ1aWxkOiB7XG4gICAgLy8gVGF1cmkgc3VwcG9ydHMgZXMyMDIxXG4gICAgdGFyZ2V0OiBwcm9jZXNzLmVudi5UQVVSSV9QTEFURk9STSA9PSBcIndpbmRvd3NcIiA/IFwiY2hyb21lMTA1XCIgOiBcInNhZmFyaTEzXCIsXG4gICAgLy8gZG9uJ3QgbWluaWZ5IGZvciBkZWJ1ZyBidWlsZHNcbiAgICBtaW5pZnk6ICFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyA/IFwiZXNidWlsZFwiIDogZmFsc2UsXG4gICAgLy8gcHJvZHVjZSBzb3VyY2VtYXBzIGZvciBkZWJ1ZyBidWlsZHNcbiAgICBzb3VyY2VtYXA6ICEhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcsXG4gICAgcm9sbHVwT3B0aW9uczoge1xuICAgICAgaW5wdXQ6IHtcbiAgICAgICAgbWFpbjogcmVzb2x2ZShfX2Rpcm5hbWUsIFwiaW5kZXguaHRtbFwiKSxcbiAgICAgIH1cbiAgICB9XG4gIH0sXG59KSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQWdRLFNBQVMsb0JBQW9CO0FBQzdSLE9BQU8sV0FBVztBQUNsQixTQUFTLGVBQWU7QUFGeEIsSUFBTSxtQ0FBbUM7QUFLekMsSUFBTyxzQkFBUSxhQUFhLGFBQWE7QUFBQSxFQUN2QyxTQUFTLENBQUMsTUFBTSxDQUFDO0FBQUE7QUFBQTtBQUFBLEVBSWpCLGFBQWE7QUFBQTtBQUFBLEVBRWIsUUFBUTtBQUFBLElBQ04sTUFBTTtBQUFBLElBQ04sWUFBWTtBQUFBLEVBQ2Q7QUFBQTtBQUFBO0FBQUEsRUFHQSxXQUFXLENBQUMsU0FBUyxRQUFRO0FBQUEsRUFDN0IsT0FBTztBQUFBO0FBQUEsSUFFTCxRQUFRLFFBQVEsSUFBSSxrQkFBa0IsWUFBWSxjQUFjO0FBQUE7QUFBQSxJQUVoRSxRQUFRLENBQUMsUUFBUSxJQUFJLGNBQWMsWUFBWTtBQUFBO0FBQUEsSUFFL0MsV0FBVyxDQUFDLENBQUMsUUFBUSxJQUFJO0FBQUEsSUFDekIsZUFBZTtBQUFBLE1BQ2IsT0FBTztBQUFBLFFBQ0wsTUFBTSxRQUFRLGtDQUFXLFlBQVk7QUFBQSxNQUN2QztBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQ0YsRUFBRTsiLAogICJuYW1lcyI6IFtdCn0K
