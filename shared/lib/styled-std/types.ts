import type { Theme }                  from '@emotion/react'
import type { InterpolationPrimitive } from '@emotion/serialize'

export type Props<T> = T & { theme: Theme }
export type Fn<T = {}> = (props: Props<T>) => InterpolationPrimitive
