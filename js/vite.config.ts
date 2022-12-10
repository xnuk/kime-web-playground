import type { Plugin } from 'vite'
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'

const trimHtml = (): Plugin => ({
	name: 'trim-html',
	transformIndexHtml: html => html.trim().replace(/\r?\n\s*/g, ''),
})

export default defineConfig({
	build: {
		target: 'esnext',
	},
	plugins: [wasm(), trimHtml()],
})
