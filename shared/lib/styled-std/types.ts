import type { Theme }                  from '@emotion/react'
import type { InterpolationPrimitive } from '@emotion/serialize'

export type Props<T = {}> = T & { theme: Theme }
export type Fn<T = {}> = (props: Props<T>) => InterpolationPrimitive

export type Paths<T> = T extends Array<infer U>
	? `${Paths<U>}`
	: T extends object
	? {
			[K in keyof T & (string | number)]: K extends string ? `${K}` | `${K}.${Paths<T[K]>}` : never
	  }[keyof T & (string | number)]
	: never
