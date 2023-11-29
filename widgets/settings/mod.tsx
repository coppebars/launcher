import { Box }              from '@mantine/core'
import { Container }        from '@mantine/core'
import { Divider }          from '@mantine/core'
import { Input }            from '@mantine/core'
import { Title }            from '@mantine/core'
import { IconFolderFilled } from '@tabler/icons-react'
import { useUnit }          from 'effector-react'

import { $settings }        from '@entity/settings'

export function SettingsWidget() {
	const settings = useUnit($settings)

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
					rightSection={<IconFolderFilled aria-label='Change directory' onClick={() => void (/* TODO */ 0)} />}
				/>
			</Input.Wrapper>
		</Container>
	)
}
