/* eslint-disable react/no-unused-prop-types */

import { useCallback }    from 'react'

import { rem, Stack } from '@mantine/core'
import { UnstyledButton } from '@mantine/core'
import { IconBolt, IconDownload } from '@tabler/icons-react'
import { IconSettings2 }  from '@tabler/icons-react'
import { useLocation }    from 'react-router-dom'
import { useNavigate }    from 'react-router-dom'

import { SidebarLayout }  from '@layout/sidebar'

import * as styles        from './styles.css.ts'

interface SidebarItemProps {
	icon: typeof IconBolt
	active: boolean
	onClick: () => void
}

function SidebarItem(props: SidebarItemProps) {
	const { icon: Icon, active, onClick } = props

	return (
		<UnstyledButton onClick={onClick} className={styles.item} data-active={active || undefined}>
			<Icon style={{ width: rem(20), height: rem(20) }} stroke={1.6} />
			{/*<div className={styles.glow} data-active={active || undefined} />*/}
			{/*<div*/}
			{/*	className={styles.glow}*/}
			{/*	style={{ filter: 'blur(3px)', width: '1rem', height: '1rem' }}*/}
			{/*	data-active={active || undefined}*/}
			{/*/>*/}
		</UnstyledButton>
	)
}

interface RouteItem {
	path: string
	icon: typeof IconBolt
	index?: boolean
}

const topRoutes: RouteItem[] = [
	{
		path: '/',
		icon: IconBolt,
		index: true,
	},
	{
		path: '/versions',
		icon: IconDownload,
	},
]

const bottomRoutes: RouteItem[] = [
	{
		path: '/settings',
		icon: IconSettings2,
	},
]

export function SidebarWidget() {
	const { pathname } = useLocation()
	const navigate = useNavigate()

	const renderItem = useCallback(
		({ icon, path, index }: RouteItem) => (
			<SidebarItem
				icon={icon}
				active={index ? pathname === '/' : pathname.startsWith(path)}
				onClick={() => navigate(path)}
			/>
		),
		[navigate, pathname],
	)

	return (
		<SidebarLayout>
			{topRoutes.map(renderItem)}
			<div style={{ flexGrow: 1 }} />
			{bottomRoutes.map(renderItem)}
		</SidebarLayout>
	)
}
