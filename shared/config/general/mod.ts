import { dataDir }        from '@tauri-apps/api/path'
import { join }           from '@tauri-apps/api/path'

import { createDatabase } from 'indexed-db'

export interface Config {
	rootDir: string
}

export const cfg = await createDatabase<Config>({
	name: 'cfg',
	sub: 'general',
	defaults: {
		rootDir: await join(await dataDir(), '.coppertiles'),
	},
})
