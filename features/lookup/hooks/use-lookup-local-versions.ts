import { useQuery }       from '@tanstack/react-query'
import { useUnit }        from 'effector-react/effector-react.umd'

import { $settings }      from '@entity/settings'
import { lookupVersions } from 'core'

export function useLookupLocalVersions() {
	const { rootPath: path } = useUnit($settings)

	return useQuery({
		queryKey: ['local_versions'],
		queryFn: () => lookupVersions({ path }),
		retry: 0,
	})
}
