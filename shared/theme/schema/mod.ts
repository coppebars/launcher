import { mergeMantineTheme } from '@mantine/core'

import { Button }            from 'ui/mantine'
import { Checkbox }          from 'ui/mantine'

import { base }              from './base.ts'

export const theme = mergeMantineTheme(base as never, {
	components: {
		Button,
		Checkbox,
	},
})
