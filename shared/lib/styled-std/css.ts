export function opacity(color: string, val: number, space?: string) {
	return mix(color, 'transparent', val, space)
}

export function mix(first: string, second: string, val: number, space = 'srgb') {
	return `color-mix(in ${space}, ${first}, ${second} ${val * 100}%)`
}
