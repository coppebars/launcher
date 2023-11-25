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
	userSelect: 'none',

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
