/* eslint-disable react/no-unused-prop-types */

import { useCallback }    from 'react'

import { rem }            from '@mantine/core'
import { UnstyledButton } from '@mantine/core'
import { IconBolt }       from '@tabler/icons-react'
import { IconUser }       from '@tabler/icons-react'
import { IconSettings2 }  from '@tabler/icons-react'
import { useLocation }    from 'react-router-dom'
import { useNavigate }    from 'react-router-dom'

import { SidebarLayout }  from '@layout/sidebar'
import { WindowControls } from '@ui/extra'

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
			<Icon style={{ width: rem(20), height: rem(20), userSelect: 'none' }} stroke={1.6} />
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
		path: '/profiles',
		icon: IconUser,
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
				key={path}
				icon={icon}
				active={index ? pathname === '/' : pathname.startsWith(path)}
				onClick={() => navigate(path)}
			/>
		),
		[navigate, pathname],
	)

	return (
		<SidebarLayout>
			<WindowControls />
			{topRoutes.map(renderItem)}
			<div style={{ flexGrow: 1, userSelect: 'none' }} />
			{bottomRoutes.map(renderItem)}
		</SidebarLayout>
	)
}
