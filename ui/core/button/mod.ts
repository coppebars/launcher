import { Button as Base } from '@mantine/core'

import { label }          from './styles.css.ts'

export const Button = Base.extend({
	classNames: {
		label,
	},
})
