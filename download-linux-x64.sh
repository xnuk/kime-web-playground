#!/bin/sh
# shellcheck shell=dash enable=all

set -euf

if [ -z "${CACHE_DIR:-}" ]; then
	echo 'environment CACHE_DIR must be set' >&2
	exit 1
fi

mkdir -p "${CACHE_DIR}" "${CACHE_DIR}/bin-cache" "${CACHE_DIR}/bin"

check_256_hash() {
	hash="$1"
	path="$2"
	sha256sum "${path}" | grep "${hash}" >/dev/null
}

curl_download() {
	hash="$1"
	url="$2"
	path="${CACHE_DIR}/bin-cache/${hash}-$(basename "${url}")"

	if [ ! -f "${path}" ] || \
		not check_256_hash "${hash}" "${path}" 2>/dev/null
	then
		curl --location --get --output "${path}" "${url}"
	fi

	check_256_hash "${hash}" "${path}"
	printf '%s' "${path}"
}

get_eget() {
	hash="$1"
	url="$2"
	file="$3"
	path=$(curl_download "${hash}" "${url}")

	tar -xf "${path}" "${file}" -O > "${CACHE_DIR}/bin/eget"
	chmod +x "${CACHE_DIR}/bin/eget"
}

eget() {
	hash="$1"
	url="$2"
	file="$3"
	path=$(curl_download "${hash}" "${url}")
	"${CACHE_DIR}/bin/eget" --to="${CACHE_DIR}/bin/" \
		--verify-sha256="${hash}" --file="${file}" "${path}"
}

get_eget 75ee2428bc0a202131e20e592158f4f76a159a2712157bd02673013be294e28b \
	https://github.com/zyedidia/eget/releases/download/v1.3.1/eget-1.3.1-linux_amd64.tar.gz \
	eget-1.3.1-linux_amd64/eget

eget 7f158bbfcd502e84da01a668e2410a538e82eeb1902f8929e318e33b7220a52e \
	https://github.com/denoland/deno/releases/download/v1.29.4/deno-x86_64-unknown-linux-gnu.zip \
	deno

eget 2f340545b302c706bf22d2ee2382ae53a73f32bd62cb3435fb9404303c4dc940 \
	https://github.com/WebAssembly/binaryen/releases/download/version_111/binaryen-version_111-x86_64-linux.tar.gz \
	wasm-opt

eget a9e819b9f4e856933a6132c007fd794e04f8fd790035fb474b2b7ee6a4bbbdb1 \
	https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.83/wasm-bindgen-0.2.83-x86_64-unknown-linux-musl.tar.gz \
	wasm-bindgen

eget 3c7aa42ce455e819475d16f34352f3008271a43aa80adad308df4e8f29413648 \
	https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.17.3.tgz \
	esbuild
