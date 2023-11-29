import { invoke } from '@tauri-apps/api/primitives'

export interface LookupVersionsArgs {
	path: string
}

export interface Version {
	icon?: string
	id: string
}

export function lookupVersions(args: LookupVersionsArgs): Promise<Version[]> {
	return invoke<Version[]>('lookup_versions', args as {}).catch((err: string) => {
		throw new Error(err)
	})
}
