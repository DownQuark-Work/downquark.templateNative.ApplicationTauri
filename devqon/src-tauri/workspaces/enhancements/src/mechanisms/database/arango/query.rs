// extend to internal ORM?

pub fn raw() {
  println!("[arango ]> arg1");
  println!("run qry() with above");
  qry();
}

pub fn select() {
  println!("[arango ]> SELECT arg1 FROM arg2");
  println!("run qry() with above");
  qry();
}

fn qry() {
  println!("[arango ]> Query database");
}