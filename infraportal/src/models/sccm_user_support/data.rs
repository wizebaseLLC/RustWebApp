use super::{constants::*, model::*, serde_types::splunk_login_history::*};
use crate::db::TiberiusConnectionManager;
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;

impl SccmPcHardware {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_SCCM_PC_HARDWARE, &[&id]).await?;

        Self::build_result(stream).await
    }
}

impl SccmPcInstalledSoftware {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_SCCM_PC_INSTALLED_SOFTWARE, &[&id])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl SccmPcLoggedInUser {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn
            .query(QUERY_FIND_SCCM_PC_LOGGED_IN_USER, &[&id])
            .await?;

        Self::build_result(stream).await
    }
}

impl SccmUserPrimaryPc {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_SCCM_PRIMARY_PC, &[&id]).await?;

        Self::build_result(stream).await
    }
}

impl SccmUserProfile {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_SCCM_USER_PROFILE, &[&id]).await?;

        Self::build_result(stream).await
    }
}

impl AltirisUserDrives {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_SCCM_USER_DRIVES, &[&id]).await?;

        Self::build_result_many(stream).await
    }
}

impl AltirisUserConferenceRoomDesktops {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_SCCM_USER_CONFERENCE_DESKTOPS, &[&id])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl SnMirrorUserTasks {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_SN_MIRROR_USER_TASK, &[&id]).await?;

        Self::build_result_many(stream).await
    }
}

impl LoginHistoryBlankSlate {
    pub async fn query_user_login_history(
        client: reqwest::Client,
        user_name: String,
    ) -> anyhow::Result<LoginHistory> {
        let url = format!("https://splunksh.nb.com:8089/services/search/jobs/export?output_mode=json_rows&search=savedsearch infraportal_login_history user={}&summarize=true&preview=false&adhoc_search_level=fast",user_name);
        Self::query_splunk(client, url.as_str())
            .await
            .map_err(Into::into)
    }

    pub async fn query_desktop_login_history(
        client: reqwest::Client,
        desktop_name: String,
    ) -> anyhow::Result<LoginHistory> {
        let url = format!("https://splunksh.nb.com:8089/services/search/jobs/export?output_mode=json_rows&search=savedsearch infraportal_desktop_login_history  desktop={}&summarize=true&preview=false&adhoc_search_level=fast",desktop_name);
        Self::query_splunk(client, url.as_str())
            .await
            .map_err(Into::into)
    }
}
