import { useCallback }              from 'react'
import { useEffect }                from 'react'
import { useState }                 from 'react'
import { useRef }                   from 'react'

import { Button }                   from '@mantine/core'
import { Progress }                 from '@mantine/core'
import { Stack }                    from '@mantine/core'
import { Flex }                     from '@mantine/core'
import { Select }                   from '@mantine/core'
import { IconPlayerPlayFilled }     from '@tabler/icons-react'
import { Event }                    from '@tauri-apps/api/event'
import { listen }                   from '@tauri-apps/api/event'
import { useUnit }                  from 'effector-react'

import { $instances }               from '@entity/instance'
import { Instance }                 from '@entity/instance'
import { InstanceListItem }         from '@entity/instance'
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

	const [editInstace, setEditInstance] = useState<Instance | undefined>()
	const [openDrawer, setOpenDrawer] = useState(false)

	const editInstance = useCallback((it: Instance) => {
		setEditInstance(it)
		setOpenDrawer(true)
	}, [])

	return (
		<PaddedLayout>
			<Flex direction='column' justify='space-between' style={{ height: '100%' }}>
				<Stack style={{ flexGrow: 1 }}>
					{instances.map((it) => (
						<InstanceListItem instance={it} onEdit={editInstance} />
					))}
				</Stack>
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
			<CreateOrEditInstanceForm edit={editInstace} opened={openDrawer} onClose={() => setOpenDrawer(false)} />
		</PaddedLayout>
	)
}
