use std::{
  collections::HashMap,
  sync::Mutex,
};
use tauri::{
  Icon,Manager,
  menu::{CheckMenuItem, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
  tray::{ClickType, TrayIconBuilder},
};

use crate::{
  constants::enumerate,
  helpers::standards::time as std_time,
};

// struct
#[derive(Debug)]
struct ApplicationWindow {
  history: Mutex<HashMap<u128, enumerate::EnumStateAppWindow>>,
  visible: Mutex<bool>,
}

  fn toggle_system_tray_hide_menu_item(app_handle: &tauri::AppHandle) {
  let app_state_window = app_handle.state::<ApplicationWindow>();
  let app_state_window_visible = app_state_window.visible.lock().unwrap();
  if let Some(window) = app_handle.get_window("main") {
    let current_state_enum;
    if*(app_state_window_visible) {
      window.hide().unwrap();
      current_state_enum = enumerate::EnumStateAppWindow::CLOSED;
    } else {
      window.show().unwrap();
      current_state_enum = enumerate::EnumStateAppWindow::OPEN;
    };
    app_state_window.history.lock().unwrap().insert(std_time::epoch_ms(),current_state_enum);
  };
  // uncomment below to view tracked history on window toggle
  // println!("app_state_window.history.lock().unwrap(): {:?}", app_state_window.history.lock().unwrap());
}

// ensures the correct text and methods are called regardless of how the application was opened/closed
fn synchronize_state_system_tray_visibility<R: tauri::Runtime>(toggle_menu_item:tauri::menu::MenuItem<R>) {
  let app_handle = toggle_menu_item.app_handle();
  let app_state_window = app_handle.state::<ApplicationWindow>();
  if let Some(window) = app_handle.get_window("main") {
    match window.is_visible() {
      Err(why) => { println!("ERROR::window_visible: {:?}", why) },
      Ok(visible) => {
        *app_state_window.visible.lock().unwrap() = visible;
        if visible {toggle_menu_item.set_text("Hide DevQon").unwrap()}
        else {toggle_menu_item.set_text("Show DevQon").unwrap();}
      }
    }
  }
}

pub fn create_system_tray(app:&tauri::App) -> Result<tauri::AppHandle,tauri::Error> {
  let managed_history:HashMap<u128, enumerate::EnumStateAppWindow> = HashMap::from([(std_time::epoch_ms(),enumerate::EnumStateAppWindow::START)]);
  app.manage(ApplicationWindow { visible: true.into(), history:managed_history.into()}); // TODO: evaluate if this needs to be managed by `app`

  let mnu_itm_toggle_visibility = MenuItemBuilder::with_id(&enumerate::EnumAppElement::MenuItemVisible.get_id(), "Hide").build(app);
  let breakline = PredefinedMenuItem::separator(app);
  let show = MenuItemBuilder::with_id("quit", "Quit").build(app);
  
  let menu = MenuBuilder::with_id(app,&enumerate::EnumAppElement::MenuAttachedToTray.get_id())
              .items(&[&mnu_itm_toggle_visibility,&breakline,&show])
              .item(&CheckMenuItem::new(app, "no click", false, false, None),)
              .check("curvis", "current vision") // shorthand, have to use defaults
            .build()?;
  let _tray = TrayIconBuilder::with_id(&enumerate::EnumAppElement::TrayMenuAndIcon.get_id())
        .menu(&menu)
        .on_menu_event(move |app_handle, event| match event.id().as_ref() {
          "TOGGLE_VISIBILITY" => { toggle_system_tray_hide_menu_item(app_handle); }
          "quit" => { std::process::exit(0); }
          _ => (),
        })
        .on_tray_icon_event(move |tray, event| {
          if event.click_type == ClickType::Left {
            synchronize_state_system_tray_visibility(mnu_itm_toggle_visibility.clone()) }
          if event.click_type == ClickType::Right {
            let app = tray.app_handle();
            if let Some(window) = app.get_window("main") {
              let _ = window.show();
              let _ = window.set_focus();
              synchronize_state_system_tray_visibility(mnu_itm_toggle_visibility.clone())
            }
          }
        })
        .build(app)?;

  Ok(app.handle().clone())
}

// This is sparse for now, but exists as a separate function for future scalability.
    // Additional logic and/or conditions would be applied within this function.
    // > e.g.: if we chose to display an icon that changed based on the one currently displayed.
pub fn handle_system_tray_icon_update(tray_handle:tauri::AppHandle, new_status_enum:enumerate::EnumIconStatusType) {
  let tray_sys = tray_handle.tray_by_id(&enumerate::EnumAppElement::TrayMenuAndIcon.get_id()).unwrap();
  let _ = tray_sys.set_icon(None); // reset so next transform will take place
  update_system_tray_icon(&tray_handle, new_status_enum);
}

fn set_as_template(tray_handle: &tauri::AppHandle, is_template:bool) {
  // tauri needs a tick on macos for the icon to be registered before setting as template
  let tray_sys = tray_handle.tray_by_id(&enumerate::EnumAppElement::TrayMenuAndIcon.get_id()).unwrap();
  let _ = tray_sys.set_title(Some("".to_string())); // this also seems to force the icon refresh
  let _ = tray_sys.set_icon_as_template(is_template);
}
fn update_system_tray_icon(tray_handle: &tauri::AppHandle, status_enum:enumerate::EnumIconStatusType) {
  let tray_sys = tray_handle.tray_by_id(&enumerate::EnumAppElement::TrayMenuAndIcon.get_id()).unwrap();
  let icon_bytes:Vec<u8> = status_enum.to_bytes(); // get byte array from enum impl
  tray_sys.set_icon(Some(Icon::Raw(icon_bytes.clone()))).unwrap();

  #[cfg(target_os = "macos")] // only supported on macos - only run on macos
  set_as_template(tray_handle, Some(icon_bytes).is_some());
}

