/*@__PURE__*/
/*@__INLINE__*/
const words = (s: string) => s.split(' ')

const builtins = words(
	'dubeolsik sebeolsik-3-90 sebeolsik-3-91 sebeolsik-3sin-1995 sebeolsik-3sin-p2',
)

const fixCommonMistakes = (url: URL) => {
	// github
	if (url.host === 'github.com' && url.pathname.includes('/blob/')) {
		url.host = 'raw.githubusercontent.com'

		// use Array.prototype.toSpliced after few months
		const components = url.pathname.split('/')
		components.splice(3, 1)
		url.pathname = components.join('/')
	}
}

export const parseUrl = (href: string): URL | null => {
	const s = href.trim()
	if (!/^https?:\/\/[^\s]+$/.test(s)) return null
	try {
		return new URL(s)
	} catch {
		return null
	}
}

export const importUrl = async (url: URL) => {
	fixCommonMistakes(url)

	let config = await fetch(url).then(v => v.text())
	const layoutName = config.match(/hangul:.*?layout:\s*([^\s]+)\s*/s)?.[1]
	const layoutPresented = /^layouts:\s*\{?$/m.test(config)

	if (
		layoutName != null &&
		!builtins.includes(layoutName) &&
		!layoutPresented
	) {
		try {
			const layoutConfig = await fetch(
				new URL(`./layouts/${layoutName}.yaml`, url),
			).then(v => (v.status < 400 ? v.text() : Promise.reject()))

			const indented = layoutConfig
				.trim()
				.split('\n')
				.map(line => '    ' + line)
				.join('\n')
			config += `\nlayouts:\n  ${layoutName}:\n` + indented
		} catch {}
	}

	return config
}
