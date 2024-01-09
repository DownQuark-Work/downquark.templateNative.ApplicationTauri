use tauri::Manager;

use crate::helpers::system_tray;

pub fn handle_tauri_setup(app:&mut tauri::App) {
  let app_handle = app.handle();
  system_tray::handle_system_tray_icon_update(app_handle.clone(),"BUSY"); // set icon to loading symbol

  let splashscreen_window = app.get_window("splashscreen").unwrap();
  let main_window = app.get_window("main").unwrap();
  tauri::async_runtime::spawn(async move { // run initialization code on new task so app doesn't freeze
    
    println!("Initializing...");
    println!("Initialize app here instead of sleeping");
    std::thread::sleep(std::time::Duration::from_secs(3)); // replace sleep w/ actual code
    println!("Initialized");

    // After it's done, close the splashscreen and display the main window
    splashscreen_window.close().unwrap();
    main_window.show().unwrap();
    system_tray::handle_system_tray_icon_update(app_handle,"dq"); // set icon to downquark logo
  });
}

pub fn run_backend_in_background(event:tauri::RunEvent) {
  match event {
    tauri::RunEvent::ExitRequested { api, .. } => { api.prevent_exit(); }
    _ => {}
  }
}

pub fn run_frontend_in_background(event:tauri::GlobalWindowEvent) {
  match event.event() { 
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }
      _ => {}
    }
}

/* various build configurations
// - use below as templates more than anything else (for now at least)

fn build_without_persistence() {
  tauri::Builder::default()
    .system_tray(utils::helpers::build::create_system_tray())
      .on_system_tray_event(|app, event| { utils::helpers::build::handle_system_tray_events(app.clone(), event) })
    .setup(|app| { utils::helpers::build::handle_tauri_setup(app); Ok(()) })
    // .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
fn build_with_persistent_backend() {
  tauri::Builder::default()
    .system_tray(utils::helpers::build::create_system_tray())
      .on_system_tray_event(|app, event| { utils::helpers::build::handle_system_tray_events(app.clone(), event) })
    .setup(|app| { utils::helpers::build::handle_tauri_setup(app); Ok(()) })
    .on_window_event(|event| { utils::helpers::build::run_frontend_in_background(event) })
    // .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| { utils::helpers::build::run_backend_in_background(event) });
}
fn build_with_persistent_backend_and_frontend() {
  // `build_with_persistent_frontend` should be all that is needed.
  // Pretty sure the backend stays active when the frontend is persisted.
  // Keeping this for any unseen cases where both overrides would be required.
  tauri::Builder::default()
    .system_tray(utils::helpers::build::create_system_tray())
      .on_system_tray_event(|app, event| { utils::helpers::build::handle_system_tray_events(app.clone(), event) })
    .setup(|app| { utils::helpers::build::handle_tauri_setup(app); Ok(()) })
    .on_window_event(|event| { utils::helpers::build::run_frontend_in_background(event) })
    // .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| { utils::helpers::build::run_backend_in_background(event) });
}
fn build_with_persistent_frontend() {
  tauri::Builder::default()
    .system_tray(utils::helpers::build::create_system_tray())
      .on_system_tray_event(|app, event| { utils::helpers::build::handle_system_tray_events(app.clone(), event) })
    .setup(|app| { utils::helpers::build::handle_tauri_setup(app); Ok(()) })
    .on_window_event(|event| { utils::helpers::build::run_frontend_in_background(event) })
    .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
 */