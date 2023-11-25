import { rem }   from '@mantine/core'
import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const listItem = style({
	height: rem(48),

	padding: rem(8),

	borderRadius: vars.radius.md,

	':hover': {
		[vars.darkSelector]: {
			backgroundColor: vars.colors.darkShade[0],
		},

		[vars.lightSelector]: {
			backgroundColor: vars.colors.lightShade[0],
		},
	},
})

export const itemImage = style({
	height: '80%',
	aspectRatio: '1 / 1',

	[vars.darkSelector]: {
		backgroundColor: vars.colors.darkShade[1],
	},

	[vars.lightSelector]: {
		backgroundColor: vars.colors.lightShade[1],
	},

	borderRadius: vars.radius.md,
	border: 'none',
	outline: 'none',
})
