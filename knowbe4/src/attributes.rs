#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Attributes {
    pub name: String,
    pub subject: String,
    pub from: String,
    pub rating: Option<String>,
    #[serde(rename = "fromDisplayName")]
    pub from_display_name: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(rename = "replyToDisplayName")]
    pub reply_to_display_name: String,
    #[serde(rename = "contentHtml")]
    pub content_html: String,
    #[serde(rename = "attachmentFilename")]
    pub attachment_file_name: Option<String>,
    #[serde(rename = "attachmentType")]
    pub attachment_type: Option<String>,
    #[serde(rename = "landingDomainId")]
    pub landing_domain_id: Option<i32>,
    #[serde(rename = "landingPageId")]
    pub landing_page_id: Option<i32>,
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AttributesWithId {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub from: String,
    pub rating: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "fromDisplayName")]
    pub from_display_name: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(rename = "replyToDisplayName")]
    pub reply_to_display_name: String,
    #[serde(rename = "contentHtml")]
    pub content_html: String,
    #[serde(rename = "attachmentFilename")]
    pub attachment_file_name: Option<String>,
    #[serde(rename = "attachmentType")]
    pub attachment_type: Option<String>,
    #[serde(rename = "landingDomainId")]
    pub landing_domain_id: Option<i32>,
    #[serde(rename = "landingPageId")]
    pub landing_page_id: Option<i32>,
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
}
