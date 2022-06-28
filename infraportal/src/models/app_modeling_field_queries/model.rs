use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SsoPartners {
    pub sys_id: String,
    pub u_id: Option<i64>,
    pub u_partner: Option<String>,
    pub u_partnerapp: Option<String>,
    pub u_partnerpid: Option<String>,
    pub u_partnersys_id: Option<String>,
    pub u_partnerurl1: Option<String>,
    pub u_partnerurl2: Option<String>,
    pub u_partnerurl3: Option<String>,
    pub u_partnerurlrelaystate1: Option<String>,
    pub u_partnerurlrelaystate2: Option<String>,
    pub u_partnerurlrelaystate3: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct VstsProjects {
    pub sys_id: String,
    pub u_description: Option<String>,
    pub u_name: Option<String>,
    pub u_url: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct GetMoreUsers {
    pub sys_id: String,
    pub u_display_name: Option<String>,
    pub user_name: Option<String>,
    pub employee_number: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct DfsLinks {
    pub sys_id: String,
    pub u_path: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct Vips {
    pub sys_id: String,
    pub u_vip_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct Vendors {
    pub sys_id: String,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct DistributionLists {
    pub sys_id: String,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SixRs {
    pub sys_id: String,
    pub u_why: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct ServiceAccounts {
    pub sys_id: String,
    pub u_user_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct GetMoreServers {
    pub sys_id: String,
    pub name: Option<String>,
    pub sys_class_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct GetMoreDatabases {
    pub sys_id: String,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct GetMoreApps {
    pub sys_id: String,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct CmdbModels {
    pub sys_id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct ActiveDirectoryGroups {
    pub sys_id: String,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct AppAdmin {
    pub sys_id: String,
    pub u_name: Option<String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SysChoiceList {
    pub sys_id: String,
    pub label: Option<String>,
    pub value: Option<String>,
    pub element: Option<String>,
    pub name: Option<String>,
    pub hint: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum SysChoiceListOptions {
    CmdbCiService,
    CmdbCiDatabase,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct PmProject {
    pub sys_id: String,
    pub short_description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SysDictionary {
    pub column_label: Option<String>,
    pub comments: Option<String>,
    pub element: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct CmdbCiCertificate {
    pub sys_id: String,
    pub u_application_ownership: Option<String>,
    pub dv_u_application_ownership: Option<String>,
    pub issuer: Option<String>,
    pub dv_issuer: Option<String>,
    pub state: Option<String>,
    pub subject_alternative_name: Option<String>,
    pub dv_subject_alternative_name: Option<String>,
    pub valid_from: Option<NaiveDateTime>,
    pub valid_to: Option<NaiveDateTime>,
    pub subject_common_name: Option<String>,
    pub sys_updated_on: Option<NaiveDateTime>,
    pub subject_organization: Option<String>,
    pub is_self_signed: Option<String>,
    pub issuer_common_name: Option<String>,
    pub fingerprint: Option<String>,
    pub signature_algorithm: Option<String>,
    pub version: Option<i64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct BlueRelations {
    pub id: Option<String>,
    pub service_id: Option<String>,
    pub app_name: Option<String>,
    pub host_id: Option<String>,
    pub env: Option<i32>,
    pub physical_location: Option<String>,
}
