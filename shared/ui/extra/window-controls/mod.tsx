import { Box }    from '@mantine/core'
import { Flex }   from '@mantine/core'
import { invoke } from '@tauri-apps/api/primitives'

const INTERNALS = Reflect.get(window, '__TAURI_INTERNALS__')

const currentWindow = INTERNALS.metadata.currentWindow.label

function close() {
	invoke('plugin:window|close', {
		label: currentWindow,
	})
}

function minimize() {
	invoke('plugin:window|minimize', {
		label: currentWindow,
	})
}

export function WindowControls() {
	return (
		<Flex w='100%' justify='space-between'>
			<Box w='1.4rem' h='0.6rem' bg='red' style={{ cursor: 'pointer', borderRadius: '100px' }} onClick={close} />
			<Box w='1.4rem' h='0.6rem' bg='dark' style={{ cursor: 'pointer', borderRadius: '100px' }} onClick={minimize} />
		</Flex>
	)
}
