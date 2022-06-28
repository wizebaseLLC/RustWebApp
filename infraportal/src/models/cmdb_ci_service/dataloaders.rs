use super::{
    model::{CmdbCiService, DistributionListDetails},
    serde_types::serde_types_rel_ci::RootServiceNowRelCi,
};
use crate::{
    db::TiberiusConnectionManager,
    helpers::requests::{
        keys_to_sql::{build_dataloader_result, complete_dataloader_result, keys_to_sql_in_query},
        service_now_url::service_now_url,
    },
    models::{
        cmdb_ci_service::serde_types::serde_types_rel_ci::ServiceNowRelCi, sys_user::model::SysUser,
    },
};
use async_graphql::{dataloader::Loader, FieldError};
use async_trait::async_trait;
use bb8::Pool;
use itertools::Itertools;
use query_tiberius::query_tiberius::QueryTiberius;
use query_tiberius_derive::Queryable;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Queryable)]
pub struct NameIdQuery {
    pub name: String,
}

pub struct IsServerLoader(Pool<TiberiusConnectionManager>);

impl IsServerLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for IsServerLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);
        let query = format!(
            "
            SELECT
            name
            FROM  cmdb_ci_hardware
            WHERE name IN ({});
            ",
            keys.join(",")
        );
        let stream = build_dataloader_result::<NameIdQuery>(self.0.clone(), query).await;

        match stream {
            Ok(stream) => {
                let stream =
                    complete_dataloader_result(stream, |ci| (ci.name.clone().to_lowercase(), true));

                Ok(stream)
            }
            Err(err) => {
                eprintln!("{}", err);
                Ok(HashMap::<String, Self::Value>::new())
            }
        }
    }
}

pub struct IsDatabaseLoader(Pool<TiberiusConnectionManager>);

impl IsDatabaseLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for IsDatabaseLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT
            name
            FROM  cmdb_ci_database
            WHERE name IN ({});
            ",
            keys.join(",")
        );
        let stream = build_dataloader_result::<NameIdQuery>(self.0.clone(), query).await;

        match stream {
            Ok(stream) => {
                let stream =
                    complete_dataloader_result(stream, |ci| (ci.name.clone().to_lowercase(), true));

                Ok(stream)
            }
            Err(err) => {
                eprintln!("{}", err);
                Ok(HashMap::<String, Self::Value>::new())
            }
        }
    }
}

pub struct IsAppLoader(Pool<TiberiusConnectionManager>);

impl IsAppLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for IsAppLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT
            name
            FROM  cmdb_ci_service
            WHERE name IN ({});
            ",
            keys.join(",")
        );
        let stream = build_dataloader_result::<NameIdQuery>(self.0.clone(), query).await;

        match stream {
            Ok(stream) => {
                let stream =
                    complete_dataloader_result(stream, |ci| (ci.name.clone().to_lowercase(), true));

                Ok(stream)
            }
            Err(err) => {
                eprintln!("{}", err);
                Ok(HashMap::<String, Self::Value>::new())
            }
        }
    }
}

pub struct ServiceNowRelCiChildrenLoader(Client);

impl ServiceNowRelCiChildrenLoader {
    pub fn new(pool: Client) -> Self {
        Self(pool)
    }
}

fn query_rel_ci_from_sn_url(keys: &[String]) -> String {
    format!(
        "{}/api/now/table/cmdb_rel_ci?sysparm_query=parentIN{}&sysparm_display_value=all",
        service_now_url(),
        keys.join(",")
    )
}

///DataLoader with an array as child.  
///DataLoader using a rest endpoint
#[async_trait]
impl Loader<String> for ServiceNowRelCiChildrenLoader {
    type Value = Vec<ServiceNowRelCi>;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let query = query_rel_ci_from_sn_url(keys);

        let stream = CmdbCiService::query_service_now_get_builder::<RootServiceNowRelCi>(
            self.0.clone(),
            query.as_str(),
        )
        .await?
        .result;

        let stream = stream
            .clone()
            .into_iter()
            .into_group_map_by(|data| data.parent.value.clone());

        Ok(stream)
    }
}

pub struct SysUserLoader(Pool<TiberiusConnectionManager>);

