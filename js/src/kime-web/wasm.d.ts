declare module '*.wasm' {
	const initialized: Promise<void>
	export { initialized }
}
