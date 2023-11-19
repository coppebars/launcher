import { useEffect } from 'react'

import { invoke }    from '@tauri-apps/api/primitives'

export function Root() {
	useEffect(() => {
		void (async () => {
			console.log(await invoke('lookup_versions', { path: '/home/limpix/workspaces/launcher/minecraft' }))
		})()
	})

	return 'Hello!'
}
