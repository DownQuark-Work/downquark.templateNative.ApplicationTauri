import { tauri } from '@tauri-apps/api'

import type {InitResultType} from './types/devqon.d.ts'

const { invoke } = tauri

invoke('update_page_title', { name: 'DevQon' })
  .then((response) => {
    (window as any).header.innerHTML = response
  })
invoke('my_custom_command', { number: 1342, })
  .then((res) => console.log(`Message: ${(res as InitResultType).message}, Other Val: ${(res as InitResultType).other_val}`))
  .catch((e) => console.error(e))