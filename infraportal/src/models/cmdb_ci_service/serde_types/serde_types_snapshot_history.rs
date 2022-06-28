use async_graphql::SimpleObject;

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
pub struct ApplicationSnapShotHistory {
    pub relationships: Relationships,
    pub svc_now_fields: SvcNowFields,
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
pub struct Relationships {
    pub prod: Vec<String>,
    pub dev: Vec<String>,
    pub stage: Vec<String>,
    pub qa: Option<Vec<String>>,
    pub child_apps: Vec<::serde_json::Value>,
    pub parent_app: Vec<::serde_json::Value>,
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
pub struct SvcNowFields {
    pub svc_now_field1: Vec<SvcNowField>,
    pub svc_now_field2: Vec<SvcNowField>,
    pub svc_now_field3: Vec<SvcNowField>,
    pub svc_now_field4: Vec<SvcNowField>,
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
pub struct SvcNowField {
    pub value: ::serde_json::Value,
    #[serde(rename = "sys_id")]
    pub sys_id: Option<String>,
    pub svc_now_field_name: Option<String>,
}
