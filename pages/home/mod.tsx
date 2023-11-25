import { Button }               from '@mantine/core'
import { Flex }                 from '@mantine/core'
import { Select }               from '@mantine/core'
import { IconPlayerPlayFilled } from '@tabler/icons-react'
import { useUnit }              from 'effector-react'

import { $instances }           from '@entity/instance'
import { PaddedLayout }         from '@layout/padded'

export function HomePage() {
	const instances = useUnit($instances)

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
