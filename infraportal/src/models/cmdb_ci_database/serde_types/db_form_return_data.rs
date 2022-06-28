use async_graphql::SimpleObject;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::models::cmdb_ci_service::serde_types::serde_types_app_details::USignoffUser;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct DbFormReturnData {
    pub result: DbFormReturnDataResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
#[serde(rename_all = "camelCase")]
pub struct DbFormReturnDataResult {
    #[serde(rename = "attested_date")]
    pub attested_date: Option<String>,
    #[serde(rename = "operational_status")]
    pub operational_status: Option<String>,
    #[serde(rename = "u_pri_owner")]
    pub u_pri_owner: Option<UPriOwner>,
    #[serde(rename = "sys_updated_on")]
    pub sys_updated_on: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "u_is_vendor_component")]
    pub u_is_vendor_component: Option<String>,
    #[serde(rename = "discovery_source")]
    pub discovery_source: Option<String>,
    #[serde(rename = "first_discovered")]
    pub first_discovered: Option<String>,
    #[serde(rename = "due_in")]
    pub due_in: Option<String>,
    #[serde(rename = "gl_account")]
    pub gl_account: Option<String>,
    #[serde(rename = "invoice_number")]
    pub invoice_number: Option<String>,
    #[serde(rename = "sys_created_by")]
    pub sys_created_by: Option<String>,
    #[serde(rename = "u_is_decom")]
    pub u_is_decom: Option<String>,
    #[serde(rename = "warranty_expiration")]
    pub warranty_expiration: Option<String>,
    #[serde(rename = "owned_by")]
    pub owned_by: Option<String>,
    #[serde(rename = "u_source_of_data")]
    pub u_source_of_data: Option<String>,
    #[serde(rename = "checked_out")]
    pub checked_out: Option<String>,
    #[serde(rename = "sys_domain_path")]
    pub sys_domain_path: Option<String>,
    #[serde(rename = "business_unit")]
    pub business_unit: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "u_sec_owner")]
    pub u_sec_owner: Option<USecOwner>,
    #[serde(rename = "maintenance_schedule")]
    pub maintenance_schedule: Option<String>,
    #[serde(rename = "cost_center")]
    pub cost_center: Option<String>,
    #[serde(rename = "attested_by")]
    pub attested_by: Option<String>,
    #[serde(rename = "dns_domain")]
    pub dns_domain: Option<String>,
    pub assigned: Option<String>,
    #[serde(rename = "life_cycle_stage")]
    pub life_cycle_stage: Option<String>,
    #[serde(rename = "purchase_date")]
    pub purchase_date: Option<String>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    #[serde(rename = "db_server")]
    pub db_server: Option<String>,
    #[serde(rename = "managed_by")]
    pub managed_by: Option<String>,
    #[serde(rename = "u_db_tier")]
    pub u_db_tier: Option<String>,
    #[serde(rename = "can_print")]
    pub can_print: Option<String>,
    #[serde(rename = "last_discovered")]
    pub last_discovered: Option<String>,
    #[serde(rename = "sys_class_name")]
    pub sys_class_name: Option<String>,
    pub manufacturer: Option<String>,
    #[serde(rename = "u_information_classification")]
    pub u_information_classification: Option<String>,
    #[serde(rename = "life_cycle_stage_status")]
    pub life_cycle_stage_status: Option<String>,
    pub vendor: Option<String>,
    #[serde(rename = "model_number")]
    pub model_number: Option<String>,
    #[serde(rename = "assigned_to")]
    pub assigned_to: Option<String>,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "u_db_engine")]
    pub u_db_engine: Option<String>,
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    #[serde(rename = "support_group")]
    pub support_group: Option<String>,
    #[serde(rename = "correlation_id")]
    pub correlation_id: Option<String>,
    pub unverified: Option<String>,
    pub attributes: Option<String>,
    pub asset: Option<String>,
    #[serde(rename = "skip_sync")]
    pub skip_sync: Option<String>,
    #[serde(rename = "u_is_auth_source")]
    pub u_is_auth_source: Option<String>,
    #[serde(rename = "u_signoff_user")]
    pub u_signoff_user: Option<USignoffUser>,
    #[serde(rename = "u_db_name")]
    pub u_db_name: Option<String>,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: Option<String>,
    #[serde(rename = "attestation_score")]
    pub attestation_score: Option<String>,
    #[serde(rename = "sys_updated_by")]
    pub sys_updated_by: Option<String>,
    #[serde(rename = "sys_created_on")]
    pub sys_created_on: Option<String>,
    #[serde(rename = "sys_domain")]
    pub sys_domain: Option<SysDomains>,
    #[serde(rename = "u_active")]
    pub u_active: Option<String>,
    #[serde(rename = "install_date")]
    pub install_date: Option<String>,
    #[serde(rename = "asset_tag")]
    pub asset_tag: Option<String>,
    #[serde(rename = "u_application_readiness")]
    pub u_application_readiness: Option<String>,
    #[serde(rename = "u_dataserver_name")]
    pub u_dataserver_name: Option<String>,
    #[serde(rename = "u_signoff_date")]
    pub u_signoff_date: Option<String>,
    pub fqdn: Option<String>,
    #[serde(rename = "change_control")]
    pub change_control: Option<String>,
    #[serde(rename = "u_cbt_owner")]
    pub u_cbt_owner: Option<String>,
    #[serde(rename = "delivery_date")]
    pub delivery_date: Option<String>,
    #[serde(rename = "u_dbid")]
    pub u_dbid: Option<String>,
    #[serde(rename = "install_status")]
    pub install_status: Option<String>,
    #[serde(rename = "supported_by")]
    pub supported_by: Option<String>,
    #[serde(rename = "u_tech_owner")]
    pub u_tech_owner: Option<String>,
    pub name: Option<String>,
    pub subcategory: Option<String>,
    #[serde(rename = "u_env")]
    pub u_env: Option<String>,
    #[serde(rename = "u_additional_it_approvers")]
    pub u_additional_it_approvers: Option<String>,
    #[serde(rename = "assignment_group")]
    pub assignment_group: Option<String>,
    #[serde(rename = "u_testing_group")]
    pub u_testing_group: Option<String>,
    #[serde(rename = "u_app")]
    pub u_app: UApp,
    #[serde(rename = "managed_by_group")]
    pub managed_by_group: Option<String>,
    #[serde(rename = "sys_id")]
    pub sys_id: Option<String>,
    #[serde(rename = "po_number")]
    pub po_number: Option<String>,
    #[serde(rename = "checked_in")]
    pub checked_in: Option<String>,
    #[serde(rename = "sys_class_path")]
    pub sys_class_path: Option<String>,
    #[serde(rename = "mac_address")]
    pub mac_address: Option<String>,
    #[serde(rename = "u_cname")]
    pub u_cname: Option<String>,
    pub company: Option<String>,
    pub justification: Option<String>,
    pub department: Option<String>,
    pub comments: Option<String>,
    pub cost: Option<String>,
    #[serde(rename = "sys_mod_count")]
    pub sys_mod_count: Option<String>,
    pub monitor: Option<String>,
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    #[serde(rename = "model_id")]
    pub model_id: Option<ModelId>,
    #[serde(rename = "duplicate_of")]
    pub duplicate_of: Option<String>,
    #[serde(rename = "sys_tags")]
    pub sys_tags: Option<String>,
    #[serde(rename = "cost_cc")]
    pub cost_cc: Option<String>,
    #[serde(rename = "order_date")]
    pub order_date: Option<String>,
    pub schedule: Option<String>,
    pub environment: Option<String>,
    pub due: Option<String>,
    pub attested: Option<String>,
    pub location: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "fault_count")]
    pub fault_count: Option<String>,
    #[serde(rename = "lease_id")]
    pub lease_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct UPriOwner {
    pub link: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct USecOwner {
    pub link: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct SysDomains {
    pub link: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct UApp {
    pub link: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct ModelId {
    pub link: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
}
