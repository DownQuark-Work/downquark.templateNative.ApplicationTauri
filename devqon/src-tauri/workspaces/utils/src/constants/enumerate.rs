#[derive(Debug)]
pub enum EnumAppElement {
  IconDq, IconBusy,
  MenuItemVisible, MenuAttachedToTray,
  TrayMenuAndIcon,
  WindowMain, WindowSplasScreen,
}
impl EnumAppElement {
  pub fn get_id(&self) -> String {
    match self {
      Self::MenuAttachedToTray => "DQ_DEVQON_MENU_ATTACHED_TO_TRAY".to_string(),
      Self::MenuItemVisible => "TOGGLE_VISIBILITY".to_string(),
      Self::TrayMenuAndIcon => "DQ_DEVQON_MENU_TRAY_MAIN".to_string(),
      Self::WindowMain => "main".to_string(),
      Self::WindowSplasScreen => "splashscreen".to_string(),
      _ => "".to_string()
    }
  }
  pub fn get_path(&self) -> String {
    match self {
      Self::IconBusy => "../../../../icons/status/busy.ico".to_string(),
      _ => "../../../../icons/_downquark/icon.ico".to_string()
    }
  }
}

// `include_bytes` must be string literal.
// cannot use paths from `EnumAppElement`
#[derive(Debug)]
pub enum EnumIconStatusType { _DQ, BUSY, }
impl EnumIconStatusType {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      Self::BUSY => include_bytes!("../../../../icons/status/busy.ico").to_vec(),
      _ => include_bytes!("../../../../icons/_downquark/icon.ico").to_vec(), // _DQ as default
    }
  }
}

#[derive(Debug)]
pub enum EnumStateAppWindow {
  CLOSED,EXIT,OPEN,START,UNOBTAINED,
}