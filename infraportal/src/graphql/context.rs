use crate::db::TiberiusConnectionManager;
use bb8::Pool;
use reqwest::Client;
#[derive(Clone)]
pub struct GraphQLContext {
    pub rest_client: Client,
    pub rest_client_dangerous: Client,
    pub sn_mirror_connection: Pool<TiberiusConnectionManager>,
    pub infraportal_connection: Pool<TiberiusConnectionManager>,
    pub sccm_connection: Pool<TiberiusConnectionManager>,
    pub altiris_connection: Pool<TiberiusConnectionManager>,
    pub db_cmdb_connection: Pool<TiberiusConnectionManager>,
    pub blue_connection: Pool<TiberiusConnectionManager>,
    pub nb_disable_connection: Pool<TiberiusConnectionManager>,
    pub seceng_connection: Pool<TiberiusConnectionManager>,
}

pub struct MyToken(pub String, pub String);
