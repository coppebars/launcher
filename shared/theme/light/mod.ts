import { Schema }          from '@theme/common'
import { theme as common } from '@theme/common'

export const theme = {
	...common,
	palette: {
		...common.palette,
		background: common.palette.foreground,
		foreground: common.palette.background,
		primary: '#8fa560',
	},
} satisfies Schema
