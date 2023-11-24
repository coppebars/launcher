import { HomePage } from '@page/home'
import { Route }    from 'react-router-dom'
import { Routes }   from 'react-router-dom'

export function AppRoutes() {
	return (
		<Routes>
			<Route index element={<HomePage />} />
		</Routes>
	)
}
