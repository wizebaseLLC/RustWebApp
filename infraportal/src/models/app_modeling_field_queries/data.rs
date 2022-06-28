use super::{constants::*, model::*};
use crate::{db::TiberiusConnectionManager, helpers::requests::keys_to_sql::keys_to_sql_in_query};
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;
impl SsoPartners {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_ALL_SSO_PARTNERS, &[]).await?;

        Self::build_result_many(stream).await
    }
}

impl VstsProjects {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_MATCHING_VSTS_PROJECTS, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl GetMoreUsers {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_USERS, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl DfsLinks {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_DFS_LINKS, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl Vips {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_VIPS, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl Vendors {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_VENDORS, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl SixRs {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_ALL_SIX_R, &[]).await?;

        Self::build_result_many(stream).await
    }
}

impl ServiceAccounts {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_MATCHING_SERVICE_ACCOUNTS, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl GetMoreServers {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_SERVERS, &[&query]).await?;

        Self::build_result_many(stream).await
    }

    pub async fn find_many<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        ids: &[String],
    ) -> anyhow::Result<Vec<Self>> {
        let keys = keys_to_sql_in_query(ids);
        let stream = conn.query(query_many_servers(keys).as_str(), &[]).await?;

        Self::build_result_many(stream).await
    }
}

impl GetMoreDatabases {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_DATABASES, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl GetMoreApps {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_APPS, &[&query]).await?;

        Self::build_result_many(stream).await
    }

    pub async fn find_instance<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_MATCHING_APP_INSTANCES, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl CmdbModels {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_MATCHING_CMDB_MODELS, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl DistributionLists {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_DISTRIBUTION_LIST, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl ActiveDirectoryGroups {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_FIND_MATCHING_ACTIVE_DIRECTORY_GROUPS, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl AppAdmin {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_MATCHING_APP_ADMIN, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

fn set_cmdb_ci_choice(choice: SysChoiceListOptions) -> &'static str {
    match choice {
        SysChoiceListOptions::CmdbCiService => "cmdb_ci_service",
        SysChoiceListOptions::CmdbCiDatabase => "cmdb_ci_database",
    }
}

impl SysChoiceList {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        choice: SysChoiceListOptions,
    ) -> anyhow::Result<Vec<Self>> {
        let choice = set_cmdb_ci_choice(choice);
        let stream = conn.query(QUERY_FIND_SYS_CHOICE_LIST, &[&choice]).await?;

        Self::build_result_many(stream).await
    }
}

impl PmProject {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_PM_PROJECT, &[&query]).await?;

        Self::build_result_many(stream).await
    }
}

impl SysDictionary {
    pub async fn find_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        choice: SysChoiceListOptions,
    ) -> anyhow::Result<Vec<Self>> {
        let choice = set_cmdb_ci_choice(choice);
        let stream = conn.query(QUERY_FIND_SYS_DICTIONARY, &[&choice]).await?;

        Self::build_result_many(stream).await
    }
}

impl CmdbCiCertificate {
    pub async fn find_matching<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        query: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_MATCHING_CMDB_CI_CERTIFICATES, &[&query])
            .await?;

        Self::build_result_many(stream).await
    }
}

impl BlueRelations {
    pub async fn find_matching<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        app_name: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_BLUE_RELATIONS, &[&app_name]).await?;

        Self::build_result_many(stream).await
    }
}
