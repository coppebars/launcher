import { Box }              from '@mantine/core'
import { Collapse }         from '@mantine/core'
import { Container }        from '@mantine/core'
import { Divider }          from '@mantine/core'
import { Input }            from '@mantine/core'
import { Title }            from '@mantine/core'
import { IconFolderFilled } from '@tabler/icons-react'
import { useUnit }          from 'effector-react'

import { FsPicker }         from '@ui/extra'
import { $settings }        from '@entity/settings'
import { useState }         from 'react'
import { useDisclosure }    from '@mantine/hooks'

export function SettingsWidget() {
	const settings = useUnit($settings)

	const [opened, { toggle }] = useDisclosure(false)

	const mockPaths: any = {
		'C:\\example': [
			{ name: 'hello.txt', isDir: false },
			{ name: 'project', isDir: true },
		],
		'C:\\example\\project': [
			{ name: 'files', isDir: true },
			{ name: 'package.json', isDir: false },
		],
		'C:\\example\\project\\files': [
			{ name: 'another', isDir: true },
			{ name: 'another2', isDir: true },
		],
		'C:\\example\\project\\files\\another': [],
		'C:\\example\\project\\files\\another2': [],
	}

	const [path, setPath] = useState('C:\\example')

	async function mockLookup(path: string) {
		return mockPaths[path]
	}

	return (
		<Container size='50rem'>
			<Box mt={32} />
			<Title>Settings</Title>
			<Divider my='sm' />
			<Input.Wrapper
				label='Root path'
				description='Path to the launcher and game files. Such as worlds, mods, libraries'
			>
				<Input
					py={4}
					value={settings.rootPath}
					disabled
					variant='filled'
					placeholder='Oops. Something going wrong...'
					rightSectionPointerEvents='all'
					rightSection={<IconFolderFilled cursor='pointer' aria-label='Change directory' onClick={toggle} />}
				/>
				<Collapse in={opened}>
					<FsPicker lookup={mockLookup} value={path} type='dirs' onChange={(newState) => setPath(newState)} />
				</Collapse>
			</Input.Wrapper>
		</Container>
	)
}
