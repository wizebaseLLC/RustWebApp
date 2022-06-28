use crate::{graphql::graphql_subscription::MutationType, models::sys_user::model::SysUser};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, FieldResult, Object, SimpleObject,
};
use chrono::{NaiveDate, NaiveDateTime};
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};

use super::{
    dataloaders::{DistributionListLoader, SysUserLoader},
    serde_types::{
        serde_types_cert_history::ApplicationCertHistory,
        serde_types_snapshot_history::ApplicationSnapShotHistory,
    },
};

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
#[graphql(complex)]
pub struct CmdbCiService {
    pub name: Option<String>,
    pub sys_id: String,
    pub u_active: Option<String>,
    pub u_appowner_pri: Option<String>,
    pub dv_u_appowner_pri: Option<String>,
    pub u_appowner_sec: Option<String>,
    pub dv_u_appowner_sec: Option<String>,
    pub u_cbt_owner: Option<String>,
    pub dv_u_cbt_owner: Option<String>,
    pub u_appowner_asd_pri: Option<String>,
    pub dv_u_appowner_asd_pri: Option<String>,
    pub u_appowner_asd_sec: Option<String>,
    pub dv_u_appowner_asd_sec: Option<String>,
    pub u_appowner_biz_pri: Option<String>,
    pub dv_u_appowner_biz_pri: Option<String>,
    pub u_appowner_biz_sec: Option<String>,
    pub dv_u_appowner_biz_sec: Option<String>,
    pub u_asd_manager: Option<String>,
    pub dv_u_asd_manager: Option<String>,
    pub u_tertiary_asd_owner: Option<String>,
    pub dv_u_tertiary_asd_owner: Option<String>,
    pub u_tech_owner: Option<String>,
    pub dv_u_tech_owner: Option<String>,
    pub u_business_tier: Option<String>,
    pub u_app_lifecycle: Option<String>,
    pub u_service_type: Option<String>,
    pub u_external_app: Option<String>,
    pub u_ie11_explanation: Option<String>,
    pub u_ie11_dependent: Option<String>,
    pub u_asset_type: Option<String>,
    pub u_app_access_evidence_loc: Option<String>,
    pub dv_asset: Option<String>,
    pub asset: Option<String>,
    pub u_app_replacement_name: Option<String>,
    pub u_app_replacement_date: Option<NaiveDate>,
    pub dv_u_app_replacement_project: Option<String>,
    pub u_app_replacement_project: Option<String>,
    pub asset_tag: Option<String>,
    pub u_azure_aws_hosted: Option<String>,
    pub assigned_to: Option<String>,
    pub u_vstsproject: Option<String>,
    pub u_dr_failover_date: Option<NaiveDate>,
    pub dv_u_vstsproject: Option<String>,
    pub dv_assigned_to: Option<String>,
    pub assignment_group: Option<String>,
    pub dv_assignment_group: Option<String>,
    pub company: Option<String>,
    pub dv_company: Option<String>,
    pub department: Option<String>,
    pub dv_department: Option<String>,
    pub justification: Option<String>,
    pub location: Option<String>,
    pub dv_location: Option<String>,
    pub managed_by: Option<String>,
    pub dv_managed_by: Option<String>,
    pub manufacturer: Option<String>,
    pub dv_manufacturer: Option<String>,
    pub model_id: Option<String>,
    pub dv_model_id: Option<String>,
    pub owned_by: Option<String>,
    pub dv_owned_by: Option<String>,
    pub supported_by: Option<String>,
    pub dv_supported_by: Option<String>,
    pub support_group: Option<String>,
    pub dv_support_group: Option<String>,
    pub sys_class_name: Option<String>,
    pub sys_class_path: Option<String>,
    pub sys_created_by: Option<String>,
    pub sys_updated_by: Option<String>,
    pub sys_updated_on: Option<NaiveDateTime>,
    pub vendor: Option<String>,
    pub dv_vendor: Option<String>,
    pub category: Option<String>,
    pub comments: Option<String>,
    pub operational_status: Option<i64>,
    pub short_description: Option<String>,
    pub u_additional_it_approvers: Option<String>,
    pub dv_u_additional_it_approvers: Option<String>,
    pub busines_criticality: Option<String>,
    pub user_group: Option<String>,
    pub dv_user_group: Option<String>,
    pub u_6_rs_plan: Option<String>,
    pub u_ad_groups: Option<String>,
    pub dv_u_ad_groups: Option<String>,
    pub u_alt_app_name: Option<String>,
    pub u_app_access_evidence_loc_path: Option<String>,
    pub u_app_access_reviewed_by: Option<String>,
    pub dv_u_app_access_reviewed_by: Option<String>,
    pub u_app_admin: Option<String>,
    pub dv_u_app_admin: Option<String>,
    pub u_app_ci_class: Option<String>,
    pub u_app_model: Option<String>,
    pub u_asd_supported_by: Option<String>,
    pub u_biz_functions: Option<String>,
    pub u_cloud_strategy_plan_benefits: Option<String>,
    pub u_cloud_strategy_why: Option<String>,
    pub dv_u_cloud_strategy_why: Option<String>,
    pub u_cmdb_id: Option<String>,
    pub u_created_on: Option<NaiveDateTime>,
    pub u_data_location_type: Option<String>,
    pub u_db_source_control_loc: Option<String>,
    pub u_db_source_conv_date: Option<NaiveDate>,
    pub u_db_types: Option<String>,
    pub u_deploy_method: Option<String>,
    pub u_dev_platform: Option<String>,
    pub u_dfs_links: Option<String>,
    pub u_dr_doc_loc: Option<String>,
    pub u_entitlement_requested: Option<String>,
    pub u_entitlement_type: Option<String>,
    pub u_ext_audit: Option<String>,
    pub u_functional_doc_loc: Option<String>,
    pub u_gdpr_affirmed_date: Option<NaiveDate>,
    pub u_gdpr_affirmed_user: Option<String>,
    pub dv_u_gdpr_affirmed_user: Option<String>,
    pub u_gdpr_comments: Option<String>,
    pub u_gdpr_data_location: Option<String>,
    pub u_ip_whitelist: Option<String>,
    pub u_gdpr_individuals: Option<String>,
    pub u_gdpr_personal_data: Option<String>,
    pub u_gdpr_person_type: Option<String>,
    pub u_gdpr_processing_grounds: Option<String>,
    pub u_gdpr_recipients: Option<String>,
    pub u_gdpr_scope_area: Option<String>,
    pub u_gdpr_vendor_access: Option<String>,
    pub u_has_data_breach: Option<String>,
    pub u_information_classification: Option<String>,
    pub u_instance_env: Option<String>,
    pub u_invest_status: Option<String>,
    pub u_is_consent_given_gdpr: Option<String>,
    pub u_is_db_in_source_control: Option<String>,
    pub u_is_gdpr_scope: Option<String>,
    pub u_is_gpdr_affirmed: Option<String>,
    pub u_is_open_source: Option<String>,
    pub u_is_signedoff: Option<String>,
    pub u_is_signedoff_cbt: Option<String>,
    pub u_is_trade_data: Option<String>,
    pub u_is_web: Option<String>,
    pub u_last_update_by: Option<String>,
    pub dv_u_last_update_by: Option<String>,
    pub u_last_update_on: Option<NaiveDateTime>,
    pub u_num_external: Option<i64>,
    pub u_num_internal: Option<i64>,
    pub u_runbook_loc: Option<String>,
    pub u_sdlc_product: Option<String>,
    pub dv_u_sdlc_product: Option<String>,
    pub u_service_acct: Option<String>,
    pub dv_u_service_acct: Option<String>,
    pub u_signoff_date: Option<NaiveDate>,
    pub u_signoff_date_cbt: Option<NaiveDate>,
    pub u_signoff_user: Option<String>,
    pub dv_u_signoff_user: Option<String>,
    pub u_signoff_user_cbt: Option<String>,
    pub dv_u_signoff_user_cbt: Option<String>,
    pub u_source_code_loc: Option<String>,
    pub u_svcnow_queue: Option<String>,
    pub dv_u_svcnow_queue: Option<String>,
    pub u_svcnow_queue_asd: Option<String>,
    pub dv_u_svcnow_queue_asd: Option<String>,
    pub u_url: Option<String>,
    pub u_user_access_verify_date: Option<NaiveDate>,
    pub u_user_location: Option<String>,
    pub u_use_dfs: Option<String>,
    pub u_use_mail: Option<String>,
    pub u_use_mail_desc: Option<String>,
    pub u_vendor_names: Option<String>,
    pub dv_u_vendor_names: Option<String>,  
    pub u_vip_infoblox_dns: Option<String>,
    pub dv_u_vip_infoblox_dns: Option<String>,
    pub version: Option<String>,
    pub u_percent_complete: Option<String>,
    pub u_sso: Option<String>,
    pub dv_u_sso: Option<String>,
    pub sys_created_on: Option<NaiveDateTime>,
    pub u_codebase_location: Option<String>,
    pub u_contract_link: Option<String>,
    pub u_asd_distribution_list: Option<String>,
    pub dv_u_asd_distribution_list: Option<String>,
    pub u_distribution_list: Option<String>,
    pub dv_u_distribution_list: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable, Clone)]
