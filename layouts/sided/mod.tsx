import type { ReactElement } from 'react'

import      { Box }          from '@mantine/core'
import      { Flex }         from '@mantine/core'

import      * as styles      from './style.css.ts'

interface Props {
	sidebar: ReactElement
	view: ReactElement
	sidebarWidth?: string
}

export function SidedLayout(props: Props) {
	const { sidebar, view } = props
	const { sidebarWidth: width = '64px' } = props

	return (
		<Flex direction='row' className={styles.container}>
			<Box className={styles.sidebar} style={{ width }}>
				{sidebar}
			</Box>
			<Box className={styles.view}>{view}</Box>
		</Flex>
	)
}
