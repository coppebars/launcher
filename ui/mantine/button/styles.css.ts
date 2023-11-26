import { style } from '@vanilla-extract/css'

import { vars }  from '@theme/vars'

export const root = style({
	fontWeight: 400,

	selectors: {
		'&[data-variant="filled"], &:not([data-variant])': {
			[vars.darkSelector]: {
				color: vars.colors.black,
			},
		},

		'&[data-loading]': {
			vars: {
				'--_button-loading-overlay-bg': 'transparent',
			},

			[vars.darkSelector]: {
				backgroundColor: vars.colors.dark[4],
				color: vars.colors.dark[3],
			},

			[vars.lightSelector]: {
				backgroundColor: vars.colors.gray[4],
				color: vars.colors.dark[3],
			},
		},
	},
})

export const loader = style({
	[vars.darkSelector]: {
		vars: {
			'--button-color': vars.colors.dark[3],
		},
	},

	[vars.lightSelector]: {
		vars: {
			'--button-color': vars.colors.gray[4],
		},
	},
})

export const section = style({
	selectors: {
		[`${root}[data-loading] &`]: {
			[vars.darkSelector]: {
				color: vars.colors.dark[3],
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
