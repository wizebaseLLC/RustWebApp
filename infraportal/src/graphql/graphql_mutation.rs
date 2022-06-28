use async_graphql::{Context, FieldError, FieldResult, Object, ID};

use crate::{
    helpers::requests::graphql_translate::graphql_translate,
    models::{
        cmdb_ci_database::{model::CmdbCiDatabase, serde_types::submit_db_form::DbModelingSubmit},
        cmdb_ci_service::{
            model::{AppSubmitted, CmdbCiService, CmdbCiServiceCertHistory},
            serde_types::{
                serde_types_cert_history::ApplicationCertHistoryInput,
                serde_types_submit_app_form::{AppFormReturnValues, AppModelingForm},
            },
        },
        nb_disable::model::{NbDisable, NbDisableEnableInput, NbDisableInput},
    },
};

use super::{
    context::{GraphQLContext, MyToken},
    graphql_subscription::MutationType,
    simple_broker::SimpleBroker,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn hello_world<'a>(&self) -> Option<&'a str> {
        Some("hello world")
    }

    async fn submit_app_modeling(
        &self,
        ctx: &Context<'_>,
        values: String,
    ) -> FieldResult<AppFormReturnValues> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        let my_token = ctx.data_opt::<MyToken>().map(|token| token.0.as_str());

        if let Some(my_token) = my_token {
            let values: serde_json::error::Result<AppModelingForm> =
                serde_json::from_str(values.as_str());
            match values {
                Ok(values) => {
                    SimpleBroker::publish(AppSubmitted {
                        mutation_type: MutationType::Created,
                        name: values.name.clone(),
                        submitted_by: my_token.to_string(),
                    });

                    graphql_translate(
                        CmdbCiService::submit_app_modeling_form(conn, values, my_token.to_string())
                            .await,
                    )
                }
                Err(err) => {
                    log::error!(target: "error", "{} - {}", my_token, err.to_string());
                    FieldResult::Err(FieldError::from(err))
                }
            }
        } else {
            log::error!(target: "error", "User not Authenticated");
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }

    async fn submit_db_modeling(
        &self,
        ctx: &Context<'_>,
        sys_id: ID,
        values: String,
    ) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        let my_token = ctx.data_opt::<MyToken>().map(|token| token.0.as_str());

        if let Some(my_token) = my_token {
            let values: serde_json::error::Result<DbModelingSubmit> =
                serde_json::from_str(values.as_str());

            match values {
                Ok(values) => {
                    let returned_value =
                        CmdbCiDatabase::submit_db_form(conn, values, sys_id.to_string()).await;
                    match returned_value {
                        Ok(_) => Ok(true),
                        Err(err) => {
                            log::error!(target: "error", "{} - {}", my_token, err.to_string());
                            Ok(false)
                        }
                    }
                }
                Err(err) => {
                    log::error!(target: "error", "{} - {}", my_token, err.to_string());
                    FieldResult::Err(FieldError::from(err))
                }
            }
        } else {
            log::error!(target: "error", "User not Authenticated");
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }

    async fn submit_app_cert_history(
        &self,
        ctx: &Context<'_>,
        app_name: ID,
        create_date: String,
        data: String,
        cert_history: Vec<ApplicationCertHistoryInput>,
    ) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let mut conn = conn.get().await?;
        let my_token = ctx.data_opt::<MyToken>().map(|token| token.0.as_str());

        if let Some(_my_token) = my_token {
            let cert_history = serde_json::to_string(&cert_history)?;

            CmdbCiServiceCertHistory::modify_history(
                &mut conn,
                app_name.as_str(),
                create_date.as_str(),
                data.as_str(),
                cert_history.as_str(),
            )
            .await?;

            Ok(true)
        } else {
            log::error!(target: "error", "User not Authenticated");
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }

    async fn submit_nb_disable(
        &self,
        ctx: &Context<'_>,
        values: NbDisableInput,
    ) -> FieldResult<String> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        let my_token = ctx.data_opt::<MyToken>().map(|token| token.0.as_str());

        if let Some(_my_token) = my_token {
            graphql_translate(NbDisable::post_nb_disable(conn, values).await)
        } else {
            log::error!(target: "error", "User not Authenticated");
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }

    async fn submit_nb_enable(
        &self,
        ctx: &Context<'_>,
        values: NbDisableEnableInput,
    ) -> FieldResult<String> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        let my_token = ctx.data_opt::<MyToken>().map(|token| token.0.as_str());

        if let Some(_my_token) = my_token {
            graphql_translate(NbDisable::post_nb_enable(conn, values).await)
        } else {
            log::error!(target: "error", "User not Authenticated");
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }
}
