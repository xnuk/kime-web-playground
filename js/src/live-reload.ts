declare const SERVE: boolean

if (SERVE) {
	new EventSource('/esbuild').addEventListener('change', () =>
		window.location.reload(),
	)
}
