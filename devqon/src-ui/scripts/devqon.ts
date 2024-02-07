// https://beta.tauri.app/guides/upgrade-migrate/from-tauri-1/#javascript-api-changes
import { invoke } from "@tauri-apps/api/core";

import type {InitResultType} from './types/devqon.d.ts'

const invokeCommandsList = [
  {
    name:'cmd_determine_view_and_title',
    opts:{ name: 'DevQon' },
    cb:(resp:string) => { (window as any).header.innerHTML = resp },
  },
  {
    name:'cmd_two_way_comm',
    opts:{ number: 1342, },
    cb:(res:InitResultType) => console.log(`Message: ${res.message}, Other Val: ${res.other_val}`),
  },
  {
    name:'cmd_connect_to_database',
    opts:{ connStr:"//qonnect", qryId:1313, },
    cb:(res:Record<string,string>) => console.log(`database connection: ${res.conn_status} with id: ${res.qry_resp}`),
  }
]
invokeCommandsList.forEach(command => {
  invoke(command.name,command.opts)
    .then(command.cb as any).catch((e) => console.error(e))
})

