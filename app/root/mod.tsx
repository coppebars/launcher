import { AppRoutes }     from '@app/routes'
import { SidedLayout }   from '@layout/sided'
import { DragRegion }    from '@ui/extra'
import { SidebarWidget } from '@widget/sidebar'

export function Root() {
	return (
		<SidedLayout
			sidebar={<SidebarWidget />}
			view={
				<>
					<DragRegion />
					<AppRoutes />
				</>
			}
		/>
	)
}
