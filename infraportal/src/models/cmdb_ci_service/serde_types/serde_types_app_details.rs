use async_graphql::SimpleObject;

use crate::models::cmdb_ci_database::serde_types::{
    db_form_return_data::ModelId, serde_cmdb_ci_database::*,
};

use super::{serde_types_all_apps::*, serde_types_rel_ci::*};

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
pub struct RootServiceNowAppDetails {
    pub result: Vec<ServiceNowAppDetailsResult>,
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
#[graphql(rename_fields = "snake_case")]
pub struct ServiceNowAppDetailsResult {
    #[serde(rename = "attested_date")]
    pub attested_date: AttestedDate,
    pub parent: Parent,
    #[serde(rename = "u_cmdb_id")]
    pub u_cmdb_id: UCmdbId,
    #[serde(rename = "sys_updated_on")]
    pub sys_updated_on: SysUpdatedOn,
    #[serde(rename = "u_created_on")]
    pub u_created_on: UCreatedOn,
    #[serde(rename = "discovery_source")]
    pub discovery_source: DiscoverySource,
    #[serde(rename = "due_in")]
    pub due_in: DueIn,
    #[serde(rename = "u_num_external")]
    pub u_num_external: UNumExternal,
    #[serde(rename = "u_appowner_biz_pri")]
    pub u_appowner_biz_pri: UAppownerBizPri,
    #[serde(rename = "used_for")]
    pub used_for: UsedFor,
    #[serde(rename = "u_svcnow_queue_asd")]
    pub u_svcnow_queue_asd: USvcnowQueueAsd,
    #[serde(rename = "u_gdpr_personal_data")]
    pub u_gdpr_personal_data: UGdprPersonalData,
    #[serde(rename = "gl_account")]
    pub gl_account: GlAccount,
    #[serde(rename = "sys_created_by")]
    pub sys_created_by: SysCreatedBy,
    #[serde(rename = "u_app_access_evidence_loc_path")]
    pub u_app_access_evidence_loc_path: UAppAccessEvidenceLocPath,
    pub version: Version,
    #[serde(rename = "maintenance_schedule")]
    pub maintenance_schedule: MaintenanceSchedule,
    #[serde(rename = "cost_center")]
    pub cost_center: CostCenter,
    #[serde(rename = "attested_by")]
    pub attested_by: AttestedBy,
    #[serde(rename = "dns_domain")]
    pub dns_domain: DnsDomain,
    #[serde(rename = "u_url")]
    pub u_url: UUrl,
    #[serde(rename = "u_azure_aws_hosted")]
    pub u_azure_aws_hosted: UAzureAwsHosted,
    pub assigned: Assigned,
    #[serde(rename = "u_pw_code_conv_date")]
    pub u_pw_code_conv_date: UPwCodeConvDate,
    #[serde(rename = "u_tertiary_asd_owner")]
    pub u_tertiary_asd_owner: UTertiaryAsdOwner,
    #[serde(rename = "life_cycle_stage")]
    pub life_cycle_stage: LifeCycleStage,
    #[serde(rename = "u_app_id")]
    pub u_app_id: UAppId,
    #[serde(rename = "business_need")]
    pub business_need: BusinessNeed,
    #[serde(rename = "end_date")]
    pub end_date: EndDate,
    #[serde(rename = "u_appowner_biz_sec")]
    pub u_appowner_biz_sec: UAppownerBizSec,
    #[serde(rename = "busines_criticality")]
    pub busines_criticality: BusinesCriticality,
    #[serde(rename = "managed_by")]
    pub managed_by: ManagedBy,
    #[serde(rename = "u_cloud_strategy_why")]
    pub u_cloud_strategy_why: UCloudStrategyWhy,
    #[serde(rename = "u_dr_failover_date")]
    pub u_dr_failover_date: UDrFailoverDate,
    #[serde(rename = "last_discovered")]
    pub last_discovered: LastDiscovered,
    #[serde(rename = "u_db_source_conv_date")]
    pub u_db_source_conv_date: UDbSourceConvDate,
    #[serde(rename = "u_is_trade_data")]
    pub u_is_trade_data: UIsTradeData,
    #[serde(rename = "u_information_classification")]
    pub u_information_classification: UInformationClassification,
    #[serde(rename = "life_cycle_stage_status")]
    pub life_cycle_stage_status: LifeCycleStageStatus,
    pub vendor: Vendor,
    #[serde(rename = "u_app_access_reviewed_by")]
    pub u_app_access_reviewed_by: UAppAccessReviewedBy,
    #[serde(rename = "u_app_replacement_project")]
    pub u_app_replacement_project: UAppReplacementProject,
    #[serde(rename = "u_source_code_loc")]
    pub u_source_code_loc: USourceCodeLoc,
    #[serde(rename = "u_arb_review")]
    pub u_arb_review: UArbReview,
    #[serde(rename = "u_num_internal")]
    pub u_num_internal: UNumInternal,
    #[serde(rename = "u_ie11_dependent")]
    pub u_ie11_dependent: UIe11Dependent,
    #[serde(rename = "u_entitlement_requested")]
    pub u_entitlement_requested: UEntitlementRequested,
    #[serde(rename = "u_sdlc_product")]
    pub u_sdlc_product: USdlcProduct,
    #[serde(rename = "u_is_gdpr_scope")]
    pub u_is_gdpr_scope: UIsGdprScope,
    #[serde(rename = "price_unit")]
    pub price_unit: PriceUnit,
    pub u: U,
    #[serde(rename = "correlation_id")]
    pub correlation_id: CorrelationId,
    pub unverified: Unverified,
    #[serde(rename = "u_service_type")]
    pub u_service_type: UServiceType,
    #[serde(rename = "u_ie11_explanation")]
    pub u_ie11_explanation: UIe11Explanation,
    #[serde(rename = "u_signoff_user")]
    pub u_signoff_user: USignoffUser,
    #[serde(rename = "u_is_signedoff")]
    pub u_is_signedoff: UIsSignedoff,
    #[serde(rename = "u_service_acct")]
    pub u_service_acct: UServiceAcct,
    #[serde(rename = "u_gdpr_affirmed_user")]
    pub u_gdpr_affirmed_user: UGdprAffirmedUser,
    #[serde(rename = "service_level_requirement")]
    pub service_level_requirement: ServiceLevelRequirement,
    #[serde(rename = "u_business_tier")]
    pub u_business_tier: UBusinessTier,
    #[serde(rename = "sys_created_on")]
    pub sys_created_on: SysCreatedOn,
    #[serde(rename = "u_active")]
    pub u_active: UActive,
    #[serde(rename = "install_date")]
    pub install_date: InstallDate,
    #[serde(rename = "monitoring_requirements")]
    pub monitoring_requirements: MonitoringRequirements,
    #[serde(rename = "u_is_gpdr_affirmed")]
    pub u_is_gpdr_affirmed: UIsGpdrAffirmed,
    #[serde(rename = "u_ext_audit")]
    pub u_ext_audit: UExtAudit,
    #[serde(rename = "u_signoff_date")]
    pub u_signoff_date: USignoffDate,
    #[serde(rename = "u_svcnow_queue")]
    pub u_svcnow_queue: USvcnowQueue,
    pub fqdn: Fqdn,
    #[serde(rename = "u_use_dfs")]
    pub u_use_dfs: UUseDfs,
    #[serde(rename = "u_app_access_evidence_loc")]
    pub u_app_access_evidence_loc: UAppAccessEvidenceLoc,
    #[serde(rename = "unit_description")]
    pub unit_description: UnitDescription,
    #[serde(rename = "u_signoff_user_cbt")]
    pub u_signoff_user_cbt: USignoffUserCbt,
    #[serde(rename = "u_gdpr_data_location")]
    pub u_gdpr_data_location: UGdprDataLocation,
    #[serde(rename = "u_percent_complete")]
    pub u_percent_complete: UPercentComplete,
    #[serde(rename = "compatibility_dependencies")]
    pub compatibility_dependencies: CompatibilityDependencies,
    #[serde(rename = "u_runbook_loc")]
    pub u_runbook_loc: URunbookLoc,
    #[serde(rename = "u_tech_owner")]
    pub u_tech_owner: UTechOwner,
    pub name: Name,
    #[serde(rename = "u_gdpr_comments")]
    pub u_gdpr_comments: UGdprComments,
    pub subcategory: Subcategory,
    #[serde(rename = "price_model")]
    pub price_model: PriceModel,
    #[serde(rename = "u_user_access_verify_date")]
    pub u_user_access_verify_date: UUserAccessVerifyDate,
    #[serde(rename = "u_additional_it_approvers")]
    pub u_additional_it_approvers: UAdditionalItApprovers,
    #[serde(rename = "u_has_data_breach")]
    pub u_has_data_breach: UHasDataBreach,
    #[serde(rename = "assignment_group")]
    pub assignment_group: AssignmentGroup,
    #[serde(rename = "u_instance_env")]
    pub u_instance_env: UInstanceEnv,
    #[serde(rename = "managed_by_group")]
    pub managed_by_group: ManagedByGroup,
    #[serde(rename = "u_app_replacement_date")]
    pub u_app_replacement_date: UAppReplacementDate,
    #[serde(rename = "u_is_web")]
    pub u_is_web: UIsWeb,
    #[serde(rename = "sys_id")]
    pub sys_id: SysId,
    #[serde(rename = "u_pw_code_approach")]
    pub u_pw_code_approach: UPwCodeApproach,
    #[serde(rename = "u_dr_doc_loc")]
    pub u_dr_doc_loc: UDrDocLoc,
    #[serde(rename = "mac_address")]
    pub mac_address: MacAddress,
    pub company: Company,
    #[serde(rename = "u_deploy_method")]
    pub u_deploy_method: UDeployMethod,
    #[serde(rename = "u_gdpr_scope_area")]
    pub u_gdpr_scope_area: UGdprScopeArea,
    #[serde(rename = "u_last_update_by")]
    pub u_last_update_by: ULastUpdateBy,
    pub monitor: Monitor,
    #[serde(rename = "u_is_signedoff_cbt")]
    pub u_is_signedoff_cbt: UIsSignedoffCbt,
    #[serde(rename = "ip_address")]
    pub ip_address: IpAddress,
    #[serde(rename = "model_id")]
    pub model_id: ModelId,
    #[serde(rename = "duplicate_of")]
    pub duplicate_of: DuplicateOf,
    #[serde(rename = "sys_tags")]
    pub sys_tags: SysTags,
    #[serde(rename = "u_6_rs_plan")]
    pub u6_rs_plan: U6RsPlan,
    #[serde(rename = "cost_cc")]
    pub cost_cc: CostCc,
    #[serde(rename = "order_date")]
    pub order_date: OrderDate,
    pub schedule: Schedule,
    pub environment: Environment,
    #[serde(rename = "u_app_admin")]
    pub u_app_admin: UAppAdmin,
    pub attested: Attested,
    pub location: Location,
    #[serde(rename = "lease_id")]
    pub lease_id: LeaseId,
    #[serde(rename = "service_classification")]
    pub service_classification: ServiceClassification,
    #[serde(rename = "operational_status")]
    pub operational_status: OperationalStatus,
    #[serde(rename = "u_app_model")]
    pub u_app_model: UAppModel,
    #[serde(rename = "u_appowner_asd_pri")]
    pub u_appowner_asd_pri: UAppownerAsdPri,
    #[serde(rename = "u_cloud_strategy_plan_benefits")]
    pub u_cloud_strategy_plan_benefits: UCloudStrategyPlanBenefits,
    #[serde(rename = "u_dfs_links")]
    pub u_dfs_links: UDfsLinks,
    #[serde(rename = "u_signoff_date_cbt")]
    pub u_signoff_date_cbt: USignoffDateCbt,
    pub number: Number,
    #[serde(rename = "first_discovered")]
    pub first_discovered: FirstDiscovered,
    #[serde(rename = "u_gdpr_recipients")]
    pub u_gdpr_recipients: UGdprRecipients,
    #[serde(rename = "invoice_number")]
    pub invoice_number: InvoiceNumber,
    #[serde(rename = "warranty_expiration")]
    pub warranty_expiration: WarrantyExpiration,
    #[serde(rename = "u_invest_status")]
    pub u_invest_status: UInvestStatus,
    #[serde(rename = "u_vstsproject")]
    pub u_vstsproject: UVstsproject,
    pub sla: Sla,
    #[serde(rename = "owned_by")]
    pub owned_by: OwnedBy,
    #[serde(rename = "u_appowner_pri")]
    pub u_appowner_pri: UAppownerPri,
    #[serde(rename = "checked_out")]
    pub checked_out: CheckedOut,
    #[serde(rename = "sys_domain_path")]
    pub sys_domain_path: SysDomainPath,
    #[serde(rename = "u_ip_whitelist")]
    pub u_ip_whitelist: UIpWhitelist,
    #[serde(rename = "u_asset_type")]
    pub u_asset_type: UAssetType,
    #[serde(rename = "u_gdpr_individuals")]
    pub u_gdpr_individuals: UGdprIndividuals,
    #[serde(rename = "purchase_date")]
    pub purchase_date: PurchaseDate,
    #[serde(rename = "delivery_manager")]
    pub delivery_manager: DeliveryManager,
    #[serde(rename = "short_description")]
    pub short_description: ShortDescription,
    #[serde(rename = "u_alt_app_name")]
    pub u_alt_app_name: UAltAppName,
    #[serde(rename = "u_data_location_type")]
    pub u_data_location_type: UDataLocationType,
    #[serde(rename = "can_print")]
    pub can_print: CanPrint,
    #[serde(rename = "sys_class_name")]
    pub sys_class_name: SysClassName,
    pub manufacturer: Manufacturer,
    #[serde(rename = "u_use_mail_desc")]
    pub u_use_mail_desc: UUseMailDesc,
    #[serde(rename = "u_biz_functions")]
    pub u_biz_functions: UBizFunctions,
    #[serde(rename = "u_external_app")]
    pub u_external_app: UExternalApp,
    #[serde(rename = "u_appowner_asd_sec")]
    pub u_appowner_asd_sec: UAppownerAsdSec,
    #[serde(rename = "model_number")]
    pub model_number: ModelNumber,
    #[serde(rename = "assigned_to")]
    pub assigned_to: AssignedTo,
    #[serde(rename = "start_date")]
    pub start_date: StartDate,
    #[serde(rename = "u_app_ci_class")]
    pub u_app_ci_class: UAppCiClass,
    #[serde(rename = "u_app_admin_name")]
    pub u_app_admin_name: UAppAdminName,
    #[serde(rename = "serial_number")]
    pub serial_number: SerialNumber,
    #[serde(rename = "u_is_db_in_source_control")]
    pub u_is_db_in_source_control: UIsDbInSourceControl,
    #[serde(rename = "support_group")]
    pub support_group: SupportGroup,
    #[serde(rename = "u_last_update_on")]
    pub u_last_update_on: ULastUpdateOn,
    pub attributes: Attributes,
    pub asset: Asset,
    #[serde(rename = "u_ad_groups")]
    pub u_ad_groups: UAdGroups,
    #[serde(rename = "u_dev_platform")]
    pub u_dev_platform: UDevPlatform,
    #[serde(rename = "skip_sync")]
    pub skip_sync: SkipSync,
    pub aliases: Aliases,
    #[serde(rename = "attestation_score")]
    pub attestation_score: AttestationScore,
    #[serde(rename = "u_gdpr_affirmed_date")]
    pub u_gdpr_affirmed_date: UGdprAffirmedDate,
    #[serde(rename = "u_asd_manager")]
    pub u_asd_manager: UAsdManager,
    #[serde(rename = "sys_updated_by")]
    pub sys_updated_by: SysUpdatedBy,
    #[serde(rename = "u_entitlement_type")]
    pub u_entitlement_type: UEntitlementType,
    #[serde(rename = "sys_domain")]
    pub sys_domain: SysDomain,
    #[serde(rename = "u_vip_infoblox_dns")]
    pub u_vip_infoblox_dns: UVirtualIps,
    #[serde(rename = "u_is_consent_given_gdpr")]
    pub u_is_consent_given_gdpr: UIsConsentGivenGdpr,
    #[serde(rename = "asset_tag")]
    pub asset_tag: AssetTag,
    #[serde(rename = "user_group")]
    pub user_group: UserGroup,
    #[serde(rename = "u_application_readiness")]
    pub u_application_readiness: UApplicationReadiness,
    #[serde(rename = "u_db_source_control_loc")]
    pub u_db_source_control_loc: UDbSourceControlLoc,
    #[serde(rename = "change_control")]
    pub change_control: ChangeControl,
    #[serde(rename = "u_gdpr_vendor_access")]
    pub u_gdpr_vendor_access: UGdprVendorAccess,
    #[serde(rename = "business_relation_manager")]
    pub business_relation_manager: BusinessRelationManager,
    #[serde(rename = "u_cbt_owner")]
    pub u_cbt_owner: UCbtOwner,
    #[serde(rename = "last_review_date")]
    pub last_review_date: LastReviewDate,
    #[serde(rename = "business_contact")]
    pub business_contact: BusinessContact,
    #[serde(rename = "delivery_date")]
    pub delivery_date: DeliveryDate,
    #[serde(rename = "install_status")]
    pub install_status: InstallStatus,
    #[serde(rename = "u_asd_supported_by")]
    pub u_asd_supported_by: UAsdSupportedBy,
    #[serde(rename = "supported_by")]
    pub supported_by: SupportedBy,
    #[serde(rename = "u_user_location")]
    pub u_user_location: UUserLocation,
    #[serde(rename = "u_functional_doc_loc")]
    pub u_functional_doc_loc: UFunctionalDocLoc,
    #[serde(rename = "u_app_lifecycle")]
    pub u_app_lifecycle: UAppLifecycle,
    #[serde(rename = "u_appowner_sec")]
    pub u_appowner_sec: UAppownerSec,
    #[serde(rename = "u_gdpr_person_type")]
    pub u_gdpr_person_type: UGdprPersonType,
    #[serde(rename = "u_is_open_source")]
    pub u_is_open_source: UIsOpenSource,
    #[serde(rename = "u_testing_group")]
    pub u_testing_group: UTestingGroup,
    #[serde(rename = "u_db_types")]
    pub u_db_types: UDbTypes,
    #[serde(rename = "u_gdpr_processing_grounds")]
    pub u_gdpr_processing_grounds: UGdprProcessingGrounds,
    pub prerequisites: Prerequisites,
    #[serde(rename = "po_number")]
    pub po_number: PoNumber,
    #[serde(rename = "checked_in")]
    pub checked_in: CheckedIn,
    #[serde(rename = "sys_class_path")]
    pub sys_class_path: SysClassPath,
    #[serde(rename = "u_sso")]
    pub u_sso: USso,
    pub justification: Justification,
    pub department: Department,
    pub comments: Comments,
    pub cost: Cost,
    #[serde(rename = "sys_mod_count")]
    pub sys_mod_count: SysModCount,
    #[serde(rename = "u_use_mail")]
    pub u_use_mail: UUseMail,
    #[serde(rename = "u_app_replacement_name")]
    pub u_app_replacement_name: UAppReplacementName,
    #[serde(rename = "u_evidence_verification")]
    pub u_evidence_verification: UEvidenceVerification,
    pub due: Due,
    pub category: Category,
    #[serde(rename = "fault_count")]
    pub fault_count: FaultCount,
    #[serde(rename = "u_vendor_names")]
    pub u_vendor_names: UVendorNames,
    #[serde(rename = "u_codebase_location")]
    pub u_codebase_location: DefaultValue,
    #[serde(rename = "u_contract_link")]
    pub u_contract_link: DefaultValue,
    #[serde(rename = "u_asd_distribution_list")]
    pub u_asd_distribution_list: DefaultValue,
    #[serde(rename = "u_distribution_list")]
    pub u_distribution_list: DefaultValue,
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
pub struct UVirtualIps {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppReplacementName {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DefaultValue {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct AttestedDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Parent {
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub value: String,
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
pub struct UCmdbId {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UCreatedOn {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DiscoverySource {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DueIn {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UNumExternal {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppownerBizPri {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UsedFor {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USvcnowQueueAsd {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprPersonalData {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct GlAccount {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppAccessEvidenceLocPath {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Version {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct MaintenanceSchedule {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CostCenter {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct AttestedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DnsDomain {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUrl {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAzureAwsHosted {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct Assigned {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UPwCodeConvDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UTertiaryAsdOwner {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct LifeCycleStage {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppId {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct BusinessNeed {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct EndDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppownerBizSec {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct BusinesCriticality {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ManagedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UCloudStrategyWhy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDrFailoverDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct LastDiscovered {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDbSourceConvDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIsTradeData {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct LifeCycleStageStatus {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Vendor {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppAccessReviewedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppReplacementProject {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USourceCodeLoc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UArbReview {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UNumInternal {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIe11Dependent {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UEntitlementRequested {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USdlcProduct {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct UIsGdprScope {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct PriceUnit {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct U {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CorrelationId {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Unverified {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UServiceType {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIe11Explanation {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USignoffUser {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UServiceAcct {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprAffirmedUser {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ServiceLevelRequirement {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UBusinessTier {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct InstallDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct MonitoringRequirements {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UExtAudit {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct USvcnowQueue {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct Fqdn {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUseDfs {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppAccessEvidenceLoc {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UnitDescription {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct USignoffUserCbt {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprDataLocation {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UPercentComplete {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CompatibilityDependencies {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct URunbookLoc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UTechOwner {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprComments {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Subcategory {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct PriceModel {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUserAccessVerifyDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAdditionalItApprovers {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UHasDataBreach {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct AssignmentGroup {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UInstanceEnv {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct ManagedByGroup {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppReplacementDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIsWeb {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UPwCodeApproach {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UDrDocLoc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct MacAddress {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Company {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDeployMethod {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprScopeArea {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ULastUpdateBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct Monitor {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct IpAddress {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DuplicateOf {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct U6RsPlan {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CostCc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct OrderDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Schedule {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Environment {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UAppAdmin {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Attested {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Location {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct LeaseId {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ServiceClassification {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct OperationalStatus {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppModel {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UAppownerAsdPri {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UCloudStrategyPlanBenefits {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDfsLinks {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USignoffDateCbt {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Number {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct FirstDiscovered {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprRecipients {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct InvoiceNumber {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct WarrantyExpiration {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UInvestStatus {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UVstsproject {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Sla {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct OwnedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CheckedOut {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SysDomainPath {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIpWhitelist {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UAssetType {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprIndividuals {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct PurchaseDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DeliveryManager {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAltAppName {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDataLocationType {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CanPrint {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SysClassName {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Manufacturer {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUseMailDesc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UBizFunctions {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UExternalApp {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppownerAsdSec {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ModelNumber {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct AssignedTo {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct StartDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppCiClass {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppAdminName {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SerialNumber {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIsDbInSourceControl {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct SupportGroup {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ULastUpdateOn {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Attributes {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Asset {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAdGroups {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDevPlatform {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SkipSync {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Aliases {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct AttestationScore {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprAffirmedDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAsdManager {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UEntitlementType {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SysDomain {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct UIsConsentGivenGdpr {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct AssetTag {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UserGroup {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UApplicationReadiness {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UDbSourceControlLoc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct ChangeControl {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprVendorAccess {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct BusinessRelationManager {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct LastReviewDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct BusinessContact {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct DeliveryDate {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct InstallStatus {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAsdSupportedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SupportedBy {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUserLocation {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct UFunctionalDocLoc {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UAppownerSec {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,

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
pub struct UGdprPersonType {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UIsOpenSource {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UTestingGroup {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UDbTypes {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UGdprProcessingGrounds {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Prerequisites {
    #[serde(rename = "display_value")]
    pub display_value: ::serde_json::Value,
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
pub struct PoNumber {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct CheckedIn {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct SysClassPath {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct USso {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Justification {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Department {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Comments {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Cost {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UUseMail {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UEvidenceVerification {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Due {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct Category {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct FaultCount {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
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
pub struct UVendorNames {
    #[serde(rename = "display_value")]
    pub display_value: Option<String>,
    pub value: Option<String>,
}
