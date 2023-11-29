import type { FC }         from 'react'

import      { withQuery }  from './with-query.tsx'
import      { withRouter } from './with-router.tsx'
import      { withTheme }  from './with-theme.tsx'

function apply<T, R>(target: T, fn: (t: T) => R) {
	return fn(target)
}

const providers = [withRouter, withTheme, withQuery]

export function withProviders(component: FC) {
	return providers.reduce(apply, component)
}
