import { globalStyle } from '@vanilla-extract/css'

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
	height: '100vh',
	position: 'relative',
	fontSize: '16px',
	zIndex: 1,
	backgroundColor: 'transparent',
	opacity: 0.95,
})
