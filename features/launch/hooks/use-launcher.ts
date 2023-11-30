import { useCallback }          from 'react'
import { useEffect }            from 'react'
import { useState }             from 'react'

import { notifications }        from '@mantine/notifications'
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

	const reset = useCallback(() => {
		setError(undefined)
	}, [])

	const launch = useCallback(() => {
		reset()

		if (instance) {
			setRunningStatus({ id: instance.id, status: true })

			coreLaunch({
				root: settings.rootPath,
				versionId: instance.version.vid,
				provider: instance.version.provider === 'local' ? 'mojang' : instance.version.provider,
				logbackId: 'unknown',
				vars: {
					auth_player_name: 'LIMPIX31',
					auth_uuid: 'bd983a9c-0622-42dc-a0c2-47c71bd4f21b',
					game_directory: instance.path,
					user_type: 'msa',
					minecraft_services_host: 'https://nodium.ru:9000/',
					minecraft_auth_host: 'https://nodium.ru:9000/',
					minecraft_session_host: 'https://nodium.ru:9000/',
					minecraft_account_host: 'https://nodium.ru:9000/',
					width: instance.width?.toString(10) ?? '1280',
					height: instance.height?.toString(10) ?? '720',
				},
			})
				.catch(setError)
				.finally(() => setRunningStatus({ id: instance.id, status: false }))
		}
	}, [instance, reset, settings.rootPath])

	useEffect(() => {
		if (error) {
			notifications.show({
				withCloseButton: true,
				autoClose: 8000,
				title: 'Launch failed',
				message: error,
				color: 'red',
			})
		}
	}, [error])

	return { ready, running, error, launch, reset }
}
