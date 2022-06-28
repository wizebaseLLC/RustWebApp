use crate::{
    db::TiberiusConnectionManager,
    models::{cmdb_ci_database::model::CmdbCiDatabase, cmdb_ci_service::model::CmdbCiService},
};
use bb8::Pool;
use csv::Writer;
use serde::Serialize;
use warp::{http::Response as HttpResponse, Reply};

pub fn to_csv(data: Vec<impl Serialize>) -> anyhow::Result<String> {
    let mut wtr = Writer::from_writer(vec![]);

    for row in data.into_iter() {
        wtr.serialize(row).unwrap();
    }

    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}

type WarpResult<T> = std::result::Result<T, warp::Rejection>;

pub async fn out_csv_app_modeling(conn: Pool<TiberiusConnectionManager>) -> WarpResult<impl Reply> {
    let mut conn = conn.get().await.unwrap();
    let data = CmdbCiService::find_all(&mut conn).await.unwrap();
    let csv = to_csv(data).expect("invalid csv format");

    let csv = HttpResponse::builder()
        .header("Content-Disposition", "attachment")
        .body(csv)
        .expect("Failed to Parse app modeling csv");

    Ok(csv)
}

pub async fn out_csv_db_modeling(conn: Pool<TiberiusConnectionManager>) -> WarpResult<impl Reply> {
    let mut conn = conn.get().await.unwrap();
    let data = CmdbCiDatabase::find_all(&mut conn).await.unwrap();
    let csv = to_csv(data).expect("invalid csv format");

    let csv = HttpResponse::builder()
        .header("Content-Disposition", "attachment")
        .body(csv)
        .expect("Failed to Parse db modeling csv");

    Ok(csv)
}
