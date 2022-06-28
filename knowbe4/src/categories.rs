use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub phishing_template_categories: PhishingTemplateCategories,
    pub phishing_template_drafts: PhishingTemplateDrafts,
    pub user_categories: UserCategories,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplateCategories {
    pub nodes: Vec<Node>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub hidden: bool,
    pub template_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplateDrafts {
    pub count: i64,
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCategories {
    pub nodes: Vec<Node2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub hidden: bool,
}

pub const LIST_CATEGORIES: &str = r#"
query TemplateCategories(
    $type: PhishingTemplateCategoryTypes!
    $showHidden: Boolean
    $actorType: PhishingTemplateCategoryTypes!
  ) {
    phishingTemplateCategories(type: $type, showHidden: $showHidden) {
      nodes {
        id
        name
        createdAt
        hidden
        templateCount(showHidden: false)
      }
    }
    phishingTemplateDrafts(type: $type) {
      count
      totalCount
    }
    userCategories: phishingTemplateCategories(
      type: $actorType
      showHidden: $showHidden
    ) {
      nodes {
        id
        name
        createdAt
        hidden
      }
    }
  }
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoriesVariables {
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "showHidden")]
    pub show_hidden: bool,
    #[serde(rename = "actorType")]
    pub actor_type: String,
}

impl CategoriesVariables {
    pub fn new() -> Self {
        Self {
            actor_type: "USER".to_string(),
            show_hidden: true,
            typ: "USER".to_string(),
        }
    }
}
