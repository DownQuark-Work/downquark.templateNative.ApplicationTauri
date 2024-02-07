use crate::mechanisms::database::maria::connect;

pub fn conf() {
  println!("[maria ]> Configure database");
  connect::connect_to_db();
}