/// <reference no-default-lib="true" />
/// <reference lib="deno.ns" />
/// <reference lib="dom" />

import './live-reload.ts'

import { maemmae_new, initialized, Maemmae } from './kime-web/mod.ts'

const init: Promise<void> = initialized

// There is no sane way
const enum Modifier {
	SHIFT = 1,
	CONTROL = 2,
	SUPER = 4,
	ALT = 8,
}

const defaultConfig = JSON.stringify(
	{
		layout: 'xnuk',
		word_commit: false,
		addons: {
			xnuk: [
				'ComposeChoseongSsang',
				'ComposeJongseongSsang',
				'ComposeJungseongSsang',
			],
		},
	},
	null,
	2,
)

const defaultLayout = JSON.stringify(
	{
		Q: '$ㅅ$ㅒ',
		W: '$ㄹ$ㅑ',
		F: '$ㅂ$ㅐ',
		P: '$ㅌ$ㅓ',
		B: '$ㅋ$ㅕ',
		J: 'ㄹ',
		L: 'ㄷ',
		U: 'ㅁ$ㅢ',
		Y: 'ㅊㅜ',
		SemiColon: 'ㅍㅗ',

		A: '$ㅇ$ㅠ',
		R: '$ㄴ$ㅖ',
		S: '$ㅎ$ㅣ',
		T: '$ㅍ$ㅏ',
		G: '$ㄷ$ㅡ',
		M: 'ㄴ',
		N: 'ㅇ',
		E: 'ㄱ',
		I: 'ㅈ',
		O: 'ㅂ',
		Minus: 'ㅌ',

		Z: '$ㅁ$ㅢ',
		X: '$ㅆ$ㅛ',
		C: '$ㄱ$ㅔ',
		D: '$ㅈ$ㅗ',
		V: '$ㅊ$ㅜ',
		K: 'ㅅ',
		H: 'ㅎ',
		Slash: 'ㅋㅗ',
	},
	null,
	2,
)

const mam = init.then((): Maemmae | null => {
	const result = maemmae_new(defaultConfig, defaultLayout)
	const error = result.get_error()
	if (error != null) {
		console.error(error)
	}

	const mam = result.take_data()
	return mam || null
})

const input = document.getElementById('text') as HTMLInputElement
const preedit = document.getElementById('preedit') as HTMLInputElement
const commit = document.getElementById('commit') as HTMLInputElement
const configInput = document.getElementById('config') as HTMLTextAreaElement
const layoutInput = document.getElementById('layout') as HTMLTextAreaElement
const errorDisplay = document.getElementById('error') as HTMLDivElement

configInput.defaultValue = defaultConfig
layoutInput.defaultValue = defaultLayout

const new_mam = () => {
	const result = maemmae_new(configInput.value, layoutInput.value)
	const error = result.get_error()
	const hasError = error != null
	errorDisplay.textContent = error || ''
	configInput.setAttribute('aria-invalid', hasError + '')
	layoutInput.setAttribute('aria-invalid', hasError + '')

	const mam = result.take_data()
	return mam || null
}

// const executeAndSelf = <T extends unknown>(f: () => T) => (f(), f)

// ;(() => {
// 	const styleInput = document.getElementById('style') as HTMLTextAreaElement
// 	const styleInline = document.getElementById(
// 		'inline-style',
// 	) as HTMLStyleElement
// 	styleInput.addEventListener(
// 		'input',
// 		executeAndSelf(() => {
// 			const { value } = styleInput
// 			styleInline.textContent = value
// 			styleInput.rows = Math.max(value.split('\n').length, 4)
// 		}),
// 	)
// })()

const getModifier = (ev: KeyboardEvent): number => {
	let bits = 0
	if (ev.shiftKey) bits |= Modifier.SHIFT
	if (ev.ctrlKey) bits |= Modifier.CONTROL
	if (ev.altKey) bits |= Modifier.ALT
	if (ev.metaKey) bits |= Modifier.SUPER
	return bits
}

init.then(() => {
	let mam = new_mam()

	const refresh = () => {
		mam?.free()
		mam = new_mam()
	}

	configInput.addEventListener('input', refresh)
	layoutInput.addEventListener('input', refresh)

	input.addEventListener('keydown', ev => {
		if (mam == null) return

		if (mam.press_key(ev.code, getModifier(ev))) {
			ev.preventDefault()
			ev.stopPropagation()
		} else {
			mam.clear_preedit()
		}
		preedit.value = mam.preedit_str()
		commit.value = mam.get_buf()
	})
})
