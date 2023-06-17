use lazy_static::lazy_static;
use tokio::runtime::Runtime;
lazy_static! {
  pub static ref RT: Runtime = Runtime::new().unwrap();
}
