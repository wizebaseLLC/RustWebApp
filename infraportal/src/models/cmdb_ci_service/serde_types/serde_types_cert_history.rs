use async_graphql::{InputObject, SimpleObject};

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
pub struct ApplicationCertHistory {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub user: String,
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
}

#[derive(InputObject, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ApplicationCertHistoryInput {
    pub date: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub user: String,
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
}
