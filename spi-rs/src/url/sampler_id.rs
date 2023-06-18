use anypack::url_fn;
use axum::extract::Path;
use xxpg::Q1;

Q1!(sampler_name_by_id : SELECT name FROM img.sampler WHERE id=$1);

url_fn!(get(Path(id): Path<u32>) {
    sampler_name_by_id(&id)
});
