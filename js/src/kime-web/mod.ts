// @deno-types="./wasm-pkg/kime-web.d.ts"
export * from './wasm-pkg/kime-web_bg.js'

// @deno-types="./wasm.d.ts"
export { initialized } from './wasm-pkg/kime-web_bg.wasm'
