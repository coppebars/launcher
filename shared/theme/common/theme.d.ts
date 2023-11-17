import                     '@emotion/react'

import type { Schema } from '@theme/common'

declare module '@emotion/react' {
	export interface Theme extends Schema {}
}
