import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [vue()],
	server: {
		proxy: {
			"/files": "http://127.0.0.1:8080",
			"/config.json": "http://127.0.0.1:8080"
		}
	}
})
