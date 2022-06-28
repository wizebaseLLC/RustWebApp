use super::{
    constants::QUERY_FIND_APPLICATION_CERT_HISTORY,
    constants::*,
    model::*,
    serde_types::{
        serde_types_all_apps::RootServiceNowApps,
        serde_types_app_details::RootServiceNowAppDetails,
        serde_types_rel_ci::{RelCiTypes, RootServiceNowRelCi},
        serde_types_submit_app_form::{AppModelingForm, AppModelingFormForServiceNow},
    },
};
use crate::{
    db::TiberiusConnectionManager, helpers::requests::service_now_url::service_now_url,
    models::cmdb_ci_service::serde_types::serde_types_submit_app_form::AppFormReturnValues,
};
use bb8::PooledConnection;
use query_tiberius::query_tiberius::QueryTiberius;
use query_tiberius_derive::Queryable;
#[derive(Debug, Queryable, Clone)]
pub struct SysIdQuery {
    pub sys_id: String,
}

impl CmdbCiService {
    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_ALL_CMDB_CI_SERVICE, &[]).await?;

        Self::build_result_many(stream).await
    }

    pub async fn find_cbt_applications<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        cbt_id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_ALL_CBT_APPLICATIONS, &[&cbt_id])
            .await?;

        Self::build_result_many(stream).await
    }

    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_CMDB_CI_SERVICE, &[&app_name]).await?;

        Self::build_result(stream).await
    }

    pub async fn query_service_now_for_matching_apps(
        client: reqwest::Client,
        sys_id: String,
    ) -> anyhow::Result<RootServiceNowApps> {
        Self::query_service_now_get_builder(
            client,
            query_matching_apps_from_sn_url(sys_id).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn query_service_now_for_all_apps(
        client: reqwest::Client,
    ) -> anyhow::Result<RootServiceNowApps> {
        Self::query_service_now_get_builder(client, query_all_app_from_service_now().as_str())
            .await
            .map_err(Into::into)
    }

    pub async fn query_service_now_for_my_apps<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        client: reqwest::Client,
        user_id: String,
    ) -> anyhow::Result<RootServiceNowApps> {
        let stream = conn
            .query(QUERY_FIND_SYS_ID_FROM_SYS_USER_FROM_USERNAME, &[&user_id])
            .await?;

        let stream = SysIdQuery::build_result(stream).await?;

        Self::query_service_now_get_builder(
            client,
            query_my_apps_from_sn_url(stream.sys_id).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn submit_app_modeling_form(
        client: reqwest::Client,
        values: AppModelingForm,
        submitted_by: String,
    ) -> anyhow::Result<AppFormReturnValues> {
        let values = AppModelingFormForServiceNow::new(values, submitted_by);

        Self::service_now_post_builder::<AppFormReturnValues, AppModelingFormForServiceNow>(
            client,
            post_app_modeling_form_url().as_str(),
            &values,
        )
        .await
        .map_err(Into::into)
    }

    pub async fn query_service_now_app_details(
        client: reqwest::Client,
        sys_id: String,
    ) -> anyhow::Result<RootServiceNowAppDetails> {
        Self::query_service_now_get_builder(client, get_app_details_from_sn_url(sys_id).as_str())
            .await
            .map_err(Into::into)
    }

    pub async fn query_service_now_cbt_apps(
        client: reqwest::Client,
        sys_id: String,
    ) -> anyhow::Result<RootServiceNowAppDetails> {
        Self::query_service_now_get_builder(
            client,
            query_cbt_owned_apps_from_sn_url(sys_id).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn query_service_now_rel_ci(
        client: reqwest::Client,
        sys_id: String,
        ci_type: RelCiTypes,
    ) -> anyhow::Result<RootServiceNowRelCi> {
        Self::query_service_now_get_builder(
            client,
            query_rel_ci_from_sn_url(sys_id, ci_type).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn find_incidents<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        sys_id: &str,
    ) -> anyhow::Result<Vec<CmdbCiIncidents>> {
        let stream = conn.query(QUERY_FIND_INCIDENTS_FROM_CI, &[&sys_id]).await?;

        CmdbCiIncidents::build_result_many(stream).await
    }

    pub async fn find_incidents_by_opened_by<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        sys_id: &str,
    ) -> anyhow::Result<Vec<CmdbCiIncidents>> {
        let stream = conn
            .query(QUERY_FIND_INCIDENTS_FROM_OPENED_BY, &[&sys_id])
            .await?;

        CmdbCiIncidents::build_result_many(stream).await
    }

    pub async fn find_changes<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        sys_id: &str,
    ) -> anyhow::Result<Vec<CmdbCiChanges>> {
        let stream = conn.query(QUERY_FIND_CHANGES_FROM_CI, &[&sys_id]).await?;

        CmdbCiChanges::build_result_many(stream).await
    }
}

fn get_app_details_from_sn_url(sys_id: String) -> String {
    format!(
        "{0}/api/now/table/cmdb_ci_service?sysparm_query=sys_id={1}^ORname={1}&sysparm_display_value=all",
        service_now_url(),
        sys_id
    )
}

fn post_app_modeling_form_url() -> String {
    format!(
        "{}/api/now/table/u_infra_portal_app_modeling_staging_area",
        service_now_url()
    )
}

fn query_all_app_from_service_now() -> String {
    format!("{}/api/now/table/cmdb_ci_service?sysparm_query=true%5Eu_active%3Dtrue%5Eu_app_ci_class%3Dapplication%5EORDERBYname&sysparm_display_value=all&sysparm_fields=name%2Csys_id%2Cu_active%2Cu_is_gpdr_affirmed%2Cu_is_signedoff%2Cu_is_signedoff_cbt%2Cu_app_lifecycle%2Cu_appowner_pri",service_now_url())
}

fn query_matching_apps_from_sn_url(sys_id: String) -> String {
    format!("{1}/api/now/table/cmdb_ci_service?sysparm_limit=20&sysparm_query=nameLIKE{0}^ORsys_id={0}&u_active=true&u_app_ci_class=application&sysparm_display_value=all&sysparm_fields=name%2Csys_id%2Cu_active%2Cu_is_gpdr_affirmed%2Cu_is_signedoff%2Cu_is_signedoff_cbt%2Cu_app_lifecycle%2Cu_appowner_pri",sys_id,service_now_url())
}

fn query_my_apps_from_sn_url(user_id: String) -> String {
    format!("{0}/api/now/table/cmdb_ci_service?sysparm_query=true%5Eu_active%3Dtrue%5Eu_app_ci_class%3Dapplication%5Eu_appowner_pri%3D{1}%5EORu_appowner_sec%3D{1} %5EORu_appowner_asd_pri%3D{1}%5EORu_appowner_asd_sec%3D{1}%5EORu_tertiary_asd_owner%3D{1}%5EORu_asd_manager%3D{1}%5EORu_cbt_owner%3D{1}%5Eu_app_lifecycle!%3Ddecommissioned%5EORDERBYname&sysparm_display_value=all&sysparm_fields=name%2Csys_id%2Cu_active%2Cu_is_gpdr_affirmed%2Cu_is_signedoff%2Cu_is_signedoff_cbt%2Cu_app_lifecycle%2Cu_appowner_pri",service_now_url(),user_id,)
}

fn query_cbt_owned_apps_from_sn_url(user_id: String) -> String {
    format!("{0}/api/now/table/cmdb_ci_service?sysparm_query=true%5Eu_active%3Dtrue%5Eu_cbt_owner%3D{1}%5Eu_app_ci_class%3Dapplication%5EORDERBYname&sysparm_display_value=all",service_now_url(),user_id,)
}

pub fn query_rel_ci_from_sn_url(sys_id: String, ci_type: RelCiTypes) -> String {
    match ci_type {
        RelCiTypes::Children => format!(
            "{}/api/now/table/cmdb_rel_ci?sysparm_query=parent={}^childISNOTEMPTY^childNOT LIKEempty&sysparm_display_value=all",
            service_now_url(),
            sys_id
        ),
        RelCiTypes::Parents => format!(
            "{}/api/now/table/cmdb_rel_ci?sysparm_query=child={}&sysparm_display_value=all",
            service_now_url(),
            sys_id
        ),
    }
}
impl CmdbCiServiceCertHistory {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn
            .query(QUERY_FIND_APPLICATION_CERT_HISTORY, &[&app_name])
            .await?;

        Self::build_result(stream).await
    }

    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_LIST_APPLICATION_CERT_HISTORY, &[]).await?;

        Self::build_result_many(stream).await
    }

    pub async fn find_name<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
    ) -> anyhow::Result<CertHistoryAppName> {
        let stream = conn
            .query(QUERY_FIND_APPLICATION_CERT_HISTORY_NAME_ONLY, &[&app_name])
            .await?;

        CertHistoryAppName::build_result(stream).await
    }

    pub async fn update_history<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
        data: &str,
        cert_history: &str,
    ) -> anyhow::Result<bool> {
        conn.execute(
            UPDATE_APPLICATION_CERT_HISTORY,
            &[&app_name, &data, &cert_history],
        )
        .await?;

        Ok(true)
    }

    pub async fn insert_history<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
        create_date: &str,
        data: &str,
        cert_history: &str,
    ) -> anyhow::Result<bool> {
        conn.execute(
            INSERT_APPLICATION_CERT_HISTORY,
            &[&app_name, &create_date, &data, &cert_history],
        )
        .await?;

        Ok(true)
    }

    pub async fn modify_history<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
        create_date: &str,
        data: &str,
        cert_history: &str,
    ) -> anyhow::Result<bool> {
        let does_app_exist = Self::find_name(conn, app_name).await.ok();

        if let Some(_app) = does_app_exist {
            Self::update_history(conn, app_name, data, cert_history).await
        } else {
            Self::insert_history(conn, app_name, create_date, data, cert_history).await
        }
    }
}

impl ModelingScoreCard {
    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_LIST_APPLICATION_SCORECARD_DATA, &[])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl CmdbCiSearchBar {
    pub async fn find_many<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(SEARCH_BAR_QUERY, &[&id]).await?;

        Self::build_result_many(stream).await
    }
}
