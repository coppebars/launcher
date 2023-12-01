import { keyframes } from '@vanilla-extract/css'
import { style }     from '@vanilla-extract/css'

import { vars }      from '@theme/vars'

const loadingAnimation = keyframes({
	from: {
		backgroundPositionY: '0px',
	},
	to: {
		backgroundPositionY: '11.5px',
	},
})

export const root = style({
	fontWeight: 400,

	selectors: {
		'&[data-variant="filled"][data-disabled], &:not([data-variant])[data-disabled]': {
			[vars.lightSelector]: {
				backgroundColor: vars.colors.gray[4],
				color: vars.colors.darkShade[4],
			},

			[vars.darkSelector]: {
				backgroundColor: vars.colors.darkShade[3],
				color: vars.colors.darkShade[5],
			},
		},

		'&[data-variant="filled"]:not([data-disabled]), &:not([data-variant]):not([data-disabled])': {
			[vars.darkSelector]: {
				color: vars.colors.black,
			},
		},

		'&[data-loading]': {
			vars: {
				'--_button-loading-overlay-bg': 'transparent',
			},

			animation: `${loadingAnimation} .5s linear infinite`,

			[vars.darkSelector]: {
				backgroundImage: `repeating-linear-gradient(-45deg,${vars.colors.darkShade[3]},${vars.colors.darkShade[3]} 4px,${vars.colors.darkShade[4]} 4px,${vars.colors.darkShade[4]} 8px)`,
			},

			[vars.lightSelector]: {
				backgroundImage: `repeating-linear-gradient(-45deg,${vars.colors.gray[4]},${vars.colors.gray[4]} 4px,${vars.colors.gray[2]} 4px,${vars.colors.gray[2]} 8px)`,
			},
		},
	},
})

export const loader = style({
	display: 'none',
})

export const section = style({
	selectors: {
		[`${root}[data-loading] &`]: {
			[vars.darkSelector]: {
				color: vars.colors.darkShade[5],
			},

			[vars.lightSelector]: {
				color: vars.colors.dark[6],
			},

			opacity: 0.3,
		},
	},
})

export const label = style({
	selectors: {
		[`${root}[data-loading] &`]: {
			vars: {
				'--_button-label-opacity': '1',
			},
		},
	},
})
