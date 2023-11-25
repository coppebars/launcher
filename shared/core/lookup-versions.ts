import { invoke } from '@tauri-apps/api/primitives'

export interface LookupVersionsArgs {
	path: string
}

export interface Version {
	icon?: string
	id: string
}

export function lookupVersions(args: LookupVersionsArgs): Promise<Version[]> {
	return invoke('lookup_versions', args as {})
}
