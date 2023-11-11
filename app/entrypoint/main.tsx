import { invoke }     from '@tauri-apps/api'
import { createRoot } from 'react-dom/client'

invoke('list').then(console.log)

createRoot(document.getElementById('root')!).render('Hello')
