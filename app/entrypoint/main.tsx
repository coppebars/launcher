import { invoke }     from '@tauri-apps/api'
import { createRoot } from 'react-dom/client'

invoke('mojang_list_versions').then(console.log)

createRoot(document.getElementById('root')!).render('Hello')
