import type { FC }         from 'react'

import      { withModals } from './with-modals.tsx'
import      { withQuery }  from './with-query.tsx'
import      { withRouter } from './with-router.tsx'
import      { withTheme }  from './with-theme.tsx'

function apply<T, R>(target: T, fn: (t: T) => R) {
	return fn(target)
}

const providers = [withModals, withTheme, withQuery, withRouter]

export function withProviders(component: FC) {
	return providers.reduce(apply, component)
}
