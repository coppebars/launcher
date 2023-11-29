import type { ReactNode } from 'react'

import      { Flex }      from '@mantine/core'

interface Props {
	children: ReactNode[]
}

export function SidebarLayout(props: Props) {
	const { children } = props

	return (
		<Flex
			direction='column'
			p={8}
			gap={6}
			justify='space-between'
			align='center'
			style={{ height: '100%', userSelect: 'none' }}
		>
			{children}
		</Flex>
	)
}
