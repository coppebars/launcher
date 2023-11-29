import { ActionIcon } from '@mantine/core'
import { MenuItem }   from '@mantine/core'
import { Box }        from '@mantine/core'
import { Menu }       from '@mantine/core'
import { Flex }       from '@mantine/core'
import { IconDots }   from '@tabler/icons-react'

import { Instance }   from '@entity/instance'

import * as styles    from './styles.css.ts'

interface Props {
	instance: Instance
	onEdit?: (instance: Instance) => void
}

export function ListItem(props: Props) {
	const { instance, onEdit } = props

	return (
		<Flex gap={16} justify='space-between' className={styles.listItem}>
			<Flex gap={16} align='center'>
				<Box>{instance.name}</Box>
				<Box>{instance.versionId}</Box>
			</Flex>
			<Flex gap={8} justify='end' align='center'>
				<Menu shadow='md' width={100}>
					<Menu.Target>
						<ActionIcon variant='transparent' color='gray'>
							<IconDots style={{ width: '70%', height: '70%' }} stroke={1.5} />
						</ActionIcon>
					</Menu.Target>

					<Menu.Dropdown>
						<MenuItem disabled={!onEdit} onClick={() => onEdit?.(instance)}>
							Edit
						</MenuItem>
					</Menu.Dropdown>
				</Menu>
			</Flex>
		</Flex>
	)
}
