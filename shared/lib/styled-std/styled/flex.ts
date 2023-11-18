import type { Theme } from '@emotion/react'

import      { make }  from 'styled-std'

import      { std }   from '../prop.ts'
import type { Paths } from '../types.ts'

export interface FlexProps {
	flex?: boolean
	row?: boolean
	column?: boolean
	gap?: Paths<Theme>
}

const { when } = std<FlexProps>()

const display = when('flex', () => ({
	display: 'flex',
}))

const row = when('row', () => ({
	display: 'flex',
	flexDirection: 'row',
}))

const column = when('column', () => ({
	display: 'flex',
	flexDirection: 'column',
}))

const gap = when('gap', ({ gap }) => ({
	gap,
}))

export const flex = make(display, row, column, gap)
