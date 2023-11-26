import { combine }     from 'effector'
import { createEvent } from 'effector'
import { createStore } from 'effector'
import { persist }     from 'effector-storage/local'

export interface InstanceScreenFullscreen {
	type: 'fullscreen'
}

export interface InstanceScreenSize {
	type: 'resolution'
	width: number
	height: number
}

export interface Instance {
	id: string
	name: string
	versionId: string
	path: string
	screen: InstanceScreenFullscreen | InstanceScreenSize
	alloc: number
	args?: string
	running?: boolean
}

export const setRunningStatus = createEvent<{ id: string; status: boolean }>('set_running_status')

export const $instances = createStore<Instance[]>(
	[
		{
			id: 'lol',
			path: '/home/limpix/workspaces/launcher/minecraft/instances/main',
			name: 'Main',
			versionId: '1.20.1',
			alloc: 2048,
			screen: {
				type: 'resolution',
				width: 1280,
				height: 720,
			},
		},
	],
	{ name: 'instances' },
)

$instances.on(setRunningStatus, (its, { id, status }) => {
	const it = its.find((it) => it.id === id)

	if (it) {
		it.running = status
	}

	return [...its]
})

export const select = createEvent<string | null>('select_instance')

export const $selectedInstanceId = createStore<string | null>(null, { name: 'selected_instance' })

$selectedInstanceId.on(select, (_, nid) => ($instances.getState().find(({ id }) => id === nid) ? nid : null))

export const $selectedInstance = combine(
	$instances,
	$selectedInstanceId,
	(instances, selectedId) => instances.find(({ id }) => id === selectedId),
	{ skipVoid: false },
)

persist({ store: $instances })
persist({ store: $selectedInstanceId })