pub struct DistributionListDetails {
    pub u_display_name: Option<String>,
    pub sys_id: String,
    pub email: Option<String>,
    pub u_primaryowner: Option<String>,
    pub dv_u_primaryowner: Option<String>,
    pub u_secondaryowner: Option<String>,
    pub dv_u_secondaryowner: Option<String>,
}

#[ComplexObject]
impl CmdbCiService {
    pub async fn distribution_list_details(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<DistributionListDetails>> {
        if let Some(value) = self.u_distribution_list.clone() {
            let data: Option<DistributionListDetails> = ctx
                .data_unchecked::<DataLoader<DistributionListLoader>>()
                .load_one(value)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn asd_distribution_list_details(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<DistributionListDetails>> {
        if let Some(value) = self.u_asd_distribution_list.clone() {
            let data: Option<DistributionListDetails> = ctx
                .data_unchecked::<DataLoader<DistributionListLoader>>()
                .load_one(value)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_owner(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_pri.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sysUserSecondaryOwner(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_sec.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sysUserASDOwner(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_asd_pri.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sysUserSecondaryASDOwner(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_asd_sec.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_tertiary_asd_owner(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_tertiary_asd_owner.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_asd_manager(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_asd_manager.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_business_owner(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_biz_pri.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_business_secondary_owner(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_appowner_biz_sec.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }

    pub async fn sys_user_cbt_owner(&self, ctx: &Context<'_>) -> FieldResult<Option<SysUser>> {
        if let Some(owner) = self.u_cbt_owner.clone() {
            let data: Option<SysUser> = ctx
                .data_unchecked::<DataLoader<SysUserLoader>>()
                .load_one(owner)
                .await?;

            Ok(data)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable)]
pub struct CmdbCiServiceCertHistory {
    pub data: String,
    pub name: Option<String>,
    pub cert_history: Option<String>,
    pub create_date: Option<String>,
}

#[Object]
impl CmdbCiServiceCertHistory {
    async fn create_date(&self) -> Option<&str> {
        self.create_date.as_deref()
    }

    async fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    async fn cert_history(&self) -> Option<Vec<ApplicationCertHistory>> {
        if let Some(history) = &self.cert_history {
            serde_json::from_str(history.as_str()).ok()
        } else {
            None
        }
    }

    async fn data(&self) -> Option<ApplicationSnapShotHistory> {
        serde_json::from_str(self.data.as_str()).ok()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct ModelingScoreCard {
    pub cbt: Option<String>,
    pub is_infra: Option<i32>,
    pub total: Option<i32>,
    pub n_owner_signoff: Option<i32>,
    pub n_cbt_signoff: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SimpleObject, Queryable)]
#[graphql(rename_fields = "snake_case")]
pub struct CmdbCiChanges {
    pub sys_id: String,
    pub cmdb_ci: Option<String>,
    pub dv_cmdb_ci: Option<String>,
    pub number: Option<String>,
    pub short_description: Option<String>,
    pub dv_opened_by: Option<String>,
    pub dv_assignment_group: Option<String>,
    pub state: Option<i64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub opened_at: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
#[graphql(rename_fields = "snake_case")]
pub struct CmdbCiIncidents {
    pub sys_id: String,
    pub cmdb_ci: Option<String>,
    pub dv_cmdb_ci: Option<String>,
    pub number: Option<String>,
    pub short_description: Option<String>,
    pub dv_opened_by: Option<String>,
    pub dv_assignment_group: Option<String>,
    pub state: Option<i64>,
    pub opened_at: Option<NaiveDateTime>,
    pub priority: Option<i64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
#[graphql(rename_fields = "snake_case")]
pub struct CmdbCiSearchBar {
    pub sys_id: String,
    pub name: Option<String>,
    pub user_name: Option<String>,
    pub table_name: Option<String>,
    pub employee_number: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct CertHistoryAppName {
    pub name: String,
}

#[derive(Clone, SimpleObject)]
pub struct AppSubmitted {
    pub mutation_type: MutationType,
    pub name: String,
    pub submitted_by: String,
}
