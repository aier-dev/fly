use anypack::{url_fn, VecAny};
use xxpg::Q;

Q!(
  sampler_id_name :
    SELECT id,name FROM img.sampler;
);

url_fn!(get() {
    sampler_id_name().await?
});
