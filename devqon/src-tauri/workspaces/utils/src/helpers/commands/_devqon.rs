use tauri::{State};
use std::collections::HashMap;
// use tauri::{Manager, State};

use crate::helpers::state::_devqon::{UserSession,History,UpdatedSetting,ActiveVision,};

#[derive(Debug,Default)]
pub struct Database {
    conn_str: String,
    qry_id: usize,
}
#[derive(serde::Serialize)]
pub struct RespConnectDB {
  conn_status: String,
  qry_resp: String,
}

#[derive(serde::Serialize)]
pub struct RespTwoWayComm {
  message: String,
  other_val: usize,
}

async fn some_other_function() -> Option<String> {
  Some("secondary fn called".into())
}

#[tauri::command]
pub async fn cmd_two_way_comm(
  window: tauri::Window,
  number: usize,
  // database: State<'_, Database>,
) -> Result<RespTwoWayComm, String> {

  println!("2 way comm with -> {}", window.label());
  let result: Option<String> = some_other_function().await;
  if let Some(message) = result {
    Ok(RespTwoWayComm {
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

// pub async fn cmd_connect_to_database(state: State<'_, Database>) -> Result<(), String> {
#[tauri::command]
pub fn cmd_connect_to_database(
  database: State<'_, Database>,
  conn_str:&str,
  qry_id:usize,
) -> Result<RespConnectDB, String> {
  
println!("conn_str: {}",conn_str);
println!("qry_id: {}",qry_id);

  Database {
    conn_str:conn_str.to_string(),
    qry_id
  };
  println!("with database: {:#?}",database);
  Ok(RespConnectDB {
    conn_status: "SUCCESS".to_string(),
    qry_resp: "QUERY RESULTS...".to_string()
  })
}

#[tauri::command]
pub fn cmd_determine_view_and_title(name: &str) -> String {
  display_correct_view();
  format!(".::. {}!", name)
}

// static state
#[derive(serde::Serialize)]
pub struct ResUserSession {
  created: String,
  session_id: String,
}
#[tauri::command] // comments below show more examples. keeping for reference
pub async fn cmd_state_trigger_us( // request "special" args when it is run
  // window: tauri::Window,
  // app_handle: tauri::AppHandle, // for completeness ... you can access app_handle from `tauri::command]`
  user_session: State<'_, UserSession> // user_session gets the specified `UserSession` object from `tauri::State`
) -> Result<ResUserSession, String> {
  // println!("Called from -> {}", window.label());
  // println!("wrapped state object: {:?}", user_session); // wrapped state object
  // println!("state object: {:?}", user_session.inner()); // state object
  // println!("state object: {:?}", user_session.session_id); // only session_id was marked as public
  Ok(ResUserSession {
    created: "PRIVATE FIELD".to_string(),
    session_id: user_session.session_id.to_string(),
  })
}

// mutable states
#[derive(serde::Serialize)]
pub struct ResVisionUpdate {
  successful: u8,
  updated_at: String,
}
#[tauri::command]
pub fn update_current_vision_setting(vision: State<ActiveVision>, updated:String)
  -> Result<ResVisionUpdate,String> {
  // initialize the vision, mutating the state with interior mutability
  *vision.current_vision.lock().unwrap() = Some(UpdatedSetting { updated_at: updated.clone() });  // `*` is deref
  println!("updated? {:?}", vision);
  Ok(ResVisionUpdate {
    successful: 1,
    updated_at: updated,
  })
}

#[derive(serde::Serialize)]
pub struct ResTrackNavigation {
  nav: HashMap<u64, String>
}
#[tauri::command]
pub fn track_navigation(at: u64, page: String, history: State<History>)
-> Result<ResTrackNavigation,String> {
  println!("at: {}, page: {}", at, page);
  history.navigation.lock().unwrap().insert(at, page);
  Ok(ResTrackNavigation {nav:history.navigation.lock().unwrap().clone()})
}