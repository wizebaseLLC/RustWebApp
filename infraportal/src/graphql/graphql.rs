use crate::{
    helpers::{
        ping::ping::{run_ssh_ping_many, PingResult},
        requests::graphql_translate::graphql_translate,
    },
    models::{
        app_modeling_field_queries::model::*,
        cmdb_ci_database::{
            model::{CmdbCiDatabase, DbModelingScoreCard},
            serde_types::serde_cmdb_ci_database::{
                CmdbCiDatabaseJson, MyDatabaseListFromServiceNow,
            },
        },
        cmdb_ci_hardware::model::{CmdbCiHostLookup, CmdbCiServer, CmdbCiServerParentApps},
        cmdb_ci_service::{
            model::{
                CmdbCiChanges, CmdbCiIncidents, CmdbCiSearchBar, CmdbCiService,
                CmdbCiServiceCertHistory, ModelingScoreCard,
            },
            serde_types::{
                serde_types_all_apps::RootServiceNowApps,
                serde_types_app_details::RootServiceNowAppDetails,
                serde_types_rel_ci::{RelCiTypes, RootServiceNowRelCi},
            },
        },
        knowbe4::model::{
            KnowBe4ReportedPhishing, KnowBe4Results, KnowBe4Test, KnowBe4Training, KnowBe4Users,
        },
        nb_disable::model::NbDisable,
        sccm_user_support::{
            model::*,
            serde_types::splunk_login_history::{LoginHistory, LoginHistoryBlankSlate},
        },
        sys_user::model::SysUser,
    },
};
use query_tiberius::v_sphere::{VsphereMetrics, VsphereMetricsInput, VsphereSeverProperties};

use super::{
    context::{GraphQLContext, MyToken},
    graphql_mutation::MutationRoot,
    graphql_subscription::SubscriptionRoot,
};

use async_graphql::{Context, FieldError, FieldResult, Object, Schema, ID};

