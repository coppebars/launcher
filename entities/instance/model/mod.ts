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
	name: string
	versionId: string
	path: string
	screen: InstanceScreenFullscreen | InstanceScreenSize
	alloc: number
	args?: string
}

export const $instances = createStore<Instance[]>(
	[
		{
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

persist({ store: $instances })
