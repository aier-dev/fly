#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

mod pg;
mod url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  awp::init();
  crate::pg::init().await;

  let mut router = Router::new();
  macro_rules! get {
    (=> $func:ident) => {
      get!("", $func)
    };
    ($url:stmt => $func:ident) => {
      get!(
        const_str::replace!(
          const_str::replace!(
            const_str::unwrap!(const_str::strip_suffix!(stringify!($url), ";")),
            " ",
            ""
          ),
          "&",
          ":"
        ),
        $func
      )
    };
    ($url:expr, $func:ident) => {
      router = router.route(const_str::concat!('/', $url), get($crate::url::$func::get))
    };
  }

  // get!( => stat);
  get!(sampler/&id => sampler_id);
  get!(sampler => sampler);

  // router = router.route("/sampler", get(crate::url::sampler::get));

  router = router.layer(CorsLayer::permissive());

  let port = match std::env::var("PORT") {
    Ok(val) => val.parse::<u16>().unwrap_or(8080),
    _ => 8080,
  };

  awp::srv(router, port).await;
  Ok(())
}