pub type MySchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_world<'a>(&self) -> Option<&'a str> {
        Some("hello world")
    }
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
    async fn list_all_cmdb_ci_services(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<CmdbCiService>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbCiService::find_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cmdb_ci_service(&self, ctx: &Context<'_>, id: ID) -> FieldResult<CmdbCiService> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbCiService::find(&mut conn, id.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cbt_applications(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiService>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(
                CmdbCiService::find_cbt_applications(&mut conn, id.as_str()).await,
            ),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cbt_applications_service_now(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<RootServiceNowAppDetails> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(CmdbCiService::query_service_now_cbt_apps(conn, id.to_string()).await)
    }

    async fn query_service_now_for_matching_apps(
        &self,
        ctx: &Context<'_>,
        name: ID,
    ) -> FieldResult<RootServiceNowApps> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(
            CmdbCiService::query_service_now_for_matching_apps(conn, name.as_str().to_string())
                .await,
        )
    }

    async fn query_service_now_for_all_apps(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<RootServiceNowApps> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(CmdbCiService::query_service_now_for_all_apps(conn).await)
    }

    async fn query_service_now_for_my_apps(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<RootServiceNowApps> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                let client = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
                let my_token = ctx
                    .data_opt::<MyToken>()
                    .map(|token| (token.0.as_str(), token.1.as_str()));
                if let Some(my_token) = my_token {
                    graphql_translate(
                        CmdbCiService::query_service_now_for_my_apps(&mut conn, client, {
                            if my_token.1.len() > 0 {
                                my_token.1.to_string()
                            } else {
                                my_token.0.to_string()
                            }
                        })
                        .await,
                    )
                } else {
                    FieldResult::Err(FieldError::from("Not Authenticated"))
                }
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_service_now_app_details(
        &self,
        ctx: &Context<'_>,
        sys_id: ID,
    ) -> FieldResult<RootServiceNowAppDetails> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(
            CmdbCiService::query_service_now_app_details(conn, sys_id.as_str().to_string()).await,
        )
    }

    async fn query_service_now_rel_ci(
        &self,
        ctx: &Context<'_>,
        sys_id: ID,
        ci_type: RelCiTypes,
    ) -> FieldResult<RootServiceNowRelCi> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(
            CmdbCiService::query_service_now_rel_ci(conn, sys_id.as_str().to_string(), ci_type)
                .await,
        )
    }

    async fn get_sso_partners(&self, ctx: &Context<'_>) -> FieldResult<Vec<SsoPartners>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SsoPartners::find(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_vsts_projects(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<VstsProjects>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(VstsProjects::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_users(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<GetMoreUsers>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(GetMoreUsers::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_dfs_links(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<DfsLinks>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(DfsLinks::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_vips(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<Vips>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(Vips::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_vendors(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<Vendors>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(Vendors::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_six_rs(&self, ctx: &Context<'_>) -> FieldResult<Vec<SixRs>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SixRs::find(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_service_accounts(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<ServiceAccounts>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(ServiceAccounts::find(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_servers(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<GetMoreServers>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(GetMoreServers::find(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_many_more_servers(
        &self,
        ctx: &Context<'_>,
        ids: Vec<String>,
    ) -> FieldResult<Vec<GetMoreServers>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                let ids = ids.as_slice();
                graphql_translate(GetMoreServers::find_many(&mut conn, ids).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_databases(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<GetMoreDatabases>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(GetMoreDatabases::find(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_apps(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<GetMoreApps>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(GetMoreApps::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_app_instances(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<GetMoreApps>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(GetMoreApps::find_instance(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_cmdb_models(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<CmdbModels>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbModels::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_distribution_lists(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<DistributionLists>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(DistributionLists::find(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_active_directory_groups(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<ActiveDirectoryGroups>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(ActiveDirectoryGroups::find(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_app_admin(&self, ctx: &Context<'_>, query: ID) -> FieldResult<Vec<AppAdmin>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(AppAdmin::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_sys_choice_list(
        &self,
        ctx: &Context<'_>,
        choice: SysChoiceListOptions,
    ) -> FieldResult<Vec<SysChoiceList>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SysChoiceList::find(&mut conn, choice).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_user_as_me(&self, ctx: &Context<'_>) -> FieldResult<SysUser> {
        let user_name = ctx
            .data_opt::<MyToken>()
            .map(|token| (token.0.as_str(), token.1.as_str()))
            .expect("Not logged in");

        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SysUser::find_me(&mut conn, user_name).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_user(&self, ctx: &Context<'_>, user_name: ID) -> FieldResult<SysUser> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SysUser::find(&mut conn, user_name.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<SysUser>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SysUser::find_many(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_managers_from_business_head(
        &self,
        ctx: &Context<'_>,
        display_name: ID,
    ) -> FieldResult<Vec<SysUser>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(
                SysUser::find_manager_from_business_head(&mut conn, display_name.as_str()).await,
            ),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_application_cert_history(
        &self,
        ctx: &Context<'_>,
        app_name: ID,
    ) -> FieldResult<CmdbCiServiceCertHistory> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(
                CmdbCiServiceCertHistory::find(&mut conn, app_name.as_str()).await,
            ),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_application_cert_history(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<CmdbCiServiceCertHistory>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbCiServiceCertHistory::find_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_application_scorecard_data(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<ModelingScoreCard>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(ModelingScoreCard::find_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_all_cmdb_ci_databases(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<CmdbCiDatabase>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbCiDatabase::find_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cmdb_ci_database(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<CmdbCiDatabase> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(CmdbCiDatabase::find(&mut conn, id.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cmdb_ci_database_from_service_now(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<CmdbCiDatabaseJson> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(CmdbCiDatabase::find_from_service_now(conn, id.as_str()).await)
    }

    async fn list_my_databases_from_service_now(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<MyDatabaseListFromServiceNow> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        let my_token = ctx
            .data_opt::<MyToken>()
            .map(|token| (token.0.as_str(), token.1.as_str()));

        if let Some(my_token) = my_token {
            graphql_translate(
                CmdbCiDatabase::list_my_databases_from_service_now(conn, {
                    if my_token.1.len() > 0 {
                        my_token.1
                    } else {
                        my_token.0
                    }
                })
                .await,
            )
        } else {
            FieldResult::Err(FieldError::from("Not Authenticated"))
        }
    }

    async fn list_all_databases_from_service_now(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<MyDatabaseListFromServiceNow> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(CmdbCiDatabase::list_all_databases_from_service_now(conn).await)
    }

    async fn list_all_cmdb_ci_database_scorecard_data(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<DbModelingScoreCard>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(DbModelingScoreCard::find_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_more_pm_projects(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<PmProject>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(PmProject::find(&mut conn, query.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_sys_dictionary(
        &self,
        ctx: &Context<'_>,
        choice: SysChoiceListOptions,
    ) -> FieldResult<Vec<SysDictionary>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SysDictionary::find_all(&mut conn, choice).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_service_now_for_matching_certificates(
        &self,
        ctx: &Context<'_>,
        query: ID,
    ) -> FieldResult<Vec<CmdbCiCertificate>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(CmdbCiCertificate::find_matching(&mut conn, query.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sccm_pc_hardware(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<SccmPcHardware> {
        let conn = ctx.data::<GraphQLContext>()?.sccm_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SccmPcHardware::find(&mut conn, id.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sccm_pc_installed_software(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<SccmPcInstalledSoftware>> {
        let conn = ctx.data::<GraphQLContext>()?.sccm_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(SccmPcInstalledSoftware::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sccm_pc_logged_in_user(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<SccmPcLoggedInUser> {
        let conn = ctx.data::<GraphQLContext>()?.sccm_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(SccmPcLoggedInUser::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sccm_user_primary_pc(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<SccmUserPrimaryPc> {
        let conn = ctx.data::<GraphQLContext>()?.sccm_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(SccmUserPrimaryPc::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sccm_user_profile(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<SccmUserProfile> {
        let conn = ctx.data::<GraphQLContext>()?.sccm_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(SccmUserProfile::find(&mut conn, id.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_altiris_user_drives(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<AltirisUserDrives>> {
        let conn = ctx.data::<GraphQLContext>()?.altiris_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(AltirisUserDrives::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_altiris_conference_room_desktops(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<AltirisUserConferenceRoomDesktops>> {
        let conn = ctx.data::<GraphQLContext>()?.altiris_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(
                AltirisUserConferenceRoomDesktops::find(&mut conn, id.as_str()).await,
            ),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_sn_mirror_user_tasks(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<SnMirrorUserTasks>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(SnMirrorUserTasks::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_cmdb_ci_incidents_by_ci(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiIncidents>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(CmdbCiService::find_incidents(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_cmdb_ci_incidents_by_opened_by(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiIncidents>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(
                CmdbCiService::find_incidents_by_opened_by(&mut conn, id.as_str()).await,
            ),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_cmdb_ci_changes_by_ci(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiChanges>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(CmdbCiService::find_changes(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cmdb_ci_servers_by_id_or_name(
        &self,
        ctx: &Context<'_>,
        ids: Vec<String>,
    ) -> FieldResult<Vec<CmdbCiServer>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                let ids = ids.as_slice();
                graphql_translate(CmdbCiServer::find_many(&mut conn, ids).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn find_cmdb_ci_server_parent_apps(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiServerParentApps>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(CmdbCiServerParentApps::find(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_vsphere_metrics(
        &self,
        ctx: &Context<'_>,
        input: VsphereMetricsInput,
    ) -> FieldResult<VsphereMetrics> {
        let conn = ctx
            .data::<GraphQLContext>()
            .unwrap()
            .rest_client_dangerous
            .clone();
        graphql_translate(CmdbCiServer::query_vsphere_metrics(conn, input).await)
    }

    async fn query_vsphere_server_properties(
        &self,
        ctx: &Context<'_>,
        resource_id: String,
    ) -> FieldResult<VsphereSeverProperties> {
        let conn = ctx
            .data::<GraphQLContext>()
            .unwrap()
            .rest_client_dangerous
            .clone();
        graphql_translate(CmdbCiServer::query_vsphere_server_properties(conn, resource_id).await)
    }

    async fn find_cmdb_ci_host_lookup(
        &self,
        ctx: &Context<'_>,
        ids: Vec<String>,
    ) -> FieldResult<Vec<CmdbCiHostLookup>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                let ids = ids.as_slice();
                graphql_translate(CmdbCiHostLookup::find_many(&mut conn, ids).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_cmdb_ci_searchbar(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<CmdbCiSearchBar>> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(CmdbCiSearchBar::find_many(&mut conn, id.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_blue_relations(
        &self,
        ctx: &Context<'_>,
        app_name: ID,
    ) -> FieldResult<Vec<BlueRelations>> {
        let conn = ctx.data::<GraphQLContext>()?.blue_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                graphql_translate(BlueRelations::find_matching(&mut conn, app_name.as_str()).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn query_user_login_history(
        &self,
        ctx: &Context<'_>,
        user_name: ID,
    ) -> FieldResult<LoginHistory> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(
            LoginHistoryBlankSlate::query_user_login_history(conn, user_name.to_string()).await,
        )
    }

    async fn query_desktop_login_history(
        &self,
        ctx: &Context<'_>,
        desktop_name: ID,
    ) -> FieldResult<LoginHistory> {
        let conn = ctx.data::<GraphQLContext>().unwrap().rest_client.clone();
        graphql_translate(
            LoginHistoryBlankSlate::query_desktop_login_history(conn, desktop_name.to_string())
                .await,
        )
    }

    async fn query_nb_disable(
        &self,
        ctx: &Context<'_>,
        host_name: ID,
    ) -> FieldResult<Vec<NbDisable>> {
        let conn = ctx.data::<GraphQLContext>()?.nb_disable_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(NbDisable::find(&mut conn, host_name.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_nb_disable(&self, ctx: &Context<'_>) -> FieldResult<Vec<NbDisable>> {
        let conn = ctx.data::<GraphQLContext>()?.nb_disable_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(NbDisable::list_all(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_recipients(
        &self,
        ctx: &Context<'_>,
        ids: Vec<ID>,
    ) -> FieldResult<Vec<KnowBe4Test>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => {
                let ids = ids.iter().map(|id| id.as_str()).collect::<Vec<&str>>();

                if ids.len() != 6 {
                    return FieldResult::Err(FieldError::from(
                        "Must have 6 ids in the 'ids' array",
                    ));
                }
                graphql_translate(KnowBe4Test::find(&mut conn, ids).await)
            }
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_security_tests(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<KnowBe4Test>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4Test::list(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_results(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> FieldResult<Vec<KnowBe4Results>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4Results::find(&mut conn, id.as_str()).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn list_know_be4_results(&self, ctx: &Context<'_>) -> FieldResult<Vec<KnowBe4Results>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4Results::list(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_security_training(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<KnowBe4Training>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4Training::list(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_reported_phishing(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<KnowBe4ReportedPhishing>> {
        let conn = ctx.data::<GraphQLContext>()?.seceng_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4ReportedPhishing::list(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn get_know_be4_users_who_work_for_me(
        &self,
        ctx: &Context<'_>,
        manager_email: ID,
    ) -> FieldResult<Vec<KnowBe4Users>> {
        graphql_translate(KnowBe4Users::find_by_manager_email(ctx, manager_email.as_str()).await)
    }

    async fn get_know_be4_users_who_work_for_us(
        &self,
        ctx: &Context<'_>,
        manager_emails: Vec<ID>,
    ) -> FieldResult<Vec<KnowBe4Users>> {
        graphql_translate(
            KnowBe4Users::find_users_that_works_for_us(
                ctx,
                manager_emails.iter().map(|i| i.as_str()).collect(),
            )
            .await,
        )
    }

    async fn list_know_be4_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<KnowBe4Users>> {
        let conn = ctx.data::<GraphQLContext>()?.infraportal_connection.clone();
        let conn = conn.get().await;
        match conn {
            Ok(mut conn) => graphql_translate(KnowBe4Users::list(&mut conn).await),
            Err(err) => {
                log::error!(target: "error", "{}", err.to_string());
                FieldResult::Err(FieldError::from(err))
            }
        }
    }

    async fn ping_host_many(
        &self,
        _ctx: &Context<'_>,
        hosts: Vec<String>,
    ) -> FieldResult<Vec<PingResult>> {
        let results = run_ssh_ping_many(hosts.as_slice()).await;
        graphql_translate(results)
    }
}
