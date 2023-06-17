use async_lazy::Lazy;
use tokio_postgres::{types::ToSql, Client, NoTls, Row, ToStatement};
//   r = ONE0"SELECT name FROM img.sampler WHERE id=#{id}"
// else
//   r = await LI"SELECT id,name FROM img.sampler"

pub const SQL_SAMPLER_ID_NAME: &str = "SELECT id::bigint::oid,name FROM img.sampler";

pub const SQL_SAMPLER_NAME_BY_ID: &str = "SELECT name FROM img.sampler WHERE id=$1::bigint";

static PG: Lazy<Client> = Lazy::const_new(|| {
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

    futures_util::future::try_join(
      client.prepare(SQL_SAMPLER_NAME_BY_ID),
      client.prepare(SQL_SAMPLER_ID_NAME),
    )
    .await
    .unwrap();
    client
  })
});

pub async fn init() {
  PG.force().await;
}

#[allow(non_snake_case)]
pub async fn Q<T>(
  statement: &T,
  params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<Row>, tokio_postgres::Error>
where
  T: ?Sized + ToStatement,
{
  PG.get().unwrap().query(statement, params).await
}

#[allow(non_snake_case)]
pub async fn Q1<T>(
  statement: &T,
  params: &[&(dyn ToSql + Sync)],
) -> Result<Row, tokio_postgres::Error>
where
  T: ?Sized + ToStatement,
{
  PG.get().unwrap().query_one(statement, params).await
}
