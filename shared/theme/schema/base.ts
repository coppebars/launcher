import      { createTheme }          from '@mantine/core'
import      { MantineThemeOverride } from '@mantine/core'
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
			`color-mix(in srgb, ${primary} 10%, ${black})`,
			`color-mix(in srgb, ${primary} 90%, ${black})`,
			primary,
			primary,
			`color-mix(in srgb, ${primary} 90%, ${white})`,
			`color-mix(in srgb, ${primary} 10%, ${white})`,
			undefined!,
			undefined!,
		],
	},
	primaryColor: 'primary',
	primaryShade: 4,
} satisfies PartialDeep<MantineThemeOverride>)
