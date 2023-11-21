import { useEffect, useRef } from 'react'

import { listen }    from '@tauri-apps/api/event'
import { invoke }    from '@tauri-apps/api/primitives'

export function Root() {
	const ref = useRef<HTMLDivElement>(null!)

	useEffect(() => {
		listen('prepare', ({ payload }: any) => {
			if (payload.finish) {
				ref.current.style.width = `${(payload.finish.progress / payload.finish.total) * 100}%`
			}
		})

		void (async () => {
			invoke('mojang_prepare', { path: '/home/limpix/workspaces/launcher/minecraft', id: '1.20.1' }).then(
				console.log,
				console.log,
			)
		})()
	})

	return (
		<div>
			<div ref={ref} style={{ width: '0', height: '20px', background: 'yellow' }} />
		</div>
	)
}
