use super::constants::*;
use super::model::{NbDisable, NbDisableEnableInput, NbDisableInput, NbDisableInputBody};
use crate::db::TiberiusConnectionManager;
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;

impl NbDisable {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        host_name: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_NB_DISABLE, &[&host_name]).await?;
        Self::build_result_many(stream).await
    }

    pub async fn list_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(LIST_ALL_NB_DISABLE, &[]).await?;
        Self::build_result_many(stream).await
    }

    pub async fn post_nb_disable(
        client: reqwest::Client,
        values: NbDisableInput,
    ) -> anyhow::Result<String> {
        let values = NbDisableInputBody::new(values);
        let url = "http://itsmportal.nb.com/hpovtools_ws/ovdisable2.pl";
        Self::post_to_itsm_portal(client, url, &values)
            .await
            .map_err(Into::into)
    }

    pub async fn post_nb_enable(
        client: reqwest::Client,
        values: NbDisableEnableInput,
    ) -> anyhow::Result<String> {
        let url = "http://itsmportal.nb.com/hpovtools_ws/ovdisable2.pl";
        Self::post_to_itsm_portal(client, url, &values)
            .await
            .map_err(Into::into)
    }
}
