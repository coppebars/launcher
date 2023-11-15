import { Global } from '@emotion/react'

import { mabry }  from './mabry.ts'

export function Fonts() {
	return <Global styles={[...mabry]} />
}
