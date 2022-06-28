use serde::{Deserialize, Serialize};

pub const MOVE_CATEGORY: &str = "https://training.knowbe4.com/spa/phishing/email_templates_actions";

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveCategoryBody {
    pub mass_action: String,
    pub cat_id: i64,
    pub ids: Vec<i64>,
    #[serde(rename = "type")]
    pub typ: String,
}

impl MoveCategoryBody {
    pub fn new(ids: Vec<i64>) -> Self {
        Self {
            typ: "user".to_string(),
            mass_action: "move".to_string(),
            cat_id: 394812,
            ids,
        }
    }
}
