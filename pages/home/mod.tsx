import { useEffect }                from 'react'
import { useRef }                   from 'react'

import { Button }                   from '@mantine/core'
import { Text }                     from '@mantine/core'
import { Progress }                 from '@mantine/core'
import { Stack }                    from '@mantine/core'
import { Flex }                     from '@mantine/core'
import { Select }                   from '@mantine/core'
import { IconPlayerPlayFilled }     from '@tabler/icons-react'
import { Event }                    from '@tauri-apps/api/event'
import { listen }                   from '@tauri-apps/api/event'
import { useUnit }                  from 'effector-react'

import { $instances }               from '@entity/instance'
import { $selectedInstanceId }      from '@entity/instance'
import { select }                   from '@entity/instance'
import { CreateOrEditInstanceForm } from '@feature/create-or-edit-instance'
import { useLauncher }              from '@feature/launch'
import { PaddedLayout }             from '@layout/padded'

export function HomePage() {
	const instances = useUnit($instances)
	const selectedId = useUnit($selectedInstanceId)
	const progress = useRef<HTMLDivElement | null>(null)

	const { ready, running, launch } = useLauncher()

	useEffect(() => {
		let unlisten: (() => void) | undefined

		listen('prepare', ({ payload }: Event<any>) => {
			const segment = progress.current

			if (payload.finish && segment) {
				console.log(payload.finish)
				segment.style.setProperty(
					'--progress-section-width',
					`${(payload.finish.progress / payload.finish.total) * 100}%`,
				)
			}
		}).then((it) => {
			unlisten = it
		})

		return unlisten
	}, [])

	useEffect(() => {
		if (!running) {
			const segment = progress.current

			if (segment) {
				segment.style.setProperty('--progress-section-width', '0%')
			}
		}
	}, [running])

	return (
		<PaddedLayout>
			<Flex direction='column' justify='space-between' style={{ height: '100%' }}>
				<div />
				<Text px={320} style={{ opacity: 0.2 }}>
					Thank you for participating in the testing. We appreciate any action towards improving the app. The app still
					has a lot of bugs, please be more patient.
				</Text>
				<Stack gap={8}>
					<Flex direction='row' gap={8}>
						<Select
							nothingFoundMessage='Nothing found...'
							placeholder='Select instance'
							checkIconPosition='right'
							searchable
							data={instances.map(({ name, id }) => ({ label: name, value: id }))}
							value={selectedId}
							onChange={select}
						/>
						<Button
							disabled={!ready}
							loading={running}
							loaderProps={{ type: 'dots' }}
							onClick={launch}
							rightSection={<IconPlayerPlayFilled size={16} />}
						>
							Launch
						</Button>
					</Flex>
					<Progress.Root radius='xs'>
						<Progress.Section ref={progress} value={0} animated />
					</Progress.Root>
				</Stack>
			</Flex>
			<CreateOrEditInstanceForm />
		</PaddedLayout>
	)
}
