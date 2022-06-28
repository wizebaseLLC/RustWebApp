use async_graphql::{InputObject, SimpleObject};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AppModelingFormForServiceNow {
    u_name: String,
    u_business_service_1: String,
    u_submitted_by: String,
}

impl AppModelingFormForServiceNow {
    pub fn new(values: AppModelingForm, submitted_by: String) -> Self {
        let u_business_service_1 = serde_json::to_string(&values).unwrap();
        Self {
            u_name: values.name,
            u_business_service_1,
            u_submitted_by: submitted_by,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AppModelingForm {
    #[serde(rename = "u_percent_complete")]
    pub u_percent_complete: Option<i64>,
    #[serde(rename = "u_is_signedoff_cbt")]
    pub u_is_signedoff_cbt: Option<bool>,
    #[serde(rename = "u_signoff_date_cbt")]
    pub u_signoff_date_cbt: Option<String>,
    #[serde(rename = "u_signoff_user_cbt")]
    pub u_signoff_user_cbt: Option<Signoff>,
    #[serde(rename = "u_signoff_date")]
    pub u_signoff_date: Option<String>,
    #[serde(rename = "u_signoff_user")]
    pub u_signoff_user: Option<Signoff>,
    #[serde(rename = "u_gdpr_affirmed_date")]
    pub u_gdpr_affirmed_date: Option<String>,
    #[serde(rename = "u_gdpr_affirmed_user")]
    pub u_gdpr_affirmed_user: Option<Signoff>,
    #[serde(rename = "sys_updated_on")]
    pub sys_updated_on: Option<String>,
    #[serde(rename = "u_last_update_by")]
    pub u_last_update_by: Option<String>,
    #[serde(rename = "sys_created_on")]
    pub sys_created_on: Option<String>,
    #[serde(rename = "child_app")]
    pub child_app: Option<Vec<ChildApp>>,
    #[serde(rename = "parent_app")]
    pub parent_app: Option<Vec<ParentApp>>,
    #[serde(rename = "external_qa")]
    pub external_qa: Option<Vec<ExternalQa>>,
    #[serde(rename = "external_stage")]
    pub external_stage: Option<Vec<ExternalStage>>,
    #[serde(rename = "external_dev")]
    pub external_dev: Option<Vec<ExternalDev>>,
    #[serde(rename = "external_prod")]
    pub external_prod: Option<Vec<ExternalProd>>,
    #[serde(rename = "db_qa")]
    pub db_qa: Option<Vec<DbQa>>,
    #[serde(rename = "db_stage")]
    pub db_stage: Option<Vec<DbStage>>,
    #[serde(rename = "db_dev")]
    pub db_dev: Option<Vec<DbDev>>,
    #[serde(rename = "db_prod")]
    pub db_prod: Option<Vec<DbProd>>,
    #[serde(rename = "host_qa")]
    pub host_qa: Option<Vec<HostQa>>,
    #[serde(rename = "host_stage")]
    pub host_stage: Option<Vec<HostStage>>,
    #[serde(rename = "host_dev")]
    pub host_dev: Option<Vec<HostDev>>,
    #[serde(rename = "host_prod")]
    pub host_prod: Option<Vec<HostProd>>,
    #[serde(rename = "u_biz_functions")]
    pub u_biz_functions: Option<Vec<Vec<Option<String>>>>,
    #[serde(rename = "u_asset_type")]
    pub u_asset_type: Option<Vec<Vec<Option<String>>>>,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: Option<bool>,
    #[serde(rename = "u_service_type")]
    pub u_service_type: Option<String>,
    #[serde(rename = "u_invest_status")]
    pub u_invest_status: Option<String>,
    #[serde(rename = "u_app_replacement_project")]
    pub u_app_replacement_project: Option<UAppReplacementProject>,
    #[serde(rename = "u_app_replacement_date")]
    pub u_app_replacement_date: Option<String>,
    #[serde(rename = "u_app_replacement_name")]
    pub u_app_replacement_name: Option<UAppReplacementName>,
    #[serde(rename = "u_gdpr_comments")]
    pub u_gdpr_comments: Option<String>,
    #[serde(rename = "u_gdpr_recipients")]
    pub u_gdpr_recipients: Option<String>,
    #[serde(rename = "u_is_gpdr_affirmed")]
    pub u_is_gpdr_affirmed: Option<bool>,
    #[serde(rename = "u_is_gdpr_scope")]
    pub u_is_gdpr_scope: Option<String>,
    #[serde(rename = "u_gdpr_personal_data")]
    pub u_gdpr_personal_data: Option<Vec<Option<String>>>,
    #[serde(rename = "u_gdpr_vendor_access")]
    pub u_gdpr_vendor_access: Option<String>,
    #[serde(rename = "u_gdpr_processing_grounds")]
    pub u_gdpr_processing_grounds: Option<Vec<Option<String>>>,
    #[serde(rename = "u_gdpr_individuals")]
    pub u_gdpr_individuals: Option<Vec<Option<String>>>,
    #[serde(rename = "u_gdpr_data_location")]
    pub u_gdpr_data_location: Option<Vec<Option<String>>>,
    #[serde(rename = "u_gdpr_scope_area")]
    pub u_gdpr_scope_area: Option<Vec<Option<String>>>,
    #[serde(rename = "u_has_data_breach")]
    pub u_has_data_breach: Option<String>,
    #[serde(rename = "u_gdpr_person_type")]
    pub u_gdpr_person_type: Option<String>,
    #[serde(rename = "u_is_consent_given_gdpr")]
    pub u_is_consent_given_gdpr: Option<String>,
    #[serde(rename = "u_cloud_strategy_plan_benefits")]
    pub u_cloud_strategy_plan_benefits: Option<String>,
    #[serde(rename = "u_6_rs_plan")]
    pub u6_rs_plan: Option<String>,
    #[serde(rename = "u_cloud_strategy_why")]
    pub u_cloud_strategy_why: Option<Vec<UCloudStrategyWhy>>,
    #[serde(rename = "u_functional_doc_loc")]
    pub u_functional_doc_loc: Option<String>,
    #[serde(rename = "u_dr_doc_loc")]
    pub u_dr_doc_loc: Option<String>,
    #[serde(rename = "u_runbook_loc")]
    pub u_runbook_loc: Option<String>,
    #[serde(rename = "u_entitlement_requested")]
    pub u_entitlement_requested: Option<String>,
    #[serde(rename = "u_app_access_evidence_loc_path")]
    pub u_app_access_evidence_loc_path: Option<String>,
    #[serde(rename = "u_app_access_evidence_loc")]
    pub u_app_access_evidence_loc: Option<String>,
    #[serde(rename = "u_app_access_reviewed_by")]
    pub u_app_access_reviewed_by: Option<UAppAccessReviewedBy>,
    #[serde(rename = "u_user_access_verify_date")]
    pub u_user_access_verify_date: Option<String>,
    #[serde(rename = "u_is_web")]
    pub u_is_web: Option<String>,
    #[serde(rename = "u_ie_11_dependent")]
    pub u_ie11_dependent: Option<String>,
    #[serde(rename = "u_ie_11_explanation")]
    pub u_ie11_explanation: Option<String>,
    #[serde(rename = "u_vip_infoblox_dns")]
    pub u_vip_infoblox_dns: Option<Vec<UVirtualIp>>,
    #[serde(rename = "u_url")]
    pub u_url: Option<String>,
    #[serde(rename = "u_service_acct")]
    pub u_service_acct: Option<Vec<UServiceAcct>>,
    #[serde(rename = "u_entitlement_type")]
    pub u_entitlement_type: Option<String>,
    #[serde(rename = "u_is_db_in_source_control")]
    pub u_is_db_in_source_control: Option<String>,
    #[serde(rename = "u_db_source_control_loc")]
    pub u_db_source_control_loc: Option<String>,
    #[serde(rename = "u_db_types")]
    pub u_db_types: Option<Vec<Option<String>>>,
    #[serde(rename = "u_use_dfs")]
    pub u_use_dfs: Option<String>,
    #[serde(rename = "u_dfs_links")]
    pub u_dfs_links: Option<Vec<UDfsLink>>,
    #[serde(rename = "u_source_code_loc")]
    pub u_source_code_loc: Option<Vec<Option<String>>>,
    #[serde(rename = "u_information_classification")]
    pub u_information_classification: Option<String>,
    #[serde(rename = "u_data_location_type")]
    pub u_data_location_type: Option<Vec<Option<String>>>,
    #[serde(rename = "u_sso")]
    pub u_sso: Option<String>,
    #[serde(rename = "u_deploy_method")]
    pub u_deploy_method: Option<String>,
    #[serde(rename = "u_use_mail")]
    pub u_use_mail: Option<String>,
    #[serde(rename = "u_use_mail_desc")]
    pub u_use_mail_desc: Option<String>,
    #[serde(rename = "u_dev_platform")]
    pub u_dev_platform: Option<Vec<Option<String>>>,
    #[serde(rename = "u_dr_failover_date")]
    pub u_dr_failover_date: Option<String>,
    #[serde(rename = "u_alt_app_name")]
    pub u_alt_app_name: Option<String>,
    #[serde(rename = "u_user_location")]
    pub u_user_location: Option<String>,
    #[serde(rename = "u_num_external")]
    pub u_num_external: Option<String>,
    #[serde(rename = "u_num_internal")]
    pub u_num_internal: Option<String>,
    #[serde(rename = "u_external_app")]
    pub u_external_app: Option<String>,
    #[serde(rename = "u_azure_aws_hosted")]
    pub u_azure_aws_hosted: Option<String>,
    #[serde(rename = "u_ext_audit")]
    pub u_ext_audit: Option<String>,
    #[serde(rename = "u_is_trade_data")]
    pub u_is_trade_data: Option<String>,
    #[serde(rename = "u_vendor_names")]
    pub u_vendor_names: Option<UVendorNames>,
    #[serde(rename = "u_asd_supported_by")]
    pub u_asd_supported_by: Option<String>,
    #[serde(rename = "u_svcnow_queue")]
    pub u_svcnow_queue: Option<USvcnowQueue>,
    #[serde(rename = "u_sdlc_product")]
    pub u_sdlc_product: Option<USdlcProduct>,
    #[serde(rename = "u_appowner_biz_sec")]
    pub u_appowner_biz_sec: Option<UAppownerBizSec>,
    #[serde(rename = "u_appowner_biz_pri")]
    pub u_appowner_biz_pri: Option<UAppownerBizPri>,
    #[serde(rename = "u_appowner_asd_pri")]
    pub u_appowner_asd_pri: Option<UAppownerBizPri>,
    #[serde(rename = "u_appowner_asd_sec")]
    pub u_appowner_asd_sec: Option<UAppownerBizPri>,
    #[serde(rename = "u_asd_manager")]
    pub u_asd_manager: Option<UAppownerBizPri>,
    #[serde(rename = "u_tertiary_asd_owner")]
    pub u_tertiary_asd_owner: Option<UAppownerBizPri>,
    #[serde(rename = "u_svcnow_queue_asd")]
    pub u_svcnow_queue_asd: Option<UAppownerBizPri>,
    #[serde(rename = "u_appowner_sec")]
    pub u_appowner_sec: Option<UAppownerSecSubmit>,
    #[serde(rename = "u_appowner_pri")]
    pub u_appowner_pri: UAppownerPriSubmit,
    #[serde(rename = "u_business_tier")]
    pub u_business_tier: Option<String>,
    #[serde(rename = "u_app_lifecycle")]
    pub u_app_lifecycle: Option<String>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    pub name: String,
    #[serde(rename = "u_vstsproject")]
    pub u_vstsproject: Option<UAppownerBizSec>,
    #[serde(rename = "u_app_admin")]
    pub u_app_admin: Option<UAppownerBizSec>,
    #[serde(rename = "u_codebase_location")]
    pub u_codebase_location: Option<String>,
    #[serde(rename = "u_contract_link")]
    pub u_contract_link: Option<String>,
    #[serde(rename = "u_ip_whitelist")]
    pub u_ip_whitelist: Option<String>,
    #[serde(rename = "u_asd_distribution_list")]
    pub u_asd_distribution_list: Option<UAppownerPriSubmit>,
    #[serde(rename = "u_distribution_list")]
    pub u_distribution_list: Option<UAppownerPriSubmit>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ChildApp {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ParentApp {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalQa {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalStage {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDev {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalProd {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct DbQa {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct DbStage {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct DbDev {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct DbProd {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct HostQa {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct HostStage {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct HostDev {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct HostProd {
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppReplacementProject {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppReplacementName {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UCloudStrategyWhy {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct Signoff {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppAccessReviewedBy {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UVirtualIp {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UServiceAcct {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UDfsLink {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UVendorNames {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct USvcnowQueue {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct USdlcProduct {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppownerBizSec {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppownerBizPri {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppownerSecSubmit {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    InputObject,
)]
#[serde(rename_all = "camelCase")]
pub struct UAppownerPriSubmit {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct AppFormReturnValues {
    pub result: AppFormSubmitResult,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct AppFormSubmitResult {
    #[serde(rename = "u_name")]
    pub u_name: String,
    #[serde(rename = "u_app_relations")]
    pub u_app_relations: String,
    #[serde(rename = "sys_mod_count")]
    pub sys_mod_count: String,
    #[serde(rename = "sys_updated_on")]
    pub sys_updated_on: String,
    #[serde(rename = "u_submitted_by")]
    pub u_submitted_by: String,
    #[serde(rename = "sys_tags")]
    pub sys_tags: String,
    #[serde(rename = "sys_id")]
    pub sys_id: String,
    #[serde(rename = "u_business_service_4")]
    pub u_business_service4: String,
    #[serde(rename = "sys_updated_by")]
    pub sys_updated_by: String,
    #[serde(rename = "u_business_service_3")]
    pub u_business_service3: String,
    #[serde(rename = "u_business_service_2")]
    pub u_business_service2: String,
    #[serde(rename = "u_business_service_1")]
    pub u_business_service1: String,
    #[serde(rename = "sys_created_on")]
    pub sys_created_on: String,
    #[serde(rename = "sys_created_by")]
    pub sys_created_by: String,
}
