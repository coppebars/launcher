import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const label = style({
	fontWeight: 400,

	[vars.darkSelector]: {
		color: vars.colors.black,
	},
})
