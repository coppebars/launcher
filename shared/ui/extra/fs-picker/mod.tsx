import { Flex }      from '@mantine/core'
// Функция для объединения пути, она асинхронная
import { join }      from '@tauri-apps/api/path'
import { useEffect } from 'react'
import { useState }  from 'react'
import Dir           from './components/dir/mod'
import { styled }    from './styles.css'
import File          from './components/file/mod'
import { Entry }     from './types'

interface Props {
	// Функция которая вернёт список файлов / папок для текущей директории
	lookup: (path: string) => Promise<Entry[]>
	// Текущее выбранный путь
	value: string
	onChange: (newPath: string) => void
	// Что выбирать: папки / файлы
	type: 'files' | 'dirs'
}

export function FsPicker(props: Props) {
	const [dirs, setDirs] = useState<Entry[]>([])

	const getDirs = async () => {
		const dirsAs = await props.lookup(props.value)
		setDirs(dirsAs)
	}

	useEffect(() => {
		getDirs()
	}, [dirs])

	return (
		<Flex direction='column' className={styled}>
			{dirs?.map((el: Entry) =>
				props.type === 'dirs' ? (
					el.isDir ? (
						<Dir
							key={el.name}
							name={el.name}
							newPath={props.onChange}
							join={join}
							value={props.value}
							type={props.type}
							lookup={props.lookup}
						/>
					) : null
				) : !el.isDir ? (
					<File key={el.name} name={el.name} newPath={props.onChange} join={join} value={props.value} />
				) : null)}
		</Flex>
	)
}
