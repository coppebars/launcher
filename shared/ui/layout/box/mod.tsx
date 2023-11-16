import      styled           from '@emotion/styled'
import type { ColorProps }   from 'styled-system'
import type { DisplayProps } from 'styled-system'
import type { FlexboxProps } from 'styled-system'
import type { GridProps }    from 'styled-system'
import type { LayoutProps }  from 'styled-system'
import type { SpaceProps }   from 'styled-system'
import      { color }        from 'styled-system'
import      { display }      from 'styled-system'
import      { flexbox }      from 'styled-system'
import      { grid }         from 'styled-system'
import      { layout }       from 'styled-system'
import      { space }        from 'styled-system'

import      { make }         from 'styled-std'

export interface Props extends GridProps, FlexboxProps, LayoutProps, DisplayProps, ColorProps, SpaceProps {}

export const Box = styled.div<Props>(make(grid, flexbox, layout, display, color, space))
