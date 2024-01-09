// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

#[derive(Debug)]
struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  other_val: usize,
}

async fn some_other_function() -> Option<String> {
  Some("mocked db query response".into())
}

#[tauri::command]
async fn my_custom_command(
  window: tauri::Window,
  number: usize,
  database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
  println!("Called from -> {}", window.label());
  println!("with database: {:#?}",database);
  let result: Option<String> = some_other_function().await;
  if let Some(message) = result {
    println!("DB message: {}",message);
    Ok(CustomResponse {
      message,
      other_val: number,
    })
  } else {
    Err("No result".into())
  }
}

fn display_correct_view() {
  println!("Show correct view based on results from splash screen init");
}

#[tauri::command]
fn update_page_title(name: &str) -> String {
  display_correct_view();
  format!(".::. {}!", name)
}

fn main() {
  tauri::Builder::default()
    .system_tray(utils::helpers::system_tray::create_system_tray())
      .on_system_tray_event(|app, event| { utils::helpers::system_tray::handle_system_tray_events(app.clone(), event) })
    .setup(|app| { utils::helpers::build::handle_tauri_setup(app); Ok(()) })
    .on_window_event(|event| { utils::helpers::build::run_frontend_in_background(event) })
    .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}