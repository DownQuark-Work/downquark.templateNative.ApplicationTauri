use tauri::{Manager};

use crate::constants::enumerate;
use crate::helpers::tray as dq_tray;

pub fn initialize_application(tray_handle: &tauri::AppHandle){
  let main_window = tray_handle.get_window(&enumerate::EnumAppElement::WindowMain.get_id()).unwrap();
  let splashscreen_window = tray_handle.get_window(&enumerate::EnumAppElement::WindowSplasScreen.get_id()).unwrap();
  let tray_sys = tray_handle.tray_by_id(&enumerate::EnumAppElement::TrayMenuAndIcon.get_id()).unwrap();
  
  dq_tray::handle_system_tray_icon_update(tray_handle.clone(),enumerate::EnumIconStatusType::BUSY); // set icon to downquark logo
  #[cfg(target_os = "macos")] // only supported on macos - only run on macos
  let _ = tray_sys.set_icon_as_template(true);

  tauri::async_runtime::spawn(async move { // run initialization code on new task so app doesn't freeze
    
    println!("Initializing.::.");
    println!("Initialize app here instead of sleeping");
    std::thread::sleep(std::time::Duration::from_secs(2)); // replace sleep w/ actual code
    println!("Initialized");

    // After it's done, close the splashscreen and display the main window
    splashscreen_window.close().unwrap();
    main_window.show().unwrap();
    dq_tray::handle_system_tray_icon_update(tray_sys.app_handle().clone(),enumerate::EnumIconStatusType::_DQ); // set icon to downquark logo
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