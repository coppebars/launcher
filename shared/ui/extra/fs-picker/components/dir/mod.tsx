import { Box } from "@mantine/core"

interface Props {
	name: string
	newPath: (newPath: string) => void
	join: (...paths: string[]) => Promise<string>
	value: string
}

const Dir = (props: Props) => {
	return (
		<Box
			onClick={async () => {
				const path = await props.join(props.value, props.name)
				props.newPath(path)
			}}
		>
			Dir: {props.name}
		</Box>
	)
}

export default Dir
