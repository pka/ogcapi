type Connection = mobc::Connection<mobc_postgres::PgConnectionManager<tokio_postgres::NoTls>>;

pub async fn db_query(client: &Connection) -> i32 {
    let stmt = client.prepare(&"SELECT 42").await.unwrap();

    let rows = client.query(&stmt, &[]).await.unwrap();
    let value: i32 = rows[0].get(0);
    value
}
