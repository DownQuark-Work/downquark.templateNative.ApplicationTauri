// extend to internal ORM?

pub fn raw() {
  println!("arg1");
  println!("run qry() with above");
  qry();
}

pub fn select() {
  println!("SELECT arg1 FROM arg2");
  println!("run qry() with above");
  qry();
}

fn qry() {
  println!("Query database");
}