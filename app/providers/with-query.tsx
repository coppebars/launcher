import type { FC }                  from 'react'

import      { QueryClient }         from '@tanstack/react-query'
import      { QueryClientProvider } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function withQuery(Component: FC) {
	return function () {
		return (
			<QueryClientProvider client={queryClient}>
				<Component />
			</QueryClientProvider>
		)
	}
}
