use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};

use crate::{
    graphql::context::GraphQLContext,
    models::cmdb_ci_service::{
        model::CmdbCiService,
        serde_types::{
            serde_types_all_apps::UIsSignedoff, serde_types_app_details::USignoffUser,
            serde_types_rel_ci::SysId,
        },
    },
};

use super::db_form_return_data::*;

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[graphql(complex)]
#[serde(rename_all = "snake_case")]
#[graphql(rename_fields = "snake_case")]
pub struct CmdbCiDatabaseJson {
    pub result: Vec<Result>,
}

#[ComplexObject]
impl CmdbCiDatabaseJson {
    pub async fn application_details(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<CmdbCiService>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let data = if self.result.is_empty() == false {
            if let Some(u_app_value) = &self.result[0].u_app.value {
                if u_app_value.len() > 1 {
                    Some(CmdbCiService::find(&mut conn, u_app_value.as_str()).await?)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

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
#[serde(rename_all = "snake_case")]
#[graphql(rename_fields = "snake_case")]
pub struct Result {
    #[serde(rename = "short_description")]
    pub short_description: ShortDescription,
    #[serde(rename = "u_dataserver_name")]
    pub u_dataserver_name: UDataserverName,
    #[serde(rename = "u_db_engine")]
    pub u_db_engine: UDbEngine,
    #[serde(rename = "u_signoff_date")]
    pub u_signoff_date: USignoffDate,
    #[serde(rename = "u_db_tier")]
    pub u_db_tier: UDbTier,
    #[serde(rename = "u_app")]
    pub u_app: UApps,
    #[serde(rename = "u_source_of_data")]
    pub u_source_of_data: USourceOfData,
    #[serde(rename = "u_pri_owner")]
    pub u_pri_owner: UPriOwner,
    #[serde(rename = "u_signoff_user")]
    pub u_signoff_user: USignoffUser,
    #[serde(rename = "u_is_auth_source")]
    pub u_is_auth_source: UIsAuthSource,
    #[serde(rename = "u_cbt_owner")]
    pub u_cbt_owner: UCbtOwner,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: UIsSignedoff,
    #[serde(rename = "u_sec_owner")]
    pub u_sec_owner: USecOwner,
    #[serde(rename = "u_is_vendor_component")]
    pub u_is_vendor_component: UIsVendorComponent,
    #[serde(rename = "u_information_classification")]
    pub u_information_classification: UInformationClassification,
    pub name: Name,
    #[serde(rename = "u_is_decom")]
    pub u_is_decom: UIsDecom,
    #[serde(rename = "u_cname")]
    pub u_cname: Defaults,
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
pub struct Defaults {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct ShortDescription {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UDataserverName {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UDbEngine {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct USignoffDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UDbTier {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UApps {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub link: Option<String>,
    pub value: Option<String>,
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
pub struct USourceOfData {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UIsAuthSource {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
    pub value: Option<String>,
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
pub struct UCbtOwner {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UIsVendorComponent {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
    pub value: Option<String>,
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
pub struct UInformationClassification {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct Name {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
pub struct UIsDecom {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
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
#[serde(rename_all = "snake_case")]
#[graphql(rename_fields = "snake_case")]
pub struct MyDatabaseListFromServiceNow {
    pub result: Vec<MyDbValues>,
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
#[serde(rename_all = "snake_case")]
#[graphql(rename_fields = "snake_case")]
pub struct MyDbValues {
    #[serde(rename = "sys_id")]
    pub sys_id: SysId,
    pub name: Name,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: UIsSignedoff,
}
