use crate::mechanisms::database::connect;

pub fn conf() {
  println!("Configure database");
  connect::connect_to_db();
}