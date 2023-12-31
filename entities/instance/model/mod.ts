/** @purpose: Type only */
import      { combine }     from 'effector'
import      { createEvent } from 'effector'
import      { createStore } from 'effector'
import      { persist }     from 'effector-storage/local'
import      { nanoid }      from 'nanoid/non-secure'

import type { Version }     from '@entity/version'

export interface Instance {
	id: string
	name: string
	version: Version
	path: string
	fullscreen?: boolean
	width?: number
	height?: number
	alloc: number
	extraArgs?: string
}

export interface RuntimeInstanceData {
	running?: boolean
}

export const $instances = createStore<Instance[]>([], { name: 'instances' })
export const $selectedInstanceId = createStore<string | null>(null, { name: 'selected_instance' })
export const $runtimeInstancesData = createStore<Record<string, RuntimeInstanceData>>(
	{},
	{ name: 'runtime_instances_data' },
)

export const update = createEvent<{ id: string; payload: Partial<Instance> }>('update')
export const add = createEvent<Omit<Instance, 'id'>>('add')
export const remove = createEvent<string>('remove')
export const select = createEvent<string | null>('select_instance')
export const setRunningStatus = createEvent<{ id: string; status: boolean }>('set_running_status')

$runtimeInstancesData.on(setRunningStatus, (its, { id, status }) => {
	const it = its[id]

	return { ...its, [id]: { ...it, running: status } }
})

$instances.on(update, (its, { id, payload }) => {
	const it = its.find((it) => it.id === id)

	if (it) {
		Object.keys(payload).forEach((key) => {
			Reflect.set(it, key, Reflect.get(payload, key))
		})
	}

	return [...its]
})

$instances.on(add, (its, instance) => [...its, { id: nanoid(8), ...instance }])

$instances.on(remove, (its, id) => its.filter((it) => it.id !== id))

$selectedInstanceId.on(select, (_, nid) => ($instances.getState().find(({ id }) => id === nid) ? nid : null))

export const $selectedInstance = combine(
	$instances,
	$selectedInstanceId,
	(instances, selectedId) => instances.find(({ id }) => id === selectedId),
	{ skipVoid: false },
)

persist({ store: $instances })
persist({ store: $selectedInstanceId })
