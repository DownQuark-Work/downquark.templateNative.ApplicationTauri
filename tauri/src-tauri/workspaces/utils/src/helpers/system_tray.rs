use tauri::{Icon,Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_system_tray() -> SystemTray {
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("quit".to_string(), "Quit DevQon");
  let hide = CustomMenuItem::new("hide".to_string(), "Close DevQon");
  let tray_menu = SystemTrayMenu::new()
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);
  let system_tray = SystemTray::new()
    .with_id("sys-tray-development-qonsole")
    .with_menu(tray_menu);
  system_tray
}

pub fn handle_system_tray_events(app:tauri::AppHandle, event:SystemTrayEvent) {
  match event {
      SystemTrayEvent::LeftClick { position: _, size: _, .. } => { update_system_tray_hide_menu_item(app.clone(), false); }
      SystemTrayEvent::RightClick { position: _, size: _, .. } => { /* println!("system tray received a right click");*/ }
      SystemTrayEvent::DoubleClick { position: _, size: _, .. } => { /* println!("system tray received a double click"); */ }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => { std::process::exit(0); }
          "hide" => { update_system_tray_hide_menu_item(app.clone(), true) }
          _ => {}
        }
      }
      _ => {}
    }
}

pub fn handle_system_tray_icon_update(app_handle:tauri::AppHandle, new_status:&str) {
  /*
    This is sparse for now, but exists as a separate function for future scalability.
    Additional logic and/or conditions would be applied within this function.
    > e.g.: if we chose to display an icon that changed based on the one currently displayed.
  */
  update_system_tray_icon(app_handle, new_status);
}

// private
fn update_system_tray_hide_menu_item(app_handle: tauri::AppHandle, toggle_menu_item:bool) {
  // correctly handles Show|Hide items in System Tray Menu regardless of how the windows were opened/closed
  let item_handle = app_handle.tray_handle().get_item("hide");
  let window = app_handle.get_window("main").unwrap();
  fn hide_window(win:tauri::Window,itm: tauri::SystemTrayMenuItemHandle) { win.hide().unwrap(); itm.set_title("Open DevQon").unwrap(); }
  fn show_window(win:tauri::Window,itm: tauri::SystemTrayMenuItemHandle) { win.show().unwrap(); itm.set_title("Close DevQon").unwrap(); }
  match window.is_visible() {
    Err(why) => { println!("ERROR: {:?}", why) },
    Ok(visible) => {
      if visible {
        if toggle_menu_item { hide_window(window,item_handle) } else { show_window(window,item_handle) }
      } else {
        if toggle_menu_item { show_window(window,item_handle) } else { hide_window(window,item_handle) }
      }
    }
  }
}
fn update_system_tray_icon(app_handle:tauri::AppHandle, new_status:&str) {
  
  #![allow(unused_assignments)]
  let mut new_icon_bytes:Vec<u8> = "".into();
  match new_status {
    "BUSY" => { new_icon_bytes = include_bytes!("../../../../icons/status/busy.ico").to_vec(); }
    _ => { new_icon_bytes = include_bytes!("../../../../icons/_downquark/icon.ico").to_vec(); }
  }
  app_handle.tray_handle().set_icon(Icon::Raw(new_icon_bytes)).unwrap();
}