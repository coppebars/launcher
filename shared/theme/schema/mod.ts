import { mergeMantineTheme } from '@mantine/core'

import { Button }            from '@ui/core'

import { base }              from './base.ts'

export const theme = mergeMantineTheme(base as never, {
	components: {
		Button,
	},
})
