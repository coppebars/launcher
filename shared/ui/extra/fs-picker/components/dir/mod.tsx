import { Box }      from "@mantine/core"
import { useState } from "react"
import File         from "../file/mod"

interface Entry {
	isDir: boolean
	name: string
}

interface Props {
	name: string
	newPath: (newPath: string) => void
	join: (...paths: string[]) => Promise<string>
	value: string
	lookup: (path: string) => Promise<Entry[]>
	type: string
}

const Dir = (props: Props) => {
	const [dirs, setDirs] = useState<any>([])
	const [visible, setVisible] = useState<boolean>(false)
	return (
		<>
			<Box
				onClick={async () => {
					if (visible) {
						const index = props.value.indexOf(props.name)
						if (index > 2) {
							props.newPath(props.value.substring(0, index))
						}
					} else {
						// Надо исправить баг, когда есть две вложенных папки в одной, можно выбрать одну в другую
						const path = await props.join(props.value, props.name)
						const dirsAs = await props.lookup(path)
						setDirs(dirsAs)
						props.newPath(path)
					}
					console.log(props.value)
					setVisible(!visible)
				}}
			>
				Dir: {props.name}
			</Box>
			{visible ? (
				<Box style={{ marginLeft: '10px' }}>
					{dirs?.map((el: any) =>
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
