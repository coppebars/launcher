import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const styled = style({
	[vars.darkSelector]: {
		color: vars.colors.dark[4],
	},
	[vars.lightSelector]: {
		color: vars.colors.dark[1],
	},
})
