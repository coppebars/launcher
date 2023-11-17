import type { FC }           from 'react'

import      { MemoryRouter } from 'react-router-dom'

export function withRouter(Component: FC) {
	return function () {
		return (
			<MemoryRouter>
				<Component />
			</MemoryRouter>
		)
	}
}
