use anypack::{url_fn, VecAny};
use awp::Result;
use axum::response::Response;

url_fn!(get() {
  use crate::pg::{Q, SQL_SAMPLER_ID_NAME};

  let li = Q(SQL_SAMPLER_ID_NAME, &[]).await?;
  let mut vec = VecAny::new();
  for i in li {
    let id = i.get::<_, u32>(0);
    let name = i.get::<_, &str>(1).to_string();
    vec.push(id);
    vec.push(name);
  }
  vec
});
