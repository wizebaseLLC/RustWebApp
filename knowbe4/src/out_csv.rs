use crate::attributes::AttributesWithId;
use chrono::Utc;
use csv::Writer;
use serde::Serialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

fn to_csv(data: &[impl Serialize]) -> anyhow::Result<String> {
    let mut wtr = Writer::from_writer(vec![]);

    for row in data.iter() {
        wtr.serialize(row).unwrap();
    }

    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}

pub async fn out_csv_app_modeling(list: &[AttributesWithId]) -> anyhow::Result<()> {
    let time_generated = Utc::now().to_string();
    let time_generated = &time_generated[0..21];
    let time_generated = format!(
        "knowbe4\\{}.csv",
        time_generated.replace(" ", "-").replace(":", "-")
    );
    println!(
        "Generating a csv with the data prior to updating. - {}",
        time_generated
    );
    let csv = to_csv(list).expect("invalid csv format");
    let mut file = File::create(time_generated.as_str()).await?;

    file.write_all(csv.as_bytes()).await?;

    Ok(())
}
