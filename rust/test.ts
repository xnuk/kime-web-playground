const binary = await Deno.readFile('./pkg/kime_web_bg.wasm')
WebAssembly.compile(binary).then(b => WebAssembly.instantiate(b, {}))
