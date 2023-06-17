use std::{pin::Pin, sync::Arc};

use anypack::{url_fn, VecAny};
use async_once_cell::Lazy;
use axum::response::Response;
use tokio_postgres::Statement;

use crate::pg::{PG, Q};

static SQL_SAMPLER_ID_NAME: Pin<Arc<Lazy<Statement, dyn Fn>>> = Arc::pin(Lazy::new(async move {
  PG.get()
    .unwrap()
    .prepare("SELECT id::bigint::oid,name FROM img.sampler")
    .await
    .unwrap()
}));

url_fn!(get() {

    let li = Q(&*SQL_SAMPLER_ID_NAME.as_ref().await, &[]).await?;
    let mut vec = VecAny::new();
    for i in li {
        let id = i.get::<_, u32>(0);
        let name = i.get::<_, &str>(1).to_string();
        vec.push(id);
        vec.push(name);
    }
    vec
});
