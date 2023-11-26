import { Alert }          from '@mantine/core'
import { Flex }           from '@mantine/core'
import { Loader }         from '@mantine/core'
import { LoadingOverlay } from '@mantine/core'
import { Stack }          from '@mantine/core'
import { useQuery }       from '@tanstack/react-query'

import { lookupVersions } from 'core'

import * as styles        from './styles.css.ts'

export function VersionsWidget() {
	const { data, status, error } = useQuery({
		queryKey: ['local_versions'],
		queryFn: () => lookupVersions({ path: '/home/limpix/workspaces/launcher/minecraft' }),
		retry: 0,
	})

	if (status === 'pending') {
		return (
			<LoadingOverlay visible overlayProps={{ opacity: 0 }} loaderProps={{ children: <Loader color='primary' /> }} />
		)
	}

	if (status === 'error') {
		return (
			<Alert variant='filled' color='red' title='Error'>
				{error.message}
			</Alert>
		)
	}

	return (
		<Stack gap={8}>
			{data.map(({ id, icon }) => (
				<Flex key={id} gap={16} align='center' className={styles.listItem}>
					{icon ? <img alt='icon' src={icon} className={styles.itemImage} /> : <div className={styles.itemImage} />}
					{id}
				</Flex>
			))}
		</Stack>
	)
}
