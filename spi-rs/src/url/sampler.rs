use apg::Q;

Q!(
  sampler_id_name :
    SELECT id,name FROM img.sampler;
);

pub async fn get() -> awp::any!() {
  Ok(sampler_id_name().await?)
}
