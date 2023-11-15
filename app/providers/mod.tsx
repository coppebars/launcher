import type { FC }         from 'react'

import      { withRouter } from './with-router.tsx'

function apply<T, R>(target: T, fn: (t: T) => R) {
	return fn(target)
}

const providers = [withRouter]

export function withProviders(component: FC) {
	return providers.reduce(apply, component)
}
