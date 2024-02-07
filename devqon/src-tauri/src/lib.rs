// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use utils::{
  helpers::{
    build    as dq_build,
    commands as dq_commands,
    state    as dq_state,
    tray     as dq_tray,
  }
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
      #[cfg(debug_assertions)] // only include this code on debug builds
        { app.get_webview_window("main").unwrap().open_devtools(); }
      dq_state::initialize_app_states(app); // `manage` states
      let tray_app_handle = dq_tray::create_system_tray(app); // tray menu item
      dq_build::initialize_application(&tray_app_handle.unwrap());
      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .manage(dq_commands::_devqon::Database::default()) // TODO: update or remove - this is temporary for now
    .on_window_event(|window, event| { utils::helpers::build::run_frontend_in_background(&window, event.clone()) })
    .invoke_handler(tauri::generate_handler![
      dq_commands::_devqon::cmd_two_way_comm,
      dq_commands::_devqon::cmd_connect_to_database,
      dq_commands::_devqon::cmd_determine_view_and_title,
      dq_commands::_devqon::cmd_state_trigger_us,
      dq_commands::_devqon::update_current_vision_setting,
      dq_commands::_devqon::track_navigation,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
