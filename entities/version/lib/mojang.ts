import type { Version } from '@entity/version'

interface MojangVersionsResponse {
	versions: Array<{
		id: string
	}>
}

const url = new URL('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json')

export async function fetchVersions(): Promise<Version[]> {
	const { versions }: MojangVersionsResponse = await fetch(url).then((res) => res.json())

	return versions.map(({ id }) => ({
		mcv: id,
		vid: id,
		provider: 'mojang',
	}))
}
