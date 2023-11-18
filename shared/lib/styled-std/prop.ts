import type { Fn }    from './types.ts'
import type { Paths } from './types.ts'
import type { Props } from './types.ts'

export function std<T extends object>() {
	return {
		prop: (resolver: Resolver<T>) => prop(resolver),
		match: <K extends string>(matcher: (props: Props<T>) => Record<K, boolean>) => match(matcher),
		variants: <K extends string>(match: (props: Props<T>) => Record<K, boolean>, v: Record<K, Fn>) =>
			variants(match, v),
		when: (matcher: Matcher<T>, fn: Fn<T>, or?: Fn<T>) => when(matcher, fn, or),
		not: (matcher: Matcher<T>, fn: Fn<T>) => not(matcher, fn),
	}
}

type Matcher<T> = Paths<Props<T>> | ((props: Props<T>) => boolean)
type Resolver<T> = Paths<Props<T>> | ((props: Props<T>) => unknown)

export function prop<T extends object>(resolver: Resolver<T>) {
	if (typeof resolver === 'function') {
		return resolver
	}

	return (props: Props<T>) => resolver.split('.').reduce((a, c) => a?.[c], props as any)
}

export function match<T extends object, K extends string>(matcher: (props: Props<T>) => Record<K, boolean>) {
	return matcher
}

export function variants<T extends object, K extends string>(
	match: (props: Props<T>) => Record<K, boolean>,
	variants: Record<K, Fn>,
) {
	// eslint-disable-next-line consistent-return
	return (props: Props<T>) => {
		const result = Object.entries(match(props))
			.filter(([, value]) => value)
			.at(0)

		if (result) {
			const [key] = result

			return variants[key as keyof typeof variants](props)
		}
	}
}

export function when<T extends object>(matcher: Matcher<T>, fn: Fn<T>, or?: Fn<T>) {
	if (typeof matcher === 'function') {
		return (props: Props<T>) => (matcher(props) ? fn(props) : or?.(props))
	}

	return (props: Props<T>) => (matcher.split('.').reduce((a, c) => a?.[c], props as any) ? fn(props) : or?.(props))
}

export function not<T extends object>(matcher: Matcher<T>, fn: Fn<T>) {
	return when(matcher, undefined!, fn)
}
