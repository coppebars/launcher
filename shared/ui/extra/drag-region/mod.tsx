import { Flex }               from '@mantine/core'
import { IconGripHorizontal } from '@tabler/icons-react'

export function DragRegion() {
	return (
		<Flex
			pos='fixed'
			align='center'
			justify='center'
			style={{ height: '2rem', userSelect: 'none', inset: 0, width: '100vw', zIndex: 1000, pointerEvents: 'none' }}
			data-tauri-drag-region
		>
			<IconGripHorizontal color='gray' style={{ userSelect: 'none' }} />
		</Flex>
	)
}
