import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const shaded = style({
	[vars.darkSelector]: {
		backgroundColor: vars.colors.dark[4],
	},
	[vars.lightSelector]: {
		backgroundColor: vars.colors.dark[1],
	},
})
