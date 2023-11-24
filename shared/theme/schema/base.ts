import      { createTheme }          from '@mantine/core'
import      { MantineThemeOverride } from '@mantine/core'
import      { mix }                  from '@theme/css'
import type { PartialDeep }          from 'type-fest'

import      { mabry }                from '@theme/fonts'

const white = '#ebebeb'
const black = '#131313'
const primary = '#E87A52'

export const base = createTheme({
	fontFamily: mabry,
	radius: {
		xs: '2px',
		sm: '4px',
		md: '6px',
		lg: '8px',
		xl: '12px',
	},
	defaultRadius: 'md',
	white: '#ebebeb',
	black: '#131313',
	colors: {
		primary: [
			undefined!,
			undefined!,
			mix(black, primary, 0.1),
			mix(black, primary, 0.9),
			primary,
			primary,
			mix(white, primary, 0.9),
			mix(white, primary, 0.1),
			undefined!,
			undefined!,
		],
		dark: [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.7, -0.1, -0.2, -0.3].map((it) => {
			return it >= 0 ? mix(white, black, it) : mix('black', black, it)
		}) as never,
	},
	primaryColor: 'primary',
	primaryShade: 4,
} satisfies PartialDeep<MantineThemeOverride>)
