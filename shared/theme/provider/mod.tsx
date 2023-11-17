import type { ReactElement }                          from 'react'

import      { ThemeProvider as EmotionThemeProvider } from '@emotion/react'

import      { theme }                                 from '@theme/dark'
import      { Fonts }                                 from '@theme/fonts'
import      { GlobalStyles }                          from '@theme/global'

interface Props {
	children: ReactElement
}

export function ThemeProvider({ children }: Props) {
	return (
		<EmotionThemeProvider theme={theme}>
			<GlobalStyles />
			<Fonts />
			{children}
		</EmotionThemeProvider>
	)
}
