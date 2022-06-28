use async_graphql::SimpleObject;
use query_tiberius_derive::Queryable;

#[derive(Queryable)]
pub struct LoginHistoryBlankSlate {}

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
pub struct LoginHistory {
    pub preview: bool,
    #[serde(rename = "init_offset")]
    pub init_offset: i64,
    pub messages: Option<Vec<Message>>,
    pub fields: Option<Vec<String>>,
    pub rows: Option<Vec<Vec<String>>>,
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
pub struct Message {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}
