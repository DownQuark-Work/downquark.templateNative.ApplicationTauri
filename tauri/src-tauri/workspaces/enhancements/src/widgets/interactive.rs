pub mod drag_drop;

mod interact {
  fn internal() {}
  fn set_element() {
    println!("set interactive element to arg1");
  }
  fn release_element() {
    println!("release previously set element -> set interactive element to null");
  }
}