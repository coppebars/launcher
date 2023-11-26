import { SettingsPage } from '@page/settings'
import { Route }        from 'react-router-dom'
import { Routes }       from 'react-router-dom'

import { HomePage }     from '@page/home'
import { VersionsPage } from '@page/versions'

export function AppRoutes() {
	return (
		<Routes>
			<Route index element={<HomePage />} />
			<Route path='/versions' element={<VersionsPage />} />
			<Route path='/settings' element={<SettingsPage />} />
		</Routes>
	)
}
