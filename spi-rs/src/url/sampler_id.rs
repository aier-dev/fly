use anypack::url_fn;
use axum::{extract::Path, response::Response};

use crate::{pg::Q1, sql};

sql!(SQL_SAMPLER_NAME_BY_ID : "SELECT name FROM img.sampler WHERE id=$1::bigint");

url_fn!(get(Path(id): Path<u64>) {
    Q1(&SQL_SAMPLER_NAME_BY_ID, &[&(id as i64)]).await?.get::<_, &str>(0).to_owned()
});
