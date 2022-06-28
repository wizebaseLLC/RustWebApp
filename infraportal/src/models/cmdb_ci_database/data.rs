use super::{
    constants::*,
    model::{CmdbCiDatabase, DbModelingScoreCard},
    serde_types::{
        db_form_return_data::DbFormReturnData,
        serde_cmdb_ci_database::{CmdbCiDatabaseJson, MyDatabaseListFromServiceNow},
        submit_db_form::DbModelingSubmit,
    },
};
use crate::db::TiberiusConnectionManager;
use crate::helpers::requests::service_now_url::service_now_url;
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;

impl CmdbCiDatabase {
    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_ALL_CMDB_CI_DATABASE, &[]).await?;

        Self::build_result_many(stream).await
    }

    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_CMDB_CI_DATABASE, &[&id]).await?;

        Self::build_result(stream).await
    }

    pub async fn find_from_service_now<'a>(
        client: reqwest::Client,
        sys_id: &str,
    ) -> anyhow::Result<CmdbCiDatabaseJson> {
        Self::query_service_now_get_builder(
            client,
            query_database_from_sn_url(sys_id.to_string()).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn submit_db_form<'a>(
        client: reqwest::Client,
        values: DbModelingSubmit,
        sys_id: String,
    ) -> anyhow::Result<DbFormReturnData> {
        Self::service_now_put_builder(client, put_database_form(sys_id).as_str(), &values)
            .await
            .map_err(Into::into)
    }

    pub async fn list_my_databases_from_service_now<'a>(
        client: reqwest::Client,
        my_id: &str,
    ) -> anyhow::Result<MyDatabaseListFromServiceNow> {
        Self::query_service_now_get_builder(
            client,
            query_my_databases_from_sn_url(my_id.to_string()).as_str(),
        )
        .await
        .map_err(Into::into)
    }

    pub async fn list_all_databases_from_service_now<'a>(
        client: reqwest::Client,
    ) -> anyhow::Result<MyDatabaseListFromServiceNow> {
        Self::query_service_now_get_builder(client, list_all_databases_from_sn_url().as_str())
            .await
            .map_err(Into::into)
    }
}

impl DbModelingScoreCard {
    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_LIST_DATABASE_SCORECARD_DATA, &[]).await?;

        Self::build_result_many(stream).await
    }
}

fn query_database_from_sn_url(id: String) -> String {
    format!("{0}/api/now/table/cmdb_ci_database?sysparm_display_value=all&sysparm_fields=name%2Cu_dataserver_name%2Cu_cname%2Cu_db_engine%2Cu_app%2Cu_sec_owner%2Cu_cbt_owner%2Cshort_description%2Cu_source_of_data%2Cu_is_decom%2Cu_information_classification%2Cu_signoff_user%2Cu_signoff_date%2Cu_is_vendor_component%2Cu_is_signedoff%2Cu_pri_owner%2Cu_is_auth_source%2Cu_db_tier&sysparm_limit=1&sys_id={1}",service_now_url(),id,)
}

fn query_my_databases_from_sn_url(id: String) -> String {
    format!("{0}/api/now/table/cmdb_ci_database?sysparm_display_value=all&sysparm_fields=name%2Cu_is_signedoff%2Cu_is_signedoff%2Csys_id&sysparm_query=u_pri_owner.user_name={1}^ORu_sec_owner.user_name={1}^ORu_cbt_owner.user_name={1}^u_is_decom=false^u_active=true^discovery_source!=user_request^ORDERBYname",service_now_url(),id,)
}

fn list_all_databases_from_sn_url() -> String {
    format!("{0}/api/now/table/cmdb_ci_database?sysparm_display_value=all&sysparm_fields=name%2Cu_is_signedoff%2Csys_id&sysparm_query=u_active=true^u_is_decom=false^discovery_source!=user_request^ORDERBYname",service_now_url(),)
}

fn put_database_form(id: String) -> String {
    format!(
        "{0}/api/now/table/cmdb_ci_database/{1}",
        service_now_url(),
        id,
    )
}
