import      { Suspense }       from 'react'
import      { use }            from 'react'

import      { Alert }          from '@mantine/core'
import      { Flex }           from '@mantine/core'
import      { Loader }         from '@mantine/core'
import      { LoadingOverlay } from '@mantine/core'
import      { Stack }          from '@mantine/core'
import      { ErrorBoundary }  from 'react-error-boundary'

import type { Version }        from 'core'
import      { lookupVersions } from 'core'

import      * as styles        from './styles.css.ts'

function List() {
	const versions: Version[] = use(lookupVersions({ path: '/home/limpix/workspaces/launcher/minecraft' }))

	return (
		<Stack gap={8}>
			{versions.map(({ id, icon }) => (
				<Flex gap={16} align='center' className={styles.listItem}>
					{icon ? <img alt='icon' src={icon} className={styles.itemImage} /> : <div className={styles.itemImage} />}
					{id}
				</Flex>
			))}
		</Stack>
	)
}

export function VersionsWidget() {
	return (
		<ErrorBoundary
			fallbackRender={({ error }) => (
				<Alert variant='filled' color='red' title='Alert title'>
					{error}
				</Alert>
			)}
		>
			<Suspense
				fallback={
					<LoadingOverlay
						visible
						overlayProps={{ opacity: 0 }}
						loaderProps={{ children: <Loader color='primary' /> }}
					/>
				}
			>
				<List />
			</Suspense>
		</ErrorBoundary>
	)
}
