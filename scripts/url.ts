import { pathToFileURL } from 'node:url'

export const directoryUrl = (path: string | URL, append?: string): URL => {
	const url = typeof path === 'string' ? pathToFileURL(path) : path
	if (!url.pathname.endsWith('/')) url.pathname += '/'
	if (append != null) return directoryUrl(new URL(append, url))
	return url
}

export { fileURLToPath as urlToPath, pathToFileURL as fileUrl } from 'node:url'
