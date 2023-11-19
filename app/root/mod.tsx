import { useEffect } from 'react'

import { invoke }    from '@tauri-apps/api/primitives'

export function Root() {
	useEffect(() => {
		void (async () => {
			await invoke('lookup_versions', { path: '~/workspaces/launcher/minecraft' }).then(console.log, console.log)
		})()
	})

	return 'Hello!'
}
