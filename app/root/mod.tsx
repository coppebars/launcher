import { SidedLayout } from '@layout/sided'
import { Box }         from '@mantine/core'
import { Stack }       from '@mantine/core'

export function Root() {
	return <SidedLayout sidebar={<Stack p={8} />} view={<Box p={8} />} />
}
