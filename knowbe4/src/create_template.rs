use crate::attributes::Attributes;
use serde::{Deserialize, Serialize};

pub const CREATE_TEMPLATE: &str = r#"
mutation createPhishingTemplate($type: PhishingTemplateTypes!, $attributes: PhishingTemplateAttributes!) {
    phishingTemplateCreate(type: $type, attributes: $attributes) {
      node {
        id
        type
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
pub struct CreateTemplateVariables {
    #[serde(rename = "type")]
    pub typ: String,
    pub attributes: Attributes,
}

impl CreateTemplateVariables {
    pub fn new(attributes: Attributes) -> Self {
        let mut subject = "[EXT] ".to_string();
        subject.push_str(attributes.subject.as_str());
        Self {
            typ: "USER".to_string(),
            attributes: Attributes {
                subject,
                ..attributes
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub phishing_template_create: PhishingTemplateCreate,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplateCreate {
    pub node: Node,
    pub errors: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}
