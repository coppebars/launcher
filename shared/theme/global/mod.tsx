import type { Theme }    from '@emotion/react'
import      { Global }   from '@emotion/react'
import      { useTheme } from '@emotion/react'

import      { opacity }  from 'styled-std'

const global = (theme: Theme) => () => ({
	'*:where(:not(html, iframe, canvas, img, svg, video, audio):not(svg *, symbol *))': {
		all: 'unset',
		display: 'revert',
	},

	'*,*::before,*::after': {
		boxSizing: 'border-box',
	},

	'a,button': {
		cursor: 'revert',
	},

	'ol,ul,menu': {
		listStyle: 'none',
	},

	img: {
		maxInlineSize: '100%',
		maxBlockSize: '100%',
		maxWidth: '100%',
		verticalAlign: 'middle',
		fontStyle: 'italic',
		backgroundRepeat: 'no-repeat',
		backgroundSize: 'cover',
		shapeMargin: '1rem',
	},

	table: {
		borderCollapse: 'collapse',
	},

	'input,textarea': {
		WebkitUserSelect: 'auto',
	},

	textarea: {
		whiteSpace: 'revert',
	},

	':where(pre)': {
		all: 'revert',
	},

	'::placeholder': {
		color: 'unset',
	},

	'::marker': {
		content: 'initial',
	},

	":where([contenteditable]:not([contenteditable='false']))": {
		MozUserModify: 'read-write',
		WebkitUserModify: 'read-write',
		overflowWrap: 'break-word',
		WebkitUserSelect: 'auto',
	},

	'html,body,#root': {
		width: '100%',
		minHeight: '100vh',
		position: 'relative',
		fontSize: '16px',
		zIndex: 1,
		fontFamily: theme.font.family.default,
		color: theme.palette.foreground,
	},

	'#root': {
		backgroundColor: opacity(theme.palette.background, 0.22),
	},

	'img,button': {
		userSelect: 'none',
	},

	'*::-webkit-scrollbar': {
		display: 'none',
	},
})

export function GlobalStyles() {
	const theme = useTheme()

	return <Global styles={global(theme) as never} />
}
