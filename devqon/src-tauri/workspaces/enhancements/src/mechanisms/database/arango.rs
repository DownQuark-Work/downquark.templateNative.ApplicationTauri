// https://github.com/fMeow/arangors
// https://docs.rs/arangors/latest/arangors/
pub mod configure;
pub mod query;

mod connect;

// // fn on_configure() {
// //     connect::connect_to_db();
// //   }
// connect::connect_to_db(); // todo: wrap in above type f

mod arangodb {
  fn internal() {}
}