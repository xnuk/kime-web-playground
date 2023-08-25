import { install, initialized } from 'kime-web'

const configInput = document.getElementById('config') as HTMLTextAreaElement
const layoutInput = document.getElementById('scratchpad') as HTMLTextAreaElement
const errorSlot = document.getElementById('error') as HTMLDivElement

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
		const ret = install(config, layoutInput)
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

initialized.then(() => {
	let instance = instantiate(configInput.value)

	configInput.addEventListener('input', () => {
		if (instance) {
			instance.free()
		}
		instance = instantiate(configInput.value)
	})
})
