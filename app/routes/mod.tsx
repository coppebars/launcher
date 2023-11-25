import { VersionsPage } from '@page/versions'
import { Route }        from 'react-router-dom'
import { Routes }       from 'react-router-dom'

import { HomePage }     from '@page/home'

export function AppRoutes() {
	return (
		<Routes>
			<Route index element={<HomePage />} />
			<Route path='/versions' element={<VersionsPage />} />
		</Routes>
	)
}
