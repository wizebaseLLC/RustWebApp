use std::convert::Infallible;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use bb8::Pool;
use warp::{cors::Builder, http::Response as HttpResponse, Filter, Rejection, Reply};

use crate::db::TiberiusConnectionManager;
use crate::graphql::context::MyToken;
use crate::models::qtrak::model::QtrakUser;
use crate::{
    db_connection_list::DatabaseConnections,
    graphql::{
        graphql::QueryRoot, graphql_mutation::MutationRoot, graphql_subscription::SubscriptionRoot,
    },
    helpers::{
        powershell_command::models::CgiNodeResult,
        requests::struct_to_csv::{out_csv_app_modeling, out_csv_db_modeling},
    },
};
use async_graphql_warp::{graphql_subscription, GraphQLResponse as Response};

/// Api endpoint data returned by Warp REST API.
pub struct ApiEndpointRoutes {
    database_connections: DatabaseConnections,
    schema: Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
}

/// Generate API data.
impl ApiEndpointRoutes {
    /// Instantiate
    pub fn new(
        database_connections: DatabaseConnections,
        schema: Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
    ) -> Self {
        Self {
            database_connections,
            schema,
        }
    }

    /// GraphQL data.
    pub fn graphql_post(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("api").and(
            warp::header::optional::<String>("auth")
                .and(warp::header::optional::<String>("impersonatedUser"))
                .and(async_graphql_warp::graphql(self.schema.clone()))
                .and_then(
                    |token,
                     impersonated_user,
                     (schema, mut request): (
                        Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
                        async_graphql::Request,
                    )| async move {
                        if let Some(token) = token {
                            if let Some(impersonated_user) = impersonated_user {
                                request = request.data(MyToken(token, impersonated_user));
                            } else {
                                request = request.data(MyToken(token, "".to_string()));
                            }
                        }
                        let resp = schema.execute(request).await;
                        Ok::<_, Infallible>(Response::from(resp))
                    },
                ),
        )
    }

    /// Graphql Playground for development.
    pub fn graphql_playground(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("api").and(warp::get()).map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/api"),
                ))
        })
    }

    /// Generate a CSV with all App Modeling apps.
    pub fn app_modeling_out_csv(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("app_modeling.csv")
            .and(warp::get())
            .and(Self::with_db(
                self.database_connections.sn_mirror_connection.clone(),
            ))
            .and_then(out_csv_app_modeling)
    }

    /// Generate a CSV with all DB Modeling databases.
    pub fn db_modeling_out_csv(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("db_modeling.csv")
            .and(warp::get())
            .and(Self::with_db(
                self.database_connections.sn_mirror_connection.clone(),
            ))
            .and_then(out_csv_db_modeling)
    }

    /// Run selenium Knowbe4 Job.  This is hit automatically on cron job from SiteScope.
    pub fn run_knowbe4_selenium_job(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("run_knowbe4_selenium_job")
            .and(warp::get())
            .and_then(knowbe4::run_program::run_knowbe4_templates_program)
    }

    /// Run QTrak user list file upload Job.  This is hit automatically on cron job from SiteScope.
    pub fn run_qtrak_job(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("run_qtrak_job")
            .and(warp::get())
            .and(Self::with_db(
                self.database_connections.sn_mirror_connection.clone(),
            ))
            .and(Self::with_db(
                self.database_connections.sn_mirror_connection.clone(),
            ))
            .and_then(QtrakUser::qtrak_warp_request)
    }

    /// Removes a CGI Node, AKA Remove a server from monitoring.  Service-now hits this daily.
    pub fn remove_cgi_node(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("remove_cgi_node"))
            .and(warp::body::json())
            .and_then(CgiNodeResult::remove_cgi_node_warp_request)
    }

    /// Allow CORS.
    pub fn cors(&self) -> Builder {
        warp::cors()
            .allow_any_origin()
            .allow_headers([
                "auth",
                "impersonatedUser",
                "accept",
                "sec-ch-ua",
                "sec-ch-ua-mobile",
                "User-Agent",
                "Sec-Fetch-Mode",
                "Referer",
                "Origin",
                "Access-Control-Request-Method",
                "Access-Control-Request-Headers",
                "content-type",
            ])
            .allow_methods(["OPTIONS", "GET", "POST", "DELETE", "PUT"])
    }

    pub async fn recover_api(_: Rejection) -> Result<impl Reply, Infallible> {
        Ok(warp::http::StatusCode::NOT_FOUND)
    }

    /// Allows database injection
    fn with_db(
        db_pool: Pool<TiberiusConnectionManager>,
    ) -> impl Filter<Extract = (Pool<TiberiusConnectionManager>,), Error = Infallible> + Clone {
        warp::any().map(move || db_pool.to_owned().clone())
    }

    /// All Routes combined
    pub fn routes(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Serve static React App.
        let react_app = warp::fs::dir("build");
        // Serve static React App.
        let other = warp::fs::file("build/index.html");
        // Logging for Warp
        let _warp_log = warp::log("info");
        //Route Builder
        react_app
            .or(graphql_subscription(self.schema.clone()))
            .or(self.remove_cgi_node())
            .or(self.app_modeling_out_csv())
            .or(self.db_modeling_out_csv())
            .or(self.run_knowbe4_selenium_job())
            .or(self.run_qtrak_job())
            .or(self.graphql_playground())
            .or(self.graphql_post())
            .or(other)
            .recover(Self::recover_api)
            .with(self.cors())
    }
}
