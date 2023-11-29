import { Route }        from 'react-router-dom'
import { Routes }       from 'react-router-dom'

import { HomePage }     from '@page/home'
import { SettingsPage } from '@page/settings'

export function AppRoutes() {
	return (
		<Routes>
			<Route index element={<HomePage />} />
			<Route path='/settings' element={<SettingsPage />} />
		</Routes>
	)
}
