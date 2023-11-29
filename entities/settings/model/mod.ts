import { dataDir }     from '@tauri-apps/api/path'
import { join }        from '@tauri-apps/api/path'
import { createEvent } from 'effector'
import { createStore } from 'effector'
import { persist }     from 'effector-storage/local'

export interface Settings {
	rootPath: string
}

export const changeRootDir = createEvent<string>('change_root_dir')

export const $settings = createStore<Settings>(
	{
		rootPath: await join(await dataDir(), '.coppebars'),
	},
	{ name: 'settings' },
)

$settings.on(changeRootDir, (state, rootPath) => ({ ...state, rootPath }))

persist({ store: $settings })
