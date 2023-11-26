import { useEffect }            from 'react'

import { Button }               from '@mantine/core'
import { Flex }                 from '@mantine/core'
import { Select }               from '@mantine/core'
import { IconPlayerPlayFilled } from '@tabler/icons-react'
import { useUnit }              from 'effector-react'

import { $instances }           from '@entity/instance'
import { $settings }            from '@entity/settings'
import { PaddedLayout }         from '@layout/padded'
import { launch }               from 'core'

export function HomePage() {
	const instances = useUnit($instances)

	useEffect(() => {
		// launch({
		// 	logbackId: 'unknown',
		// 	versionId: '1.20.1',
		// 	provider: 'mojang',
		// 	root: $settings.getState().rootPath,
		// 	vars: {
		// 		auth_player_name: 'LIMPIX31',
		// 		auth_uuid: 'bd983a9c-0622-42dc-a0c2-47c71bd4f21b',
		// 		game_directory: '/home/limpix/workspaces/launcher/minecraft/instances/main',
		// 		user_type: 'msa',
		// 		minecraft_services_host: 'https://nodium.ru:9000/',
		// 		minecraft_auth_host: 'https://nodium.ru:9000/',
		// 		minecraft_session_host: 'https://nodium.ru:9000/',
		// 		minecraft_account_host: 'https://nodium.ru:9000/',
		// 	},
		// }).then(console.log, console.error)
	}, [])

	return (
		<PaddedLayout>
			<Flex direction='column' justify='space-between' style={{ height: '100%' }}>
				<div />
				<div />
				<Flex direction='row' gap={8}>
					<Select
						nothingFoundMessage='Nothing found...'
						placeholder='Pick value'
						checkIconPosition='right'
						searchable
						data={instances.map(({ name }) => ({ label: name, value: name }))}
					/>
					<Button loaderProps={{ type: 'dots' }} rightSection={<IconPlayerPlayFilled size={16} />}>
						Launch
					</Button>
				</Flex>
			</Flex>
		</PaddedLayout>
	)
}