impl SysUserLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for SysUserLoader {
    type Value = SysUser;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT  
            name,
            sys_id,
            employee_number,
            user_name,
            email,
            dv_department,
            u_cbt_,
            dv_u_business_head_name,
            u_business_head,
            dv_location,
            title,
            u_display_name,
            phone,
            mobile_phone
            FROM sys_user
            WHERE sys_id IN ({});
            ",
            keys.join(",")
        );

        let stream = build_dataloader_result::<SysUser>(self.0.clone(), query).await?;
        let stream = complete_dataloader_result(stream, |ci| (ci.sys_id.clone(), ci.clone()));

        Ok(stream)
    }
}

pub struct IsDbOnlineLoader(
    Pool<TiberiusConnectionManager>,
    Pool<TiberiusConnectionManager>,
);

impl IsDbOnlineLoader {
    pub fn new(
        sn_mirror: Pool<TiberiusConnectionManager>,
        db_cmdb: Pool<TiberiusConnectionManager>,
    ) -> Self {
        Self(sn_mirror, db_cmdb)
    }
}

#[derive(Debug, Clone, Queryable)]
pub struct CmdbCiDatabaseSlim {
    pub sys_id: String,
    pub u_dataserver_name: Option<String>,
    pub u_db_name: Option<String>,
}

#[derive(Debug, Clone, Queryable)]
pub struct DbCmdb {
    pub db_instance: Option<String>,
    pub server_name: Option<String>,
    pub instance_name: Option<String>,
    pub database_name: Option<String>,
    pub env_type: Option<String>,
}

#[async_trait]
impl Loader<String> for IsDbOnlineLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let find_names_query = format!(
            "
            SELECT  
            sys_id,
            u_db_name,
            u_dataserver_name
            FROM cmdb_ci_database
            WHERE sys_id IN ({});
            ",
            keys.join(",")
        );

        let find_names_query =
            build_dataloader_result::<CmdbCiDatabaseSlim>(self.0.clone(), find_names_query).await?;

        let db_names = find_names_query
            .clone()
            .into_iter()
            .map(|db| format!("'{}'", db.u_db_name.unwrap()))
            .collect::<Vec<String>>();

        let db_names = if db_names.len() > 0 {
            db_names.join(",")
        } else {
            "''".to_string()
        };

        let server_names = find_names_query
            .clone()
            .into_iter()
            .map(|db| format!("'{}'", db.u_dataserver_name.unwrap()))
            .collect::<Vec<String>>();

        let server_names = if server_names.len() > 0 {
            server_names.join(",")
        } else {
            "''".to_string()
        };

        let query = format!(
            "
            DECLARE @a TABLE  
            (dbinstance nvarchar(255), 
            ServerName nvarchar(255), 
            InstanceName nvarchar(255), 
            DatabaseName nvarchar(255), 
            EnvType nvarchar(255)) 
            INSERT INTO @a EXEC [dbo].[spListSQLDatabases]
           
            SELECT 
            dbinstance as 'db_instance',
            ServerName as 'server_name',
            InstanceName as 'instance_name',
            DatabaseName as 'database_name',
            EnvType as 'env_type'
            FROM @a 
            WHERE DatabaseName IN ({})
            AND 
            ServerName IN ({});
            ",
            db_names, server_names,
        );

        let stream = build_dataloader_result::<DbCmdb>(self.1.clone(), query).await?;
        let stream = complete_dataloader_result(stream.clone(), |db| {
            let query = find_names_query.clone().into_iter().find(|query| {
                query.u_dataserver_name == db.server_name && query.u_db_name == db.database_name
            });

            if let Some(query) = query {
                (query.sys_id, true)
            } else {
                ("".to_string(), false)
            }
        });

        Ok(stream)
    }
}

pub struct DistributionListLoader(Pool<TiberiusConnectionManager>);

impl DistributionListLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for DistributionListLoader {
    type Value = DistributionListDetails;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT 
                sys_id,
                u_display_name,
                email,
                u_primaryowner,
                dv_u_primaryowner,
                u_secondaryowner,
                dv_u_secondaryowner
            FROM [sn_mirror].[dbo].[sys_user_group]
            WHERE active = 'true'
            AND sys_id IN ({});
            ",
            keys.join(",")
        );

        let stream =
            build_dataloader_result::<DistributionListDetails>(self.0.clone(), query).await?;
        let stream = complete_dataloader_result(stream, |ci| (ci.sys_id.clone(), ci.clone()));

        Ok(stream)
    }
}
