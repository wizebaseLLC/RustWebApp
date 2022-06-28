use crate::{file::write_new_csv, send_contacts::send_contacts_to_qtrak};
use dotenv::dotenv;
use std::env;

pub async fn run_qtrak(chicago_base64: String, london_base64: String) -> anyhow::Result<()> {
    match start_qtrak(chicago_base64, london_base64).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(target: "error", "{}", err.to_string());
            Err(anyhow::anyhow!(err))
        }
    }
}

async fn start_qtrak(chicago_base64: String, london_base64: String) -> anyhow::Result<()> {
    dotenv().ok();
    let client = reqwest::Client::new();
    // * There is a ORGID for both London and Chicago
    let org_id_chicago = env::var("ORGID_CHICAGO").expect("ORGID_CHICAGO must be set");
    let api_key_chicago =
        env::var("APIKEY_QTRAK_CHICAGO").expect("APIKEY_QTRAK_CHICAGO must be set");

    let org_id_london = env::var("ORGID_LONDON").expect("ORGID_LONDON must be set");
    let api_key_london = env::var("APIKEY_QTRAK_LONDON").expect("APIKEY_QTRAK_LONDON must be set");
    // Create file name
    let filename_chicago = format!("{}__contacts.csv", org_id_chicago);
    let filename_london = format!("{}__contacts.csv", org_id_london);

    // Write base64 to new csv file
    write_new_csv(chicago_base64, &filename_chicago).await?;
    write_new_csv(london_base64, &filename_london).await?;

    // Send data up to Qtrak
    send_contacts_to_qtrak(client.clone(), filename_chicago, api_key_chicago).await?;
    send_contacts_to_qtrak(client.clone(), filename_london, api_key_london).await?;
    Ok(())
}
