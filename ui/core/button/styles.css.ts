import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const label = style({
	[vars.darkSelector]: {
		color: vars.colors.black,
	},
})
