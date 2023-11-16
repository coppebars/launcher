import { Column } from '@ui/layout'
import { Row }    from '@ui/layout'

export function Root() {
	return (
		<Column p={4} gap={5}>
			<Row>Row1</Row>
			<Row>Row 1</Row>
		</Column>
	)
}
