import * as lf from 'localforage'

const LFI = Symbol('LFI')

export function database(table: string, db = 'app') {
	const key = `${db}::${table}`

	const lfi: Record<string, ReturnType<typeof lf.createInstance>> = Reflect.get(globalThis, LFI)

	if (!Reflect.has(globalThis, LFI)) {
		Reflect.set(globalThis, LFI, {})
	}

	if (lfi[key]) {
		return lfi[key]
	}

	const newInstance = lf.createInstance({
		driver: lf.INDEXEDDB,
		name: db,
		storeName: table,
	})

	lfi[key] = newInstance

	return newInstance
}

export function seq<T>(table: string, dbName = 'app') {
	const db = database(table, dbName)

	const data: T[] = []

	db.iterate((it: T) => data.push(it))

	return new Proxy(data, {
		set(target, p: any, newValue: T, receiver: any): boolean {
			db.setItem(p.toString(), newValue)

			return Reflect.set(target, p, newValue, receiver)
		},
	})
}
