import { Button as Base } from '@mantine/core'

import { label }          from './styles.css.ts'
import { loader }         from './styles.css.ts'
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
