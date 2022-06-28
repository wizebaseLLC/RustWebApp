use dotenv::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use std::env;

use crate::selenium::run_selenium::Cookies;

pub async fn query_knowbe4<R, T>(
    client: reqwest::Client,
    body: &T,
    cookie: &Cookies,
) -> anyhow::Result<R>
where
    R: DeserializeOwned,
    T: Serialize + Send + Sync + std::fmt::Debug,
{
    dotenv().ok();
    let bearer_token = env::var("KNOWBE4_API_TOKEN").expect("KNOWBE4_API_TOKEN must be set");
    let cookie = format!("{}={}", cookie.name, cookie.value);

    client
        .post("https://training.knowbe4.com/graphql")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Cookie", cookie)
        .bearer_auth(bearer_token)
        .json(body)
        .send()
        .await?
        .json()
        .await
        .map_err(Into::into)
}
