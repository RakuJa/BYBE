use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow)]
pub struct MyString {
    pub my_str: String,
}

pub async fn fetch_unique_values_of_field(
    conn: &Pool<Sqlite>,
    table: &str,
    field: &str,
) -> anyhow::Result<Vec<String>> {
    let query = format!(
        "SELECT CAST(t1.{field} AS TEXT) AS my_str FROM ((SELECT DISTINCT ({field}) FROM {table})) t1"
    );
    let x: Vec<MyString> = sqlx::query_as(query.as_str()).fetch_all(conn).await?;
    Ok(x.iter().map(|x| x.my_str.clone()).collect())
}
