use crate::attributes::Attributes;
use serde::{Deserialize, Serialize};

pub const UPDATE_TEMPLATE: &str = r#"
mutation updatePhishingTemplate($id: Int!, $attributes: PhishingTemplateAttributes!) {
    phishingTemplateUpdate(id: $id, attributes: $attributes) {
      node {
        id
        type
        category {
          id
        }
      }
      errors {
        field
        reason
        placeholders
      }
    }
  }
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTemplateVariables {
    pub id: i64,
    pub attributes: Attributes,
}

impl UpdateTemplateVariables {
    pub fn new(id: i64, attributes: Attributes) -> Self {
        let mut subject = "[EXT] ".to_string();
        subject.push_str(&attributes.subject.as_str().replace("[EXT] ", ""));
        Self {
            id,
            attributes: Attributes {
                subject,
                ..attributes
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub phishing_template_update: PhishingTemplateUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplateUpdate {
    pub node: Node,
    pub errors: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub category: Category,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
}
