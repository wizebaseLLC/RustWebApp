use async_graphql::dataloader::DataLoader;
use reqwest::Client;

use crate::{
    db_connection_list::DatabaseConnections,
    models::{
        cmdb_ci_hardware::dataloaders::{CmdbCiChangesLoader, IsOnlineLoader},
        cmdb_ci_service::dataloaders::{
            DistributionListLoader, IsAppLoader, IsDatabaseLoader, IsDbOnlineLoader,
            IsServerLoader, ServiceNowRelCiChildrenLoader, SysUserLoader,
        },
        knowbe4::dataloaders::{Knowbe4ReportedEmailLoader, Knowbe4ResultsLoader},
    },
};

/// Dataloaders used by Async Graphql for performance
pub struct DataLoaders {
    pub is_server_loader: DataLoader<IsServerLoader>,
    pub is_database_loader: DataLoader<IsDatabaseLoader>,
    pub is_app_loader: DataLoader<IsAppLoader>,
    pub service_now_children_loader: DataLoader<ServiceNowRelCiChildrenLoader>,
    pub sys_user_loader: DataLoader<SysUserLoader>,
    pub is_db_online_loader: DataLoader<IsDbOnlineLoader>,
    pub cmdb_ci_changes_loader: DataLoader<CmdbCiChangesLoader>,
    pub knowbe4_results_loader: DataLoader<Knowbe4ResultsLoader>,
    pub knowbe4_reported_email_loader: DataLoader<Knowbe4ReportedEmailLoader>,
    pub is_online_loader: DataLoader<IsOnlineLoader>,
    pub distribution_list_loader: DataLoader<DistributionListLoader>,
}

impl DataLoaders {
    pub fn new(database_connections: DatabaseConnections, rest_client: Client) -> Self {
        let distribution_list_loader = DataLoader::new(
            DistributionListLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let knowbe4_reported_email_loader = DataLoader::new(
            Knowbe4ReportedEmailLoader::new(database_connections.seceng_connection.clone()),
            tokio::spawn,
        );
        let is_online_loader = DataLoader::new(IsOnlineLoader::new(), tokio::spawn);
        let knowbe4_results_loader = DataLoader::new(
            Knowbe4ResultsLoader::new(database_connections.infraportal_connection.clone()),
            tokio::spawn,
        );
        let is_database_loader = DataLoader::new(
            IsDatabaseLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let cmdb_ci_changes_loader = DataLoader::new(
            CmdbCiChangesLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let is_app_loader = DataLoader::new(
            IsAppLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let is_server_loader = DataLoader::new(
            IsServerLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let sys_user_loader = DataLoader::new(
            SysUserLoader::new(database_connections.sn_mirror_connection.clone()),
            tokio::spawn,
        );
        let is_db_online_loader = DataLoader::new(
            IsDbOnlineLoader::new(
                database_connections.sn_mirror_connection.clone(),
                database_connections.db_cmdb_connection.clone(),
            ),
            tokio::spawn,
        );
        let service_now_children_loader = DataLoader::new(
            ServiceNowRelCiChildrenLoader::new(rest_client.clone()),
            tokio::spawn,
        );

        Self {
            is_server_loader,
            is_database_loader,
            is_app_loader,
            service_now_children_loader,
            sys_user_loader,
            is_db_online_loader,
            cmdb_ci_changes_loader,
            knowbe4_results_loader,
            knowbe4_reported_email_loader,
            is_online_loader,
            distribution_list_loader,
        }
    }
}
