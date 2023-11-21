import { useEffect } from 'react'
import { useRef }    from 'react'

import { listen }    from '@tauri-apps/api/event'
import { invoke }    from '@tauri-apps/api/primitives'

export function Root() {
	const ref = useRef<HTMLDivElement>(null!)

	useEffect(() => {
		listen('log::0', ({ payload }: any) => {
			console.log(payload)
		})

		void (async () => {
			invoke('mojang_launch', {
				path: '/home/limpix/workspaces/launcher/minecraft',
				uid: '0',
				id: '1.20.1',
				vars: {},
			}).then(console.log, console.log)
		})()
	})

	return (
		<div>
			<div ref={ref} style={{ width: '0', height: '20px', background: 'yellow' }} />
		</div>
	)
}
