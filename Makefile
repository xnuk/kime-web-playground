all: js/dist/index.html

.PHONY: clean
clean:
	rm -rf rust/target/ rust/pkg/ js/node_modules/ js/dist/

rust/pkg/kime_web_bg.wasm:
	cd rust && wasm-pack build --target bundler

js/node_modules:
	cd js && pnpm install

js/dist/index.html: rust/pkg/kime_web_bg.wasm js/node_modules
	cd js && pnpm build
