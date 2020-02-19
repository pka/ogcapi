use deadpool_postgres::Client;

pub async fn db_query(client: &Client) -> i32 {
    let stmt = client.prepare(&"SELECT 42").await.unwrap();

    let rows = client.query(&stmt, &[]).await.unwrap();
    let value: i32 = rows[0].get(0);
    value
}
