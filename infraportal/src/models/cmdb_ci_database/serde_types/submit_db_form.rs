#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbModelingSubmit {
    pub name: Option<String>,
    #[serde(rename = "u_is_decom")]
    pub u_is_decom: Option<bool>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    #[serde(rename = "u_db_engine")]
    pub u_db_engine: Option<String>,
    #[serde(rename = "u_pri_owner")]
    pub u_pri_owner: Option<String>,
    #[serde(rename = "u_sec_owner")]
    pub u_sec_owner: Option<String>,
    #[serde(rename = "u_app")]
    pub u_app: Option<String>,
    #[serde(rename = "u_db_tier")]
    pub u_db_tier: Option<String>,
    #[serde(rename = "u_is_auth_source")]
    pub u_is_auth_source: Option<String>,
    #[serde(rename = "u_information_classification")]
    pub u_information_classification: Option<String>,
    #[serde(rename = "u_is_vendor_component")]
    pub u_is_vendor_component: Option<String>,
    #[serde(rename = "u_signoff_user")]
    pub u_signoff_user: Option<String>,
    #[serde(rename = "u_signoff_date")]
    pub u_signoff_date: Option<String>,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: Option<bool>,
    #[serde(rename = "u_cname")]
    pub u_cname: Option<String>,
}
