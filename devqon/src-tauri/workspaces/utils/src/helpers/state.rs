pub mod _devqon;

mod state {
  use tauri::Manager;
  use crate::helpers::state;

  pub fn add_init_states(app:&mut tauri::App){
    println!("adding initial states");
    println!(" - static");
    app.manage(state::_devqon::set_user_session()); // set initial static state - make sure values are populated
    println!(" - dynamic");
    app.manage(state::_devqon::ActiveVision{ current_vision: Default::default() }); // set initial mutable state with default values - we can mutate
    app.manage(state::_devqon::History{ navigation: Default::default() }); // set initial mutable state with default values - we can mutate
  }
}

pub fn initialize_app_states(app:&mut tauri::App) {
  // TODO: switch by page here -- calll from mod when ready
  state::add_init_states(app);
}