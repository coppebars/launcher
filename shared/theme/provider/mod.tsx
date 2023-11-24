import type { ReactElement }                   from 'react'

import      { localStorageColorSchemeManager } from '@mantine/core'
import      { MantineProvider }                from '@mantine/core'

import      { theme }                          from '@theme/schema'

const colorSchemeManager = localStorageColorSchemeManager({ key: 'color-scheme' })

interface Props {
	children: ReactElement
}

export function ThemeProvider({ children }: Props) {
	return (
		<MantineProvider defaultColorScheme='dark' colorSchemeManager={colorSchemeManager} theme={theme}>
			{children}
		</MantineProvider>
	)
}
