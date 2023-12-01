import type { FC }            from 'react'

import      { Notifications } from '@mantine/notifications'

import      { ThemeProvider } from '@theme/provider'

export function withTheme(Component: FC) {
	return function () {
		return (
			<ThemeProvider>
				<Notifications limit={5} position='bottom-right' zIndex={1000} />
				<Component />
			</ThemeProvider>
		)
	}
}
