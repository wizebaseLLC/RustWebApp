use async_graphql::Schema;
use reqwest::Client;

use crate::{
    dataloader_list::DataLoaders,
    db_connection_list::DatabaseConnections,
    graphql::{
        context::GraphQLContext, graphql::QueryRoot, graphql_mutation::MutationRoot,
        graphql_subscription::SubscriptionRoot,
    },
};

pub struct GraphqlSchema;

impl GraphqlSchema {
    pub fn new(
        database_connections: DatabaseConnections,
        dataloaders: DataLoaders,
        rest_client: Client,
        rest_client_dangerous: Client,
    ) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
            .data(GraphQLContext {
                sn_mirror_connection: database_connections.sn_mirror_connection.clone(),
                infraportal_connection: database_connections.infraportal_connection.clone(),
                sccm_connection: database_connections.sccm_connection.clone(),
                altiris_connection: database_connections.altiris_connection.clone(),
                db_cmdb_connection: database_connections.db_cmdb_connection.clone(),
                blue_connection: database_connections.blue_connection.clone(),
                nb_disable_connection: database_connections.nb_disable_connection.clone(),
                seceng_connection: database_connections.seceng_connection.clone(),
                rest_client,
                rest_client_dangerous,
            })
            .data(dataloaders.is_server_loader)
            .data(dataloaders.is_database_loader)
            .data(dataloaders.is_app_loader)
            .data(dataloaders.service_now_children_loader)
            .data(dataloaders.sys_user_loader)
            .data(dataloaders.is_db_online_loader)
            .data(dataloaders.cmdb_ci_changes_loader)
            .data(dataloaders.knowbe4_results_loader)
            .data(dataloaders.knowbe4_reported_email_loader)
            .data(dataloaders.is_online_loader)
            .data(dataloaders.distribution_list_loader)
            .finish()
    }
}
