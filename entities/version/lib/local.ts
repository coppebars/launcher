// FIXME: Temporary FSD Violation
import      { $settings }      from '@entity/settings'
import type { Version }        from '@entity/version'
import      { lookupVersions } from 'core'

export async function lookupLocalVersions(): Promise<Version[]> {
	const result = await lookupVersions({ path: $settings.getState().rootPath })

	return result.map(({ id }) => ({
		provider: 'local',
		vid: id,
		mcv: undefined,
	}))
}
