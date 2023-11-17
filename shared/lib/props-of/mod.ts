import { ReactElement } from 'react'

export type PropsOf<T> = T extends (...args: infer A) => ReactElement ? (A[0] extends never ? {} : A[0]) : {}
