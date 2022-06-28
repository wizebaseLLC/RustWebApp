use crate::models::cmdb_ci_hardware::dataloaders::IsOnlineLoader;
use crate::models::cmdb_ci_service::dataloaders::{
    IsAppLoader, IsDatabaseLoader, IsDbOnlineLoader, IsServerLoader, ServiceNowRelCiChildrenLoader,
};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Enum, FieldResult, SimpleObject};

use super::serde_types_app_details::Parent;
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum RelCiTypes {
    Parents,
    Children,
}
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct RootServiceNowRelCi {
    pub result: Vec<ServiceNowRelCi>,
}
#[derive(
    Default,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
    Debug,
)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct ServiceNowRelCi {
    #[serde(rename = "connection_strength")]
    pub connection_strength: ConnectionStrength,
    pub parent: Parent,
    #[serde(rename = "sys_mod_count")]
    pub sys_mod_count: SysModCount,
    #[serde(rename = "sys_updated_on")]
    pub sys_updated_on: SysUpdatedOn,
    #[serde(rename = "sys_tags")]
    pub sys_tags: SysTags,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(rename = "sys_id")]
    pub sys_id: SysId,
    #[serde(rename = "sys_updated_by")]
    pub sys_updated_by: SysUpdatedBy,
    pub port: Port,
    #[serde(rename = "sys_created_on")]
    pub sys_created_on: SysCreatedOn,
    #[serde(rename = "percent_outage")]
    pub percent_outage: PercentOutage,
    #[serde(rename = "sys_created_by")]
    pub sys_created_by: SysCreatedBy,
    pub child: Child,
}

#[ComplexObject]
impl ServiceNowRelCi {
    pub async fn children(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<ServiceNowRelCi>>> {
        let data: Option<Vec<ServiceNowRelCi>> = ctx
            .data_unchecked::<DataLoader<ServiceNowRelCiChildrenLoader>>()
            .load_one(self.child.value.clone())
            .await?;

        Ok(data)
    }

    pub async fn is_server(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let data: Option<bool> = ctx
            .data_unchecked::<DataLoader<IsServerLoader>>()
            .load_one(self.child.display_value.clone().to_lowercase())
            .await?;

        if let Some(data) = data {
            Ok(data)
        } else {
            Ok(false)
        }
    }

    pub async fn is_database(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let data: Option<bool> = ctx
            .data_unchecked::<DataLoader<IsDatabaseLoader>>()
            .load_one(self.child.display_value.clone().to_lowercase())
            .await?;

        if let Some(data) = data {
            Ok(data)
        } else {
            Ok(false)
        }
    }

    pub async fn is_app(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let data: Option<bool> = ctx
            .data_unchecked::<DataLoader<IsAppLoader>>()
            .load_one(self.child.display_value.clone().to_lowercase())
            .await?;

        if let Some(data) = data {
            Ok(data)
        } else {
            Ok(false)
        }
    }

    // Decpreceated: using dataloader now.
    // pub async fn is_server_online(&self) -> FieldResult<Option<bool>> {
    //     let result = run_ssh_ping(&self.child.display_value).await.ok();
    //     Ok(result)
    // }

    pub async fn is_server_online(&self, ctx: &Context<'_>) -> FieldResult<Option<bool>> {
        let data: Option<bool> = ctx
            .data_unchecked::<DataLoader<IsOnlineLoader>>()
            .load_one(self.child.display_value.clone())
            .await?;

        Ok(data)
    }

    pub async fn is_db_online(&self, ctx: &Context<'_>) -> FieldResult<Option<bool>> {
        let data: Option<bool> = ctx
            .data_unchecked::<DataLoader<IsDbOnlineLoader>>()
            .load_one(self.child.value.clone())
            .await?;

        Ok(data)
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStrength {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysModCount {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysUpdatedOn {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysTags {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysId {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysUpdatedBy {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysCreatedOn {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct PercentOutage {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct SysCreatedBy {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
}
