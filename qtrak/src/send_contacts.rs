use dotenv::dotenv;
use reqwest::Body;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn send_contacts_to_qtrak(
    client: reqwest::Client,
    filename: String,
    api_key: String,
) -> anyhow::Result<()> {
    dotenv().ok();
    let url = &format!("https://5026ab7ub6.execute-api.us-east-2.amazonaws.com/prod/contact/uploadfile?fileName={}",filename);
    let file = File::open(filename).await?;

    let response = client
        .post(url)
        .body(file_to_body(file))
        .header("ContentEncoding", "base64")
        .header("apikey", api_key)
        .send()
        .await?;

    println!("{}", response.status());
    Ok(())
}

fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}
