use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct CmdbCiDatabase {
    pub sys_id: String,
    pub name: Option<String>,
    pub u_db_name: Option<String>,
    pub u_db_engine: Option<String>,
    pub u_active: Option<String>,
    pub u_pri_owner: Option<String>,
    pub dv_u_pri_owner: Option<String>,
    pub u_sec_owner: Option<String>,
    pub dv_u_sec_owner: Option<String>,
    pub short_description: Option<String>,
    pub u_information_classification: Option<String>,
    pub sys_class_name: Option<String>,
    pub u_signoff_user: Option<String>,
    pub dv_u_signoff_user: Option<String>,
    pub u_is_signedoff: Option<String>,
    pub u_signoff_date: Option<NaiveDateTime>,
    pub u_is_auth_source: Option<String>,
    pub u_is_vendor_component: Option<String>,
    pub u_dataserver_name: Option<String>,
    pub u_db_tier: Option<String>,
    pub u_is_decom: Option<String>,
    pub dv_u_app: Option<String>,
    pub dv_u_cbt: Option<String>,
    pub u_app: Option<String>,
    pub field_type: Option<String>,
    pub model_id: Option<String>,
    pub u_cname: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct DbModelingScoreCard {
    pub cbt: Option<String>,
    pub is_infra: Option<i32>,
    pub total: Option<i32>,
    pub n_owner_signoff: Option<i32>,
}
