import { Flex }               from '@mantine/core'
import { IconGripHorizontal } from '@tabler/icons-react'

export function DragRegion() {
	return (
		<Flex
			pos='fixed'
			align='center'
			justify='center'
			style={{
				height: '2rem',
				userSelect: 'none',
				top: 0,
				left: '50%',
				translate: '-50% 0',
				width: '80vw',
				zIndex: 1000,
			}}
			data-tauri-drag-region
		>
			<IconGripHorizontal color='gray' style={{ userSelect: 'none', pointerEvents: 'none' }} />
		</Flex>
	)
}
