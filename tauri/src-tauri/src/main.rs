// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
  println!("Called from {}", window.label());
  println!("with database: {:#?}",database);
  let result: Option<String> = some_other_function().await;
  if let Some(message) = result {
    println!("DB message: {}",message);
    Ok(CustomResponse {
      message,
      other_val: number * 100 + 72,
    })
  } else {
    Err("No result".into())
  }
}

#[tauri::command]
fn update_page_title(name: &str) -> String {
  format!(".::. {}!", name)
}

fn main() {
  tauri::Builder::default()
    .manage(Database {})
    .invoke_handler(tauri::generate_handler![my_custom_command, update_page_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}