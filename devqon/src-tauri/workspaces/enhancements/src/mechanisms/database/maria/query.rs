// extend to internal ORM?

pub fn raw() {
  println!("[maria ]> arg1");
  println!("run qry() with above");
  qry();
}

pub fn select() {
  println!("[maria ]> SELECT arg1 FROM arg2");
  println!("run qry() with above");
  qry();
}

fn qry() {
  println!("[maria ]> Query database");
}