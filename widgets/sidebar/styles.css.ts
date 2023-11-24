import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const item = style({
	position: 'relative',

	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',

	width: '100%',
	aspectRatio: '1 / 1',

	borderRadius: vars.radius.md,

	transition: 'backgroundColor, .2s',
	overflow: 'hidden',

	zIndex: 1,

	':hover': {
		[vars.darkSelector]: {
			backgroundColor: vars.colors.darkShade[0],
		},
	},

	selectors: {
		'&[data-active], &[data-active]:hover': {
			backgroundColor: 'inherit',

			[vars.darkSelector]: {
				color: vars.colors.primary[4],
				backgroundColor: vars.colors.primary[1],
			},

			[vars.lightSelector]: {
				color: vars.colors.primary[4],
				backgroundColor: vars.colors.primary[7],
			},
		},
	},
})

export const glow = style({
	position: 'absolute',

	top: '50%',
	left: '50%',
	translate: '-50% -50%',

	borderRadius: '1000px',

	transition: 'opacity, .2s',

	zIndex: -1,

	[vars.darkSelector]: {
		width: '2rem',
		height: '2rem',
		filter: 'blur(15px)',
		backgroundColor: vars.colors.primary[4],
	},

	[vars.lightSelector]: {
		width: '3rem',
		height: '3rem',
		filter: 'blur(25px)',
		backgroundColor: vars.colors.primary[4],
	},

	selectors: {
		'&:not([data-active])': {
			opacity: 0,
		},
	},
})
