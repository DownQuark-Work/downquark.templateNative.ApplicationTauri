use crate::mechanisms::database::arango::connect;

pub fn conf() {
  println!("[arango ]> Configure arango database");
  connect::connect_to_db();
}