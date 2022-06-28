use super::{
    constants::*,
    model::{CmdbCiHostLookup, CmdbCiServer, CmdbCiServerParentApps},
};
use crate::{db::TiberiusConnectionManager, helpers::requests::keys_to_sql::keys_to_sql_in_query};
use bb8::PooledConnection;
use query_tiberius::query_tiberius::QueryTiberius;
use query_tiberius::v_sphere::{VsphereMetrics, VsphereMetricsInput, VsphereSeverProperties};

impl CmdbCiServer {
    pub async fn find_many<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        ids: &[String],
    ) -> anyhow::Result<Vec<Self>> {
        let keys = keys_to_sql_in_query(ids);
        let stream = conn
            .query(query_list_cmdb_ci_servers(keys).as_str(), &[])
            .await?;

        Self::build_result_many(stream).await
    }

    pub async fn query_vsphere_metrics(
        client: reqwest::Client,
        input: VsphereMetricsInput,
    ) -> anyhow::Result<VsphereMetrics> {
        Self::get_vsphere_metrics(client, input)
            .await
            .map_err(Into::into)
    }

    pub async fn query_vsphere_server_properties(
        client: reqwest::Client,
        resource_id: String,
    ) -> anyhow::Result<VsphereSeverProperties> {
        Self::get_vsphere_server_properties(client, resource_id)
            .await
            .map_err(Into::into)
    }
}

impl CmdbCiServerParentApps {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        ids: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_LIST_CMDB_CI_SERVER_PARENT_APPS, &[&ids])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl CmdbCiHostLookup {
    pub async fn find_many<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        ids: &[String],
    ) -> anyhow::Result<Vec<Self>> {
        let keys = keys_to_sql_in_query(ids);
        let stream = conn
            .query(query_list_host_lookup(keys).as_str(), &[])
            .await?;

        Self::build_result_many(stream).await
    }
}
