import { Box }      from "@mantine/core"
import { useState } from "react"
import File         from "../file/mod"
import { Entry }    from "../../types"

interface Props {
	name: string
	newPath: (newPath: string) => void
	join: (...paths: string[]) => Promise<string>
	value: string
	lookup: (path: string) => Promise<Entry[]>
	type: string
}

const Dir = (props: Props) => {
	const [dirs, setDirs] = useState<Entry[]>([])
	const [visible, setVisible] = useState<boolean>(false)

	const handleClick = async () => {
		if (visible) {
			const index = props.value.indexOf(props.name)
			if (index > 2) {
				props.newPath(props.value.substring(0, index))
			}
		} else {
			const path = await props.join(props.value, props.name)
			const dirsAs = await props.lookup(path)
			if (dirsAs) {
				setDirs(dirsAs)
				props.newPath(path)
			} else {
				const lastIndex = props.value.lastIndexOf('\\')
				if (lastIndex > 2) {
					const backPath = await props.join(props.value.replace(props.value.substring(lastIndex), ''), props.name)
					props.newPath(backPath)
					console.log(backPath)
					const newDirs = await props.lookup(backPath)
					setDirs(newDirs)
				}
			}
		}
		setVisible(!visible)
	}

	return (
		<>
			<Box onClick={handleClick}>Dir: {props.name}</Box>
			{visible ? (
				<Box style={{ marginLeft: '10px' }}>
					{dirs?.map((el: Entry) =>
						props.type === 'dirs' ? (
							el.isDir ? (
								<Dir
									key={el.name}
									name={el.name}
									newPath={props.newPath}
									join={props.join}
									value={props.value}
									type={props.type}
									lookup={props.lookup}
								/>
							) : null
						) : !el.isDir ? (
							<File key={el.name} name={el.name} newPath={props.newPath} join={props.join} value={props.value} />
						) : null)}
				</Box>
			) : null}
		</>
	)
}

export default Dir
