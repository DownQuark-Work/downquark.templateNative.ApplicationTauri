use std::{collections::HashMap, sync::Mutex};
// use tauri::State;

/* DYNAMIC / MUTABLE */
  // here we use Mutex to achieve interior mutability
  #[derive(serde::Serialize, Debug)]
  pub struct History {
  pub navigation: Mutex<HashMap<u64, String>>,
}

#[derive(serde::Serialize, Debug)]
pub struct UpdatedSetting {
  pub updated_at: String,
}
#[derive(serde::Serialize, Debug)]
pub struct ActiveVision {
  pub current_vision: Mutex<Option<UpdatedSetting>>,
}

/* STATIC */
#[derive(serde::Serialize, Debug)]
pub struct UserSession { // an example of something that could be static in the store
  id: String,
  created: String,
  pub session_id: String,
}

pub fn set_user_session() -> UserSession {
  UserSession {
    id: "user's id".to_string(),
    created: "1705122729167".to_string(),
    session_id: "mxpbmctZXhwZWN0ZWQtZm4tcG9pbnRl".to_string(),
  }
}
 // coming soon,...
// devqon info

/*
workspace STATE HANDLES INFO INSIDE OF USER APPLICATION
// static state
// - user id
// - user profile[username,realname,email,etc]
// - user prefs[first run config file]
// - devqon prefs
//   - theme[light,dark,brual,bento,etc]?
//   - memberTier[freemium,basic,advanced,etc]
// - session state
//   - current session id » UUID-UUID-UUID-UUID
//   - encrypt type » SHA256
//   - encript session » U1RPUlk6IFRo...
//   - session start time

// mutex state
// SESSION_ID » encript session from static
// active vision
// last online sync || -1
// short term persistence from enhancements
// - maybe one mechanism finishes an async request that multiple widgets could consume
// - keep it in _devqon state until fully dispensed?
// etc
*/

/*
Other functions will be checking / alerting if there is an update
- gathering feedback
- etc
*/