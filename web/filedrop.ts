export const isYamlPath = (s: string | undefined | null) =>
	/\.ya?ml/.test(s || '')

const preventIfFiles = (e: DragEvent) => {
	const isFile = e.dataTransfer?.types.includes('Files') ?? false
	isFile && e.preventDefault()
	return isFile
}

const loadFile = (files: Iterable<File>) => {
	for (const file of files) {
		if (isYamlPath(file.name)) {
			return file.text()
		}
	}

	return Promise.reject()
}

export const installFileDrop = (
	el: HTMLElement,
	onLoad: (value: string) => void,
) => {
	el.addEventListener('dragover', e => {
		preventIfFiles(e)
	})

	el.addEventListener('drop', e => {
		if (!preventIfFiles(e)) return

		const files = e.dataTransfer?.files
		if (files == null) return

		loadFile(files).then(onLoad, () => {})
	})
}
