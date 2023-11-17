const fonts = import.meta.glob('./assets/mabry/*.woff2', { eager: true })

export const mabry = Object.entries(fonts).map(([path, { default: url }]: any) =>
	() => {
		const filename = path.split('/').at(-1)!

		const [weight, italic] = /(\d\d\d)(i)?/.exec(filename)?.slice(1) ?? []

		return {
			'@font-face': {
				fontFamily: 'mabry',
				src: `url(${url}) format('woff2')`,
				fontWeight: weight,
				fontStyle: italic ? 'italic' : 'normal',
				fontDisplay: 'swap',
			},
		}
	})
