use super::{constants::*, model::*};
use crate::{db::TiberiusConnectionManager, helpers::requests::struct_to_csv::to_csv};
use base64::encode;
use bb8::{Pool, PooledConnection};
use knowbe4::run_program::WarpResult;
use qtrak::run_qtrak::run_qtrak;
use query_tiberius::query_tiberius::*;
use warp::Reply;

/// Used to query service now for qtrak contacts, then send that data up to the vendor portal.
impl QtrakUser {
    pub async fn list_all<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        location: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_QTRAK_LIST_USERS, &[&location]).await?;
        Self::build_result_many(stream).await
    }

    pub fn convert_to_base64(contents: &[u8]) -> anyhow::Result<String> {
        //Encode as base64
        let contents = encode(contents);

        Ok(contents)
    }

    pub async fn send_contacts_to_qtrak<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        conn2: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<()> {
        let (london_contacts, chicago_contacts) = futures::future::try_join(
            Self::list_all(conn, "london"),
            Self::list_all(conn2, "chicago"),
        )
        .await?;

        let london_contacts = to_csv(london_contacts)?;
        let chicago_contacts = to_csv(chicago_contacts)?;

        // Convert results to base64
        let chicago_contacts = Self::convert_to_base64(chicago_contacts.as_bytes())?;
        let london_contacts = Self::convert_to_base64(london_contacts.as_bytes())?;

        run_qtrak(chicago_contacts, london_contacts).await
    }

    pub async fn qtrak_warp_request(
        conn: Pool<TiberiusConnectionManager>,
        conn2: Pool<TiberiusConnectionManager>,
    ) -> WarpResult<impl Reply> {
        let mut conn = conn.get().await.unwrap();
        let mut conn2 = conn2.get().await.unwrap();
        Self::send_contacts_to_qtrak(&mut conn, &mut conn2)
            .await
            .map_err(|_| warp::reject())?;
        Ok("job complete")
    }
}
