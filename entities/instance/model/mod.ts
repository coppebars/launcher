/** @purpose: Type only */
import type { Version }     from '@entity/version'
import      { combine }     from 'effector'
import      { createEvent } from 'effector'
import      { createStore } from 'effector'
import      { persist }     from 'effector-storage/local'
import      { nanoid }      from 'nanoid/non-secure'

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
	running?: boolean
}

export const setRunningStatus = createEvent<{ id: string; status: boolean }>('set_running_status')
export const update = createEvent<{ id: string; payload: Partial<Instance> }>('update')
export const add = createEvent<Omit<Instance, 'id'>>('add')

export const $instances = createStore<Instance[]>([], { name: 'instances' })

$instances.on(setRunningStatus, (its, { id, status }) => {
	const it = its.find((it) => it.id === id)

	if (it) {
		it.running = status
	}

	return [...its]
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
