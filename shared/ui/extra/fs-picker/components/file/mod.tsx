import { Box } from "@mantine/core"

interface Props {
	name: string
	newPath: (newPath: string) => void
	join: (...paths: string[]) => Promise<string>
	value: string
}

const File = (props: Props) => {
	return (
		<Box
			onClick={async () => {
				if (!props.value.includes(props.name)) {
					const path = await props.join(props.value, props.name)
					props.newPath(path)
				}
			}}
		>
			File: {props.name}
		</Box>
	)
}

export default File
