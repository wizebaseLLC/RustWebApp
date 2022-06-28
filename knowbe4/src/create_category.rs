use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategoriesVariables {
    pub name: String,
}

impl CreateCategoriesVariables {
    pub fn new() -> Self {
        Self {
            name: "Automated From RustLang".to_string(),
        }
    }
}

pub const CREATE_CATEGORY: &str = r#"
mutation create($name: String!) {
    phishingTemplateCategoryCreate(attributes: {name: $name}) {
      node {
        id
        name
        createdAt
        hidden
        type
        templateCount(showHidden: false)
      }
      errors {
        field
        placeholders
        reason
      }
    }
  }
  "#;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoriesRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub phishing_template_category_create: PhishingTemplateCategoryCreate,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplateCategoryCreate {
    pub node: Node,
    pub errors: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub hidden: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub template_count: i64,
}
