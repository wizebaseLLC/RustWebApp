use std::collections::HashMap;

use crate::{
    db::TiberiusConnectionManager,
    helpers::{
        ping::ping::run_ssh_ping_many,
        requests::keys_to_sql::{
            build_dataloader_result, complete_dataloader_result, keys_to_sql_in_query,
        },
    },
    models::cmdb_ci_service::model::CmdbCiChanges,
};
use async_graphql::{dataloader::Loader, FieldError};
use async_trait::async_trait;
use bb8::Pool;
use itertools::Itertools;

pub struct CmdbCiChangesLoader(Pool<TiberiusConnectionManager>);

impl CmdbCiChangesLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for CmdbCiChangesLoader {
    type Value = Vec<CmdbCiChanges>;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT 
            cmdb_ci,
            dv_cmdb_ci,
            number,
            short_description,
            dv_opened_by,
            dv_assignment_group,
            state,
            start_date,
            end_date,
            opened_at,
            sys_id
        FROM [sn_mirror].[dbo].[change_request]
        WHERE cmdb_ci IN ({}) and opened_at >= (GetDate() - 90)
        ORDER BY state ASC;
            ",
            keys.join(",")
        );

        let stream = build_dataloader_result::<CmdbCiChanges>(self.0.clone(), query).await?;

        let stream = stream
            .clone()
            .into_iter()
            .into_group_map_by(|data| data.cmdb_ci.clone().unwrap());

        Ok(stream)
    }
}

pub struct IsOnlineLoader;

impl IsOnlineLoader {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Loader<String> for IsOnlineLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let stream = run_ssh_ping_many(keys).await?;

        let stream =
            complete_dataloader_result(stream, |result| (result.name.to_string(), result.result));
        Ok(stream)
    }
}
