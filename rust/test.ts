const main = async () => {
	const path = Deno.args[0]
	if (path == null) {
		throw 'Usage: thing /path/to.wasm'
	}

	const binary = await Deno.readFile(path)
	const compiled = await WebAssembly.compile(binary)

	console.log('imports', WebAssembly.Module.imports(compiled))
	console.log('exports', WebAssembly.Module.exports(compiled))

	const instance = await WebAssembly.instantiate(compiled, {})

	const { Maemmae, memory, MM_RESULT, malloc, free } = instance.exports
	const mm = new Uint32Array(memory.buffer, MM_RESULT.value, 1)[0]

	const dick = new TextEncoder().encode('dick')
	const thick = malloc(dick.byteLength)
	new Uint8Array(memory.buffer, thick, dick.byteLength).set(dick)
	const foo = malloc(mm)
	console.log(new Uint8Array(memory.buffer, thick, dick.byteLength))
	console.log(Maemmae(thick, dick.byteLength, thick, dick.byteLength, foo))
}

if (import.meta.main) main()
