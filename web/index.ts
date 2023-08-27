import { install, initialized, type MountedIME } from 'kime-web'
import { importUrl, parseUrl } from './layout-from-url.ts'

const configInput = document.getElementById('config') as HTMLTextAreaElement
const textInput = document.getElementById('scratchpad') as HTMLTextAreaElement
const errorSlot = document.getElementById('error') as HTMLDivElement
const status = document.getElementById('status') as HTMLDivElement

const config = `
engine:
  default_category: Hangul

  global_hotkeys:
    Hangul: &toggle
      behavior: !Toggle [Hangul, Latin]
      result: Consume
    AltR: *toggle
    C-Space: *toggle
    S-Space: *toggle
    Esc:
      behavior: !Switch Latin
      result: Bypass

  latin:
    preferred_direct: true

  hangul:
    layout: xnuk
    word_commit: false
    addons:
      xnuk:
      - ComposeChoseongSsang
      - ComposeJongseongSsang
      - ComposeJungseongSsang

layouts:
  xnuk:
    Q: '$ㅅ$ㅒ'
    W: '$ㄹ$ㅑ'
    F: '$ㅂ$ㅐ'
    P: '$ㅌ$ㅓ'
    B: '$ㅋ$ㅕ'
    J:  'ㄹ'
    L:  'ㄷ'
    U:  'ㅁ$ㅢ'
    Y:  'ㅊㅜ'
    SemiColon:  'ㅍㅗ'

    A: '$ㅇ$ㅠ'
    R: '$ㄴ$ㅖ'
    S: '$ㅎ$ㅣ'
    T: '$ㅍ$ㅏ'
    G: '$ㄷ$ㅡ'
    M:  'ㄴ'
    N:  'ㅇ'
    E:  'ㄱ'
    I:  'ㅈ'
    O:  'ㅂ'
    Minus:  'ㅌ'

    Z: '$ㅁ$ㅢ'
    X: '$ㅆ$ㅛ'
    C: '$ㄱ$ㅔ'
    D: '$ㅈ$ㅗ'
    V: '$ㅊ$ㅜ'
    K:  'ㅅ'
    H:  'ㅎ'
    Slash:  'ㅋㅗ'
`.trim()

configInput.defaultValue = config

const instantiate = (config: string) => {
	try {
		const ret = install(config, textInput)
		errorSlot.textContent = ''
		return ret
	} catch (err: unknown) {
		if (typeof err !== 'object' || err == null) {
			return null
		}

		if ('message' in err && typeof err.message === 'string') {
			errorSlot.textContent = err.message
		}

		return null
	}
}

const reload = (() => {
	let instance = null as null | MountedIME
	let ready = false

	const reloader = () => {
		if (!ready) return
		if (instance != null) {
			instance.free()
		}
		instance = instantiate(configInput.value)
	}

	initialized.then(() => {
		ready = true
		reloader()
	})

	return reloader
})()

const debounced = <F extends (...args: any[]) => any>(
	func: F,
	delay: number = 50,
) => {
	let timeout = setTimeout(() => {})

	return (...args: Parameters<F>) => {
		clearTimeout(timeout)
		timeout = setTimeout(() => func(...args), delay)
	}
}

const fetcher = debounced((url: URL, value: string) => {
	importUrl(url)
		.then(config => {
			if (configInput.value != value) return
			configInput.value = config
			errorSlot.textContent = ''
		})
		.catch(() => {
			errorSlot.textContent = `Failed to fetch ${url}.`
		})
}, 500)

configInput.addEventListener('input', () => {
	const url = parseUrl(configInput.value)
	if (url == null) return reload()

	errorSlot.textContent = `Fetching ${url}...`
	fetcher(url, configInput.value)
})

textInput.addEventListener('kimeinputcategorychange', e => {
	const category = (e as any).detail as 'latin' | 'hangul'
	if (status.textContent !== category) {
		status.textContent = category
		status.animate([{ opacity: 0.75, display: 'block' }, { opacity: 0 }], {
			duration: 2000,
			iterations: 1,
			easing: 'ease-in',
		})
	}
})

declare global {
	const SERVE: boolean
}

if (SERVE) {
	new EventSource('/esbuild').addEventListener('change', () =>
		location.reload(),
	)
}
