use async_lazy::Lazy;
use tokio_postgres::{types::ToSql, Client, NoTls, Row, ToStatement};
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
