import { globalStyle } from '@vanilla-extract/css'
import { style }       from '@vanilla-extract/css'

import { vars }        from '@theme/vars'

export const icon = style({})

globalStyle(`${icon} > path`, {
	fill: vars.colors.black,
})
