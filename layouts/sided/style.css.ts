import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const container = style({
	height: '100%',

	selectors: {
		[vars.darkSelector]: {
			backgroundColor: vars.colors.darkTint[1],
		},

		[vars.lightSelector]: {
			backgroundColor: vars.colors.lightTint[2],
		},
	},
})

export const sidebar = style({
	position: 'relative',
	flexShrink: 0,
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
