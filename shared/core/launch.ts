import { invoke } from '@tauri-apps/api/primitives'

export interface LaunchOptions {
	logbackId: string
	versionId: string
	provider: string
	root: string
	vars: Record<string, string>
}

export async function launch(options: LaunchOptions) {
	const { provider } = options

	const prepareCommand = `${provider}_prepare`
	const launchCommand = `${provider}_launch`

	{
		const { versionId: id, root: path } = options

		await invoke(prepareCommand, { id, path })
	}

	{
		const { versionId: id, logbackId: uid, root: path, vars } = options

		await invoke(launchCommand, { uid, id, path, vars })
	}
}
