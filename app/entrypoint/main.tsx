import { createRoot }    from 'react-dom/client'

import { withProviders } from '@app/providers'
import { Root }          from '@app/root'

const RootWithProviders = withProviders(Root)

createRoot(document.getElementById('root')!).render(<RootWithProviders />)
