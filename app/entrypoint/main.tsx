import { withProviders } from '@app/providers'
import { Root }          from '@app/root'
import { createRoot }    from 'react-dom/client'

const RootWithProviders = withProviders(Root)

createRoot(document.getElementById('root')!).render(<RootWithProviders />)
