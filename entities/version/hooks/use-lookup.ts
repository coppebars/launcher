import type { Provider }                      from '@entity/version'
import type { Version }                       from '@entity/version'
import      { useQuery }                      from '@tanstack/react-query'

import      { lookupLocalVersions }           from '../lib/local.ts'
import      { fetchVersions as fetchMojangs } from '../lib/mojang.ts'

export function useLookup() {
	return useQuery({
		queryKey: ['lookup_versions'],
		queryFn: async () => {
			const mojang = await fetchMojangs()

			const local = await lookupLocalVersions()

			return {
				/// 1
				local,
				/// 2
				mojang,
			} satisfies Partial<Record<Provider, Version[]>>
		},
	})
}
