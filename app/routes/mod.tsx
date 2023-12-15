import { Route }        from 'react-router-dom'
import { Routes }       from 'react-router-dom'

import { HomePage }     from '@page/home'
import { ProfilesPage } from '@page/profiles'
import { SettingsPage } from '@page/settings'

export function AppRoutes() {
	return (
		<Routes>
			<Route index element={<HomePage />} />
			<Route path='/settings' element={<SettingsPage />} />
			<Route path='/profiles' element={<ProfilesPage />} />
		</Routes>
	)
}
