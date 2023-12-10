import { invoke } from '@tauri-apps/api/primitives'

export interface LaunchOptions {
	logbackId: string
	versionId: string
	provider: string
	root: string
	vars: Record<string, string>
}

export async function launch(options: LaunchOptions) {
	// // eslint-disable-next-line @typescript-eslint/no-unused-vars
	// const { provider } = options

	// const prepareCommand = `${provider}_prepare`

	// {
	// 	const { versionId: id, root: path } = options
	//
	// 	await invoke(prepareCommand, { version: { mcv: id }, path })
	// }

	{
		const { versionId: id, root, vars } = options

		await invoke('launch', { id, root, vars })
	}
}
