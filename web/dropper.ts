const preventIfFiles = (e: DragEvent) => {
	const isFile = e.dataTransfer?.types.includes('Files') ?? false
	isFile && e.preventDefault()
	return isFile
}

export const installDrop = (
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

		for (const file of files) {
			if (file.name.endsWith('.yaml') || file.name.endsWith('.yml')) {
				file.text().then(onLoad, () => {})
				break
			}
		}
	})
}
