use serde::{Deserialize, Serialize};

pub const LIST_TEMPLATES_FROM_CATEGORIES: &str = r#"

query phishingTemplates(
    $category_id: Int
    $type: PhishingTemplateTypes!
    $search: String
    $showHidden: Boolean
    $showExcluded: Boolean
    $sortDirection: SortDirections
    $sortField: EmailTemplateSortFields
    $per: Int
    $page: Int
    $drafts: Boolean
  ) {
    phishingTemplates(
      categoryId: $category_id
      type: $type
      search: $search
      showHidden: $showHidden
      showExcluded: $showExcluded
      sortDirection: $sortDirection
      sortField: $sortField
      per: $per
      page: $page
      drafts: $drafts
    ) {
      nodes {
        id
        name
        subject
        createdAt
        updatedAt
        rating
        from
        fromDisplayName
        replyTo
        replyToDisplayName
        contentHtml
        attachmentFilename
        attachmentType
        landingDomainId
        landingPageId
        languageCode
        hidden
        excluded
        category {
          id
          hidden
          name
        }
      }
      pagination {
        pages
        page
        per
        totalCount
      }
    }
    phishingTemplateCount(type: $type)
  }
  
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateVariables {
    #[serde(rename = "type")]
    pub typ: String,
    pub category_id: Option<i32>,
    pub drafts: bool,
    pub per: i32,
    pub page: i32,
}

impl TemplateVariables {
    pub fn new(category_id: Option<i32>) -> Self {
        Self {
            category_id,
            drafts: false,
            per: 1000,
            page: 1,
            typ: "USER".to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplatesRoot {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub phishing_templates: PhishingTemplates,
    pub phishing_template_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhishingTemplates {
    pub nodes: Vec<Node>,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub created_at: String,
    pub updated_at: String,
    pub rating: Option<String>,
    pub from: String,
    pub from_display_name: String,
    pub reply_to: String,
    pub reply_to_display_name: String,
    pub content_html: String,
    pub attachment_filename: Option<String>,
    pub attachment_type: Option<String>,
    pub landing_domain_id: Option<i32>,
    pub landing_page_id: Option<i32>,
    pub language_code: Option<String>,
    pub hidden: bool,
    pub excluded: bool,
    pub category: Category,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub hidden: bool,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub pages: i64,
    pub page: i64,
    pub per: i64,
    pub total_count: i64,
}
