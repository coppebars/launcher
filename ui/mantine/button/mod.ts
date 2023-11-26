import { Button as Base } from '@mantine/core'

import { label, loader } from './styles.css.ts'
import { section }        from './styles.css.ts'
import { root }           from './styles.css.ts'

export const Button = Base.extend({
	classNames: {
		root,
		loader,
		section,
		label,
	},
})
