import { globalStyle } from '@vanilla-extract/css'

import { vars }        from '@theme/vars'

globalStyle('img', {
	maxInlineSize: '100%',
	maxBlockSize: '100%',
	maxWidth: '100%',
	verticalAlign: 'middle',
	fontStyle: 'italic',
	backgroundRepeat: 'no-repeat',
	backgroundSize: 'cover',
	shapeMargin: '1rem',
})

globalStyle('*::-webkit-scrollbar', {
	display: 'none',
})

globalStyle('html, body, #root', {
	width: '100%',
	minHeight: '100vh',
	position: 'relative',
	fontSize: '16px',
	zIndex: 1,
	backgroundColor: 'transparent',
})

globalStyle('#root', {
	[vars.darkSelector]: {
		backgroundColor: `color-mix(in srgb, ${vars.colors.black}, transparent 10%)`,
	},
	[vars.lightSelector]: {
		backgroundColor: `color-mix(in srgb, ${vars.colors.white}, transparent 10%)`,
	},
})
