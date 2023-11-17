import { Fn } from './types.ts'

export const make = (...funcs: Fn[]): Fn =>
	(...args) =>
		Object.assign({}, ...funcs.map((func) => func(...args)))
