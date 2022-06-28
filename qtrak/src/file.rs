use base64::encode;
use tokio::io::*;

pub async fn open_file<'a>(filename: &'a str) -> anyhow::Result<String> {
    // Open file
    let mut file = tokio::fs::File::open(filename).await?;
    let mut contents = String::new();

    // Push contents of the file into the contents variable
    file.read_to_string(&mut contents).await?;

    // Convert to byes
    let contents = contents.as_bytes();

    //Encode as base64
    let contents = encode(contents);

    Ok(contents)
}

pub async fn write_new_csv<'a>(base64: String, filename: &'a str) -> anyhow::Result<()> {
    let mut file = tokio::fs::File::create(filename).await?;
    file.write_all(base64.as_bytes()).await?;
    Ok(())
}
