use async_graphql::SimpleObject;

use crate::models::cmdb_ci_database::serde_types::serde_cmdb_ci_database::Name;

use super::serde_types_rel_ci::SysId;

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
pub struct RootServiceNowApps {
    pub result: Vec<ServiceNowAppsResult>,
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
pub struct ServiceNowAppsResult {
    #[serde(rename = "sys_id")]
    pub sys_id: SysId,
    pub name: Name,
    #[serde(rename = "u_active")]
    pub u_active: UActive,
    #[serde(rename = "u_is_signedoff_cbt")]
    pub u_is_signedoff_cbt: UIsSignedoffCbt,
    #[serde(rename = "u_appowner_pri")]
    pub u_appowner_pri: UAppownerPri,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: UIsSignedoff,
    #[serde(rename = "u_is_gpdr_affirmed")]
    pub u_is_gpdr_affirmed: UIsGpdrAffirmed,
    #[serde(rename = "u_app_lifecycle")]
    pub u_app_lifecycle: UAppLifecycle,
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
pub struct UActive {
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
pub struct UIsSignedoffCbt {
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
pub struct UAppownerPri {
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
pub struct UIsSignedoff {
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
pub struct UIsGpdrAffirmed {
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
pub struct UAppLifecycle {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
}
