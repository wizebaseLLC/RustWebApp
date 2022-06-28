use super::dataloaders::{CmdbCiChangesLoader, IsOnlineLoader};
use crate::models::cmdb_ci_service::model::CmdbCiChanges;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, FieldResult, SimpleObject};
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
#[graphql(complex)]
pub struct CmdbCiServer {
    pub sys_id: String,
    pub name: Option<String>,
    pub u_tech_owner: Option<String>,
    pub pri_owner_email: Option<String>,
    pub dv_u_tech_owner: Option<String>,
    pub u_active: Option<String>,
    pub short_description: Option<String>,
    pub dv_u_cbt_owner: Option<String>,
    pub u_cbt_owner: Option<String>,
}

#[ComplexObject]
impl CmdbCiServer {
    // TODO! add dataloader on this function
    pub async fn is_online(&self, ctx: &Context<'_>) -> FieldResult<Option<bool>> {
        if let Some(name) = &self.name {
            // let result = run_ssh_ping(name).await.ok();
            // Ok(result)
            // Removed as the dataloader causees issues if cancelled early
            let data: Option<bool> = ctx
                .data_unchecked::<DataLoader<IsOnlineLoader>>()
                .load_one(name.to_string())
                .await?;

            Ok(data)
        } else {
            Ok(Some(false))
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable, Clone)]
#[graphql(complex)]
pub struct CmdbCiServerParentApps {
    pub rec_dv_parent: Option<String>,
    pub rec_type: Option<String>,
    pub rec_parent: Option<String>,
}

#[ComplexObject]
impl CmdbCiServerParentApps {
    pub async fn changes(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<CmdbCiChanges>>> {
        if let Some(parent) = &self.rec_parent {
            let data: Option<Vec<CmdbCiChanges>> = ctx
                .data_unchecked::<DataLoader<CmdbCiChangesLoader>>()
                .load_one(parent.to_owned())
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct CmdbCiHostLookup {
    pub name: Option<String>,
    pub application: Option<String>,
    pub parent: Option<String>,
    pub service: Option<String>,
    pub active: Option<String>,
    pub tier: Option<String>,
    pub parent_tier: Option<String>,
    pub owner: Option<String>,
    pub primary_email: Option<String>,
    pub secondary: Option<String>,
    pub secondary_email: Option<String>,
    pub asd_owner: Option<String>,
    pub asd_secondary_owner: Option<String>,
}
