export function mix(first: string, second: string, ratio: number, space = 'srgb') {
	return ratio >= 0
		? `color-mix(in ${space}, ${first}, ${second} ${ratio * 100}%)`
		: `color-mix(in ${space}, ${first} ${ratio * -100}%, ${second})`
}

export function alpha(color: string, alpha: number) {
	return mix(color, 'transparent', alpha)
}
