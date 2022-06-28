use infraportal::api_endpoint_routes::ApiEndpointRoutes;
use infraportal::dataloader_list::DataLoaders;
use infraportal::db_connection_list::DatabaseConnections;
use infraportal::graphql_schema::GraphqlSchema;
use infraportal::helpers::requests::service_now_url::is_prod;
use reqwest::ClientBuilder;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Environment checker.
    let is_prod = is_prod();
    // Set up logging if production.
    if is_prod {
        log4rs::init_file("log4rs.yaml", Default::default())?
    };
    // Rest client that is injected into the Graphql client.
    let rest_client = reqwest::Client::new();
    // Rest client for usage with DEV.  In Dev, there is no HTTPS.
    let rest_client_dangerous = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;
    // Start database connection pools.
    let database_connections = DatabaseConnections::new().await?;
    // Create dataloaders for graphql client
    let dataloaders = DataLoaders::new(database_connections.clone(), rest_client.clone());
    // GraphQL schema builder
    let schema = GraphqlSchema::new(
        database_connections.clone(),
        dataloaders,
        rest_client,
        rest_client_dangerous,
    );
    // Warp Rest Api Routes
    let api_endpoint_routes = ApiEndpointRoutes::new(database_connections, schema);
    // The port that is used is pulled from ENV
    let port = env::var("PORT").ok();
    let port = if let Some(port) = port {
        port.parse::<u16>()?
    } else {
        5002 as u16
    };
    // Run the application
    if is_prod {
        warp::serve(api_endpoint_routes.routes())
            .tls()
            .cert_path("../../../certs/cert_prod/certs.crt")
            .key_path("../../../certs/cert_prod/certs.key")
            .run(([0, 0, 0, 0], port))
            .await;
    } else {
        warp::serve(api_endpoint_routes.routes())
            .run(([0, 0, 0, 0], port))
            .await;
    }

    Ok(())
}
