use axum::extract::Path;
use xxpg::Q1;

Q1!(sampler_name_by_id : SELECT name FROM img.sampler WHERE id=$1);

pub async fn get(Path(id): Path<u32>) -> awp::any!() {
  Ok(sampler_name_by_id(&id).await?)
}
