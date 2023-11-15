/* eslint-disable no-dupe-class-members, lines-between-class-members */

import lf              from 'localforage'
import { PartialDeep } from 'type-fest'

type LocalForage = ReturnType<typeof lf.createInstance>

type TracePath<T> = T extends object
	? { [K in keyof T]: `${Exclude<K, symbol>}${'' | `::${TracePath<T[K]>}`}` }[keyof T]
	: never

type Query<T extends object, P extends TracePath<T>> = P extends `${infer F extends Exclude<
	keyof T,
	symbol
>}::${infer R}`
	? T[F] extends object
		? Query<T[F], R extends TracePath<T[F]> ? R : never>
		: never
	: P extends keyof T
	? T[P]
	: never

export class LFw<T extends object> {
	#lf: LocalForage

	constructor(lf: LocalForage) {
		this.#lf = lf
	}

	async #update(v: object, path?: string, f = true, s = false) {
		if (f && path) {
			await this.#lf.setItem(path, v)
		}

		await Promise.all(
			// eslint-disable-next-line consistent-return
			Object.entries(v).map(async ([key, value]) => {
				const newPath = path ? `${path}::${key}` : key

				if (typeof value === 'object' && !Array.isArray(value)) {
					return this.#update(value, newPath, false)
				}

				if (!s) {
					await this.#lf.setItem(newPath, value)
				} else if ((await this.#lf.getItem(newPath)) === null) {
					await this.#lf.setItem(newPath, value)
				}
			}),
		)
	}

	applyDefailts(v: T) {
		return this.#update(v, undefined, true, true)
	}

	update<Q extends TracePath<T>>(path: Q, v: PartialDeep<Query<T, Q>>): Promise<void>
	update(v: PartialDeep<T>): Promise<void>
	async update(a: any, b?: any): Promise<void> {
		if (b) {
			return this.#update(b, a)
		}

		return this.#update(a)
	}

	async #collect(path: string, keys: string[]) {
		const collector: any = {}

		const values = await Promise.all(
			keys.map(
				async (it) =>
					[it.split('::').filter((it) => it.length !== 0), (await this.#lf.getItem(path + it)) as any] as const,
			),
		)

		values.forEach(([path, value]) => {
			let target = collector
			const key = path.pop()!
			// eslint-disable-next-line no-multi-assign,no-return-assign
			path.forEach((it) => (target = collector[it] ??= {}))

			target[key] = value
		})

		return collector
	}

	async query<Q extends TracePath<T>>(path: Q = '' as any): Promise<Query<T, Q>> {
		const allkeys = await this.#lf.keys()
		const keys = allkeys.filter((it) => it.startsWith(path)).map((it) => it.slice(path.length))

		if (keys.length === 1 && keys[0] === '') {
			return (await this.#lf.getItem(path))!
		}

		return this.#collect(path, keys)
	}

	async drop() {
		return this.#lf.clear()
	}
}

interface CreateDatabaseOptions<T> {
	name: string
	sub: string
	version?: number
	defaults: T
}

export async function createDatabase<T extends object>(options: CreateDatabaseOptions<T>) {
	const lfw = new LFw<T>(
		lf.createInstance({
			driver: lf.INDEXEDDB,
			name: options.name,
			version: options.version,
			storeName: options.sub,
		}),
	)

	await lfw.applyDefailts(options.defaults)

	return lfw
}
