use async_trait::async_trait;
use bb8::Pool;
use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

#[derive(Clone, Debug)]
pub struct TiberiusConnectionManager {
    config: Config,
}

impl TiberiusConnectionManager {
    /// Create a new `TiberiusConnectionManager`.
    pub fn new(config: Config) -> tiberius::Result<TiberiusConnectionManager> {
        Ok(TiberiusConnectionManager { config })
    }
}

#[async_trait]
impl bb8::ManageConnection for TiberiusConnectionManager {
    type Connection = Client<Compat<TcpStream>>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(&self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        Client::connect(self.config.clone(), tcp.compat_write()).await
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        //debug!("Checking {:?}", conn);
        conn.simple_query("").await?.into_row().await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

pub async fn create_connection(
    host: &str,
    database: &str,
) -> anyhow::Result<Pool<TiberiusConnectionManager>> {
    let mut config = Config::new();
    config.host(host);
    config.database(database);
    config.authentication(AuthMethod::Integrated);
    config.trust_cert();
    let manager = TiberiusConnectionManager::new(config).unwrap();
    let pool = bb8::Pool::builder().min_idle(Some(2)).build(manager).await;
    pool.map_err(Into::into)
}
