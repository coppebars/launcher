import { useCallback }          from 'react'
import { useState }             from 'react'

import { useUnit }              from 'effector-react'

import { $selectedInstance }    from '@entity/instance'
import { setRunningStatus }     from '@entity/instance'
import { $settings }            from '@entity/settings'
import { launch as coreLaunch } from 'core'

export function useLauncher() {
	const settings = useUnit($settings)
	const instance = useUnit($selectedInstance)

	const ready = Boolean(instance)
	const running = Boolean(instance?.running)

	const [error, setError] = useState<string | undefined>()

	const launch = useCallback(() => {
		if (instance) {
			setRunningStatus({ id: instance.id, status: true })

			coreLaunch({
				root: settings.rootPath,
				versionId: instance.versionId,
				provider: 'mojang',
				logbackId: 'unknown',
				vars: {
					auth_player_name: 'LIMPIX31',
					auth_uuid: 'bd983a9c-0622-42dc-a0c2-47c71bd4f21b',
					game_directory: '/home/limpix/workspaces/launcher/minecraft/instances/main',
					user_type: 'msa',
					minecraft_services_host: 'https://nodium.ru:9000/',
					minecraft_auth_host: 'https://nodium.ru:9000/',
					minecraft_session_host: 'https://nodium.ru:9000/',
					minecraft_account_host: 'https://nodium.ru:9000/',
				},
			})
				.catch(setError)
				.finally(() => setRunningStatus({ id: instance.id, status: false }))
		}
	}, [instance, settings.rootPath])

	const reset = useCallback(() => {
		setError(undefined)
	}, [])

	return { ready, running, error, launch, reset }
}
