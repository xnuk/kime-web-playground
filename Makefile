all: bin wasm-optimize js

CACHE_DIR ?= $(shell realpath .)/cache
export CACHE_DIR
cache_dir ::= $(shell realpath --relative-to=. $(CACHE_DIR))

.PHONY: clean bin rust wasm-optimize js
clean:
	rm -rf rust/target/ rust/pkg/ js/node_modules/ js/dist/

bin:
	./download-linux-x64.sh

CARGO_BUILD ::= cargo build
CARGO_BUILD += --manifest-path ./rust/Cargo.toml
CARGO_BUILD += --target wasm32-unknown-unknown
CARGO_BUILD += --release
CARGO_BUILD += --target-dir $(cache_dir)/rust

WASM_BINDGEN ::=
# WASM_BINDGEN += WASM_BINDGEN_ANYREF=1
WASM_BINDGEN += $(cache_dir)/bin/wasm-bindgen
WASM_BINDGEN += --target bundler
WASM_BINDGEN += --out-dir $(cache_dir)/wasm-pkg
WASM_BINDGEN += --out-name kime-web
WASM_BINDGEN += --typescript
WASM_BINDGEN += --remove-producers-section
WASM_BINDGEN += --omit-default-module-path
# WASM_BINDGEN += --reference-types
WASM_BINDGEN += $(cache_dir)/rust/wasm32-unknown-unknown/release/kime_web.wasm

WASM_OPT ::= $(CACHE_DIR)/bin/wasm-opt
# WASM_OPT += --enable-reference-types
WASM_OPT += -O

DENO ::= DENO_DIR=$(CACHE_DIR)/deno
DENO += DENO_NO_PROMPT=1
DENO += DENO_NO_UPDATE_CHECK=1
DENO += ESBUILD_BINARY_PATH=$(CACHE_DIR)/bin/esbuild
DENO += $(cache_dir)/bin/deno
DENO += run
DENO += --config=./deno.json
DENO += --lock=./js/deno.lock
DENO += --allow-read=.
DENO += --allow-write=$(cache_dir)
DENO += --allow-env=ESBUILD_BINARY_PATH
DENO += --allow-run=$(cache_dir)/bin/esbuild
DENO += ./js/build.ts
DENO += --outdir=$(cache_dir)/js

ifdef minify
	DENO += --minify=$(minify)
endif

ifdef port
	DENO += --port=$(port)
endif

js: rust
	ln -sfn "$(CACHE_DIR)/wasm-pkg" ./js/src/kime-web/wasm-pkg
	$(DENO)

rust:
	$(CARGO_BUILD)
	$(WASM_BINDGEN)

wasm-optimize: bin rust
	for p in "$(cache_dir)/wasm-pkg"/*.wasm; do \
		$(WASM_OPT) "$$p" -o temp.wasm && mv -f temp.wasm "$$p" \
	; done
