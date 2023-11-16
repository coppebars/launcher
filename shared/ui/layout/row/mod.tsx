import      styled    from '@emotion/styled'

import type { Fn }    from 'styled-std'

import type { Props } from '../box.ts'
import      { Box }   from '../box.ts'

const base: Fn = () => ({
	display: 'flex',
	flexDirection: 'row',
	width: '100%',
})

export const Row = styled(Box)<Props>(base)
