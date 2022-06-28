use crate::db::TiberiusConnectionManager;
use bb8::Pool;
use query_tiberius::query_tiberius::*;
use std::collections::HashMap;

pub fn keys_to_sql_in_query(keys: &[String]) -> Vec<String> {
    keys.into_iter()
        .map(|key| format!("'{}'", key))
        .collect::<Vec<String>>()
}

pub async fn build_dataloader_result<T>(
    conn: Pool<TiberiusConnectionManager>,
    query: String,
) -> anyhow::Result<Vec<<T as QueryTiberius>::Output>>
where
    T: QueryTiberius + Send + Sync,
{
    let conn = conn.clone();
    let mut conn = conn.get().await?;
    let stream = conn.query(query, &[]).await?;
    T::build_result_many(stream).await
}

pub fn complete_dataloader_result<T, V>(
    stream: Vec<T>,
    func: impl Fn(T) -> (String, V),
) -> HashMap<String, V>
// where
//     T: QueryTiberius,
{
    stream.into_iter().map(func).collect::<HashMap<String, V>>()
}
