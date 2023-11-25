import type { ReactNode } from 'react'

import      { Box }       from '@mantine/core'

interface Props {
	children: ReactNode
}

export function PaddedLayout(props: Props) {
	const { children } = props

	return <Box p={8}>{children}</Box>
}
