import type { FC }             from 'react'

import      { ModalsProvider } from '@mantine/modals'

export function withModals(Component: FC) {
	return function () {
		return (
			<ModalsProvider labels={{ confirm: 'Submit', cancel: 'Cancel' }}>
				<Component />
			</ModalsProvider>
		)
	}
}
