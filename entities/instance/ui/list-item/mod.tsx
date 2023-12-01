import { ActionIcon } from '@mantine/core'
import { Box }        from '@mantine/core'
import { Flex }       from '@mantine/core'
import { Menu }       from '@mantine/core'
import { MenuItem }   from '@mantine/core'
import { rem }        from '@mantine/core'
import { IconDots }   from '@tabler/icons-react'
import { IconEdit }   from '@tabler/icons-react'
import { IconTrash }  from '@tabler/icons-react'

import { Instance }   from '@entity/instance'

import * as styles    from './styles.css.ts'

interface Props {
	instance: Instance
	onEdit?: (instance: Instance) => void
	onRemove?: (instance: Instance) => void
}

export function ListItem(props: Props) {
	const { instance, onEdit, onRemove } = props

	return (
		<Flex gap={16} justify='space-between' className={styles.listItem}>
			<Flex gap={16} align='center'>
				<Box>{instance.name}</Box>
				<Box>{instance.version.vid}</Box>
			</Flex>
			<Flex gap={8} justify='end' align='center'>
				<Menu shadow='md' width={140} position='left-start'>
					<Menu.Target>
						<ActionIcon variant='transparent' color='gray'>
							<IconDots style={{ width: '70%', height: '70%' }} stroke={1.5} />
						</ActionIcon>
					</Menu.Target>

					<Menu.Dropdown>
						<MenuItem
							disabled={!onEdit}
							onClick={() => onEdit?.(instance)}
							leftSection={<IconEdit style={{ width: rem(14), height: rem(14) }} />}
						>
							Edit
						</MenuItem>

						<MenuItem
							color='red'
							disabled={!onRemove}
							onClick={() => onRemove?.(instance)}
							leftSection={<IconTrash style={{ width: rem(14), height: rem(14) }} />}
						>
							Remove
						</MenuItem>
					</Menu.Dropdown>
				</Menu>
			</Flex>
		</Flex>
	)
}
