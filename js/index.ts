import { Maemmae } from '../rust/pkg'

// 니미 진짜
const enum Modifier {
	SHIFT = 1,
	CONTROL = 2,
	SUPER = 4,
	ALT = 8,
}

const mam = new Maemmae(
	JSON.stringify({
		layout: 'xnuk',
		word_commit: false,
		addons: {
			xnuk: [
				'ComposeChoseongSsang',
				'ComposeJongseongSsang',
				'ComposeJungseongSsang',
			],
		},
	}),

	JSON.stringify({
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
	}),
)

console.log(mam)

const input = document.createElement('input')
input.style.width = '90%'
const preedit = input.cloneNode() as typeof input
const commit = input.cloneNode() as typeof input
const br = document.createElement('br')

input.placeholder = '입력...'
preedit.placeholder = 'preedit'
commit.placeholder = 'commit'

document.body.append(input, br, preedit, br.cloneNode(), commit)

const getModifier = (ev: KeyboardEvent): number => {
	let bits = 0
	if (ev.shiftKey) bits |= Modifier.SHIFT
	if (ev.ctrlKey) bits |= Modifier.CONTROL
	if (ev.altKey) bits |= Modifier.ALT
	if (ev.metaKey) bits |= Modifier.SUPER
	return bits
}

input.addEventListener('keydown', ev => {
	if (mam.press_key(ev.code, getModifier(ev))) {
		ev.preventDefault()
		ev.stopPropagation()
	} else {
		mam.clear_preedit()
	}
	preedit.value = mam.preedit_str()
	commit.value = mam.get_buf()
})
