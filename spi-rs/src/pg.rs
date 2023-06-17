use std::future::IntoFuture;

use async_lazy::Lazy;
use ctor::ctor;
use tokio_postgres::{types::ToSql, Client, NoTls, Row, Statement};

use crate::rt::RT;

pub struct LazyStatement(pub async_lazy::Lazy<tokio_postgres::Statement>);

pub trait IntoStatement {
  fn into(&self) -> &Statement;
}

impl IntoStatement for LazyStatement {
  fn into(&self) -> &Statement {
    self.0.get().unwrap()
  }
}

#[macro_export]
macro_rules! sql {
    ($($var:ident : $sql:expr),+ ) => {
        $(
            pub static $var: $crate::pg::LazyStatement  =
            $crate::pg::LazyStatement(async_lazy::Lazy::const_new(|| Box::pin(async move { $crate::pg::PG.force().await.prepare($sql).await.unwrap() })));
        )+

            mod private {
                #[ctor::ctor]
                fn pg_statement_init() {
                    $crate::RT.block_on(async move {
                        $(super::$var.0.force().await;)+
                    });
                }
            }
    };
}

//   r = ONE0"SELECT name FROM img.sampler WHERE id=#{id}"
// else
//   r = await LI"SELECT id,name FROM img.sampler"

pub static PG: Lazy<Client> = Lazy::const_new(|| {
  let pg_uri = std::env::var("PG_URI").unwrap();
  Box::pin(async move {
    let (client, connection) = tokio_postgres::connect(&format!("postgres://{}", pg_uri), NoTls)
      .await
      .unwrap();
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        tracing::error!("postgres connection error: {e}");
      }
    });

    client
  })
});

#[ctor]
fn init() {
  RT.block_on(async move {
    PG.into_future().await;
  });
}

#[allow(non_snake_case)]
pub async fn Q<T>(
  statement: &T,
  params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<Row>, tokio_postgres::Error>
where
  T: ?Sized + IntoStatement,
{
  PG.get().unwrap().query(statement.into(), params).await
}

#[allow(non_snake_case)]
pub async fn Q1<T>(
  statement: &T,
  params: &[&(dyn ToSql + Sync)],
) -> Result<Row, tokio_postgres::Error>
where
  T: ?Sized + IntoStatement,
{
  PG.get().unwrap().query_one(statement.into(), params).await
}
