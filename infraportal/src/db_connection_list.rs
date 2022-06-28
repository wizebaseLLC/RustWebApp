use crate::{
    db::{create_connection, TiberiusConnectionManager},
    helpers::requests::service_now_url::is_prod,
};
use bb8::Pool;

/// Database Connections
#[derive(Clone)]
pub struct DatabaseConnections {
    pub sn_mirror_connection: Pool<TiberiusConnectionManager>,
    pub infraportal_connection: Pool<TiberiusConnectionManager>,
    pub sccm_connection: Pool<TiberiusConnectionManager>,
    pub altiris_connection: Pool<TiberiusConnectionManager>,
    pub db_cmdb_connection: Pool<TiberiusConnectionManager>,
    pub blue_connection: Pool<TiberiusConnectionManager>,
    pub nb_disable_connection: Pool<TiberiusConnectionManager>,
    pub seceng_connection: Pool<TiberiusConnectionManager>,
}

impl DatabaseConnections {
    /// Generates the database connection pools
    pub async fn new() -> anyhow::Result<Self> {
        let is_prod = is_prod();
        let sn_mirror_connection = create_connection(
            {
                if is_prod {
                    "pdb1"
                } else {
                    "itsmdb"
                }
            },
            "sn_mirror",
        );

        let infraportal_connection = create_connection(
            {
                if is_prod {
                    "pinfra"
                } else {
                    "dinfra"
                }
            },
            "infraportal",
        );
        let nb_disable_connection = create_connection(
            {
                if is_prod {
                    "pdb1"
                } else {
                    "ditsm"
                }
            },
            "ITSMFMS",
        );
        let sccm_connection = create_connection("server1", "CM_NBP");
        let altiris_connection = create_connection("server2", "AltirisNotification_Prod");
        let db_cmdb_connection = create_connection("server3", "dbcmdb");
        let blue_connection = create_connection("server4", "bluedb");
        let seceng_connection = create_connection("server5", "seceng");

        // Start each connection at the same time.
        let database_connections = tokio::try_join!(
            sn_mirror_connection,
            infraportal_connection,
            sccm_connection,
            altiris_connection,
            db_cmdb_connection,
            blue_connection,
            nb_disable_connection,
            seceng_connection
        )?;

        Ok(Self {
            sn_mirror_connection: database_connections.0,
            infraportal_connection: database_connections.1,
            sccm_connection: database_connections.2,
            altiris_connection: database_connections.3,
            db_cmdb_connection: database_connections.4,
            blue_connection: database_connections.5,
            nb_disable_connection: database_connections.6,
            seceng_connection: database_connections.7,
        })
    }
}
