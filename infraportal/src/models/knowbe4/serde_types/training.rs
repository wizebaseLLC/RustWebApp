use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Knowbe4TrainingApiItem {
    #[serde(rename = "enrollment_id")]
    pub enrollment_id: i64,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "module_name")]
    pub module_name: String,
    pub user: User,
    #[serde(rename = "campaign_name")]
    pub campaign_name: String,
    #[serde(rename = "enrollment_date")]
    pub enrollment_date: String,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "completion_date")]
    pub completion_date: Option<String>,
    pub status: String,
    #[serde(rename = "time_spent")]
    pub time_spent: i64,
    #[serde(rename = "policy_acknowledged")]
    pub policy_acknowledged: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub email: String,
}
