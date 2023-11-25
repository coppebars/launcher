import      { createTheme }          from '@mantine/core'
import      { MantineThemeOverride } from '@mantine/core'
import type { PartialDeep }          from 'type-fest'

import      { mix }                  from '@theme/css'
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
			mix(black, primary, 0.05),
			mix(black, primary, 0.1),
			mix(black, primary, 0.4),
			mix(black, primary, 0.8),
			primary,
			mix(white, primary, 0.5),
			mix(white, primary, 0.4),
			mix(white, primary, 0.2),
			mix(white, primary, 0.1),
			undefined!,
		],
		darkShade: [
			mix(black, white, 0.05),
			mix(black, white, 0.1),
			mix(black, white, 0.2),
			mix(black, white, 0.3),
			mix(black, white, 0.4),
			mix(black, white, 0.5),
			mix(black, white, 0.6),
			mix(black, white, 0.7),
			mix(black, white, 0.8),
			mix(black, white, 0.9),
		],
		lightShade: [
			mix(white, black, 0.05),
			mix(white, black, 0.1),
			mix(white, black, 0.2),
			mix(white, black, 0.3),
			mix(white, black, 0.4),
			mix(white, black, 0.5),
			mix(white, black, 0.6),
			mix(white, black, 0.7),
			mix(white, black, 0.8),
			mix(white, black, 0.9),
		],
		darkTint: [
			mix(black, 'black', 0.05),
			mix(black, 'black', 0.1),
			mix(black, 'black', 0.2),
			mix(black, 'black', 0.3),
			mix(black, 'black', 0.4),
			mix(black, 'black', 0.5),
			mix(black, 'black', 0.6),
			mix(black, 'black', 0.7),
			mix(black, 'black', 0.8),
			mix(black, 'black', 0.9),
		],
		lightTint: [
			mix(white, 'white', 0.05),
			mix(white, 'white', 0.1),
			mix(white, 'white', 0.2),
			mix(white, 'white', 0.3),
			mix(white, 'white', 0.4),
			mix(white, 'white', 0.5),
			mix(white, 'white', 0.6),
			mix(white, 'white', 0.7),
			mix(white, 'white', 0.8),
			mix(white, 'white', 0.9),
		],
		dark: [
			mix(white, black, 0.05),
			mix(white, black, 0.1),
			mix(white, black, 0.2),
			mix(white, black, 0.3),
			mix(white, black, 0.9),
			mix(white, black, 0.9),
			mix(white, black, 0.95),
			mix(mix(white, black, 0.95), 'black', 0.2),
			undefined!,
			undefined!,
		],
		gray: [
			mix(white, 'white', 0.4)!,
			undefined!,
			mix(white, black, 0.1),
			undefined!,
			mix(white, black, 0.2),
			undefined!,
			undefined!,
			undefined!,
			undefined!,
			undefined!,
		],
	},
	primaryColor: 'primary',
	primaryShade: 4,
} satisfies PartialDeep<MantineThemeOverride>)
