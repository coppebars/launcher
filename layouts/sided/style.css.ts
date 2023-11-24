import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const container = style({
	height: '100%',

	selectors: {
		[vars.darkSelector]: {
			backgroundColor: vars.colors.dark[7],
		},

		[vars.lightSelector]: {
			backgroundColor: vars.colors.light[8],
		},
	},
})

export const sidebar = style({
	position: 'relative',
})

export const view = style({
	position: 'relative',
	flexGrow: 1,

	borderTopLeftRadius: vars.radius.md,
	borderBottomLeftRadius: vars.radius.md,

	[vars.darkSelector]: {
		backgroundColor: vars.colors.black,
	},

	[vars.lightSelector]: {
		backgroundColor: vars.colors.white,
	},
})
