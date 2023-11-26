import { Button }               from '@mantine/core'
import { Flex }                 from '@mantine/core'
import { Select }               from '@mantine/core'
import { IconPlayerPlayFilled } from '@tabler/icons-react'
import { useUnit }              from 'effector-react'

import { $instances, $selectedInstanceId } from '@entity/instance'
import { select }               from '@entity/instance'
import { useLauncher }          from '@feature/launch'
import { PaddedLayout }         from '@layout/padded'

export function HomePage() {
	const instances = useUnit($instances)
	const selectedId = useUnit($selectedInstanceId)

	const { ready, running, launch } = useLauncher()
	console.log(running)

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
						data={instances.map(({ name, id }) => ({ label: name, value: id }))}
						value={selectedId}
						onChange={select}
					/>
					<Button
						disabled={!ready}
						loading={running}
						loaderProps={{ type: 'dots' }}
						onClick={launch}
						rightSection={<IconPlayerPlayFilled size={16} />}
					>
						Launch
					</Button>
				</Flex>
			</Flex>
		</PaddedLayout>
	)
}
