import { Drawer as Base } from '@mantine/core'

import { body }           from './styles.css.ts'
import { content }        from './styles.css.ts'

export const Drawer = Base.extend({
	classNames: {
		body,
		content,
	},
})
