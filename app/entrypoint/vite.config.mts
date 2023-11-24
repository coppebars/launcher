import { defineConfig }         from 'vite'

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'
import react                    from '@vitejs/plugin-react'

export default defineConfig(async () => ({
	plugins: [react(), vanillaExtractPlugin()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
	},
}))
