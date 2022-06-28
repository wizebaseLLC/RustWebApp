#![allow(non_snake_case)]

extern crate dotenv;

/// Warp Rest endpoint routes
/// TO AUTO RUN
/// RELEASE cargo watch -x 'run -- -release'
/// DEV cargo watch -x 'run'
pub mod api_endpoint_routes;
/// List of currenet Dataloaders to be injected into Async Graphql.
pub mod dataloader_list;
/// Creates the Database connection and pooling functions, which are used by the db_connection_list.
pub mod db;
/// List of current Database connections to be injected into the application.
pub mod db_connection_list;
/// Graphql Query, Mutation and Subscription hanlders.
pub mod graphql;
/// Graphql Schema
pub mod graphql_schema;
/// Functions that help with requests sent to the end point.  This includes shell scripts.
pub mod helpers;
/// App Modeling models and structs used by Tiberius and various rest apis.
pub mod models;
