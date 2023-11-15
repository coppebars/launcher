import type { FC }            from 'react'

import      { ThemeProvider } from '@theme/provider'

export function withTheme(Component: FC) {
	return function () {
		return (
			<ThemeProvider>
				<Component />
			</ThemeProvider>
		)
	}
}
