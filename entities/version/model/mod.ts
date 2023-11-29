import { z } from 'zod'

// eslint-disable-next-line prettier/prettier
export const Provider = z.union([
	z.literal('mojang'),
	z.literal('local'),
	z.literal('unknown'),
]).default('unknown')

export type Provider = z.infer<typeof Provider>

export const Version = z.object({
	vid: z.string(),
	mcv: z.string().optional(),
	provider: Provider,
})

export type Version = z.infer<typeof Version>
