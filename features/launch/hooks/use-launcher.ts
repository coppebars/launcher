import { useCallback }           from 'react'
import { useEffect }             from 'react'
import { useState }              from 'react'

import { notifications }         from '@mantine/notifications'
import { useUnit }               from 'effector-react'

import { $runtimeInstancesData } from '@entity/instance'
import { $selectedInstance }     from '@entity/instance'
import { setRunningStatus }      from '@entity/instance'
import { $nickname }             from '@entity/profile'
import { $settings }             from '@entity/settings'
import { launch as coreLaunch }  from 'core'

export function useLauncher() {
	const settings = useUnit($settings)
	const instance = useUnit($selectedInstance)
	const runtimeData = useUnit($runtimeInstancesData)
	const nickname = useUnit($nickname)

	const ready = Boolean(instance)
	const running = Boolean(instance?.id && runtimeData[instance?.id]?.running)

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
					auth_player_name: nickname,
					auth_uuid: 'bd983a9c-0622-42dc-a0c2-47c71bd4f21b',
					auth_access_token: 'null',
					game_directory: instance.path,
					user_type: 'msa',
					width: instance.width?.toString(10) ?? '1280',
					height: instance.height?.toString(10) ?? '720',
				},
			})
				.catch(setError)
				.finally(() => setRunningStatus({ id: instance.id, status: false }))
		}
	}, [instance, nickname, reset, settings.rootPath])

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
