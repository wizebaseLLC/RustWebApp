use async_trait::async_trait;
use dotenv::dotenv;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::env;
use tiberius::{QueryResult, Row};

use crate::v_sphere::{VsphereMetricsQueryBody, VsphereSeverPropertiesInput};

use super::v_sphere::{
    Token, VsphereCredentials, VsphereMetrics, VsphereMetricsInput, VsphereNameField,
    VsphereResourceId, VsphereSeverProperties,
};

#[async_trait]
pub trait QueryTiberius {
    type Output: Send + Sync;

    ///Converts the tiberius results to the type T, from within the .map Method.
    fn convert_from_sql(row: &Row) -> Self::Output;

    ///Creates results as Vec< T > from tiberius results
    async fn build_result_many<'a>(stream: QueryResult<'a>) -> anyhow::Result<Vec<Self::Output>> {
        let result = stream.into_results().await?;
        if !result.is_empty() {
            let result = result[0]
                .as_slice()
                .into_par_iter()
                .map(Self::convert_from_sql)
                .collect::<Vec<Self::Output>>();

            Ok::<_, anyhow::Error>(result).map_err(Into::into)
        } else {
            drop(result);
            Ok::<_, anyhow::Error>(Vec::<Self::Output>::new()).map_err(Into::into)
        }
    }

    ///Creates results as T from tiberius results   
    async fn build_result<'a>(stream: QueryResult<'a>) -> anyhow::Result<Self::Output> {
        let result = stream.into_results().await?;
        if result[0].is_empty() {
            return Err(anyhow::anyhow!("No results found"));
        }
        let result = &result[0][0];
        let result = Self::convert_from_sql(result);

        Ok::<_, anyhow::Error>(result).map_err(Into::into)
    }

    async fn query_splunk<T>(client: reqwest::Client, url: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        dotenv().ok();
        let username = env::var("SVC_ACCOUNT").expect("SVC_ACCOUNT must be set");
        let password = env::var("SVC_ACCOUNT_PASS").expect("SVC_ACCOUNT_PASS must be set");
        client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn query_service_now_get_builder<T>(
        client: reqwest::Client,
        url: &str,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        dotenv().ok();
        let username = env::var("SN_API_KEY_USER").expect("SN_API_KEY_USER must be set");
        let password = env::var("SN_API_KEY_PASS").expect("SN_API_KEY_PASS must be set");
        client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Used to test the response of a service now request
    async fn query_service_now_get_builder_text_only(
        client: reqwest::Client,
        url: &str,
    ) -> anyhow::Result<String> {
        dotenv().ok();
        let username = env::var("SN_API_KEY_USER").expect("SN_API_KEY_USER must be set");
        let password = env::var("SN_API_KEY_PASS").expect("SN_API_KEY_PASS must be set");
        println!("user {:#?}", username);
        println!("pass {:#?}", password);
        client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .await?
            .text()
            .await
            .map_err(Into::into)
    }

    async fn service_now_post_builder<T, B>(
        client: reqwest::Client,
        url: &str,
        body: &B,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize + Send + Sync + std::fmt::Debug,
    {
        dotenv().ok();
        let username = env::var("SN_API_KEY_USER").expect("SN_API_KEY_USER must be set");
        let password = env::var("SN_API_KEY_PASS").expect("SN_API_KEY_PASS must be set");
        client
            .post(url)
            .basic_auth(username, Some(password))
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn service_now_put_builder<T, B>(
        client: reqwest::Client,
        url: &str,
        body: &B,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize + Send + Sync + std::fmt::Debug,
    {
        dotenv().ok();
        let username = env::var("SN_API_KEY_USER").expect("SN_API_KEY_USER must be set");
        let password = env::var("SN_API_KEY_PASS").expect("SN_API_KEY_PASS must be set");
        client
            .put(url)
            .basic_auth(username, Some(password))
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn generate_v_sphere_token(client: reqwest::Client) -> anyhow::Result<Token> {
        dotenv().ok();
        let username = env::var("SVC_ACCOUNT").expect("SVC_ACCOUNT must be set");
        let password = env::var("SVC_ACCOUNT_PASS").expect("SVC_ACCOUNT_PASS must be set");
        let credentials = VsphereCredentials::new(username, password);

        client
            .post("https://vcops.nb.com/suite-api/api/auth/token/acquire")
            .header("Accept", "application/json")
            .json(&credentials)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn get_vsphere_resource_identifier(
        client: reqwest::Client,
        host: &str,
    ) -> anyhow::Result<VsphereResourceId> {
        let credentials = Self::generate_v_sphere_token(client.clone()).await?;

        let host = VsphereNameField::new(host);
        client
            .post("https://vcops.nb.com/suite-api/api/resources/query")
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!("vRealizeOpsToken {}", credentials.token).as_str(),
            )
            .json(&host)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn get_vsphere_metrics(
        client: reqwest::Client,
        input: VsphereMetricsInput,
    ) -> anyhow::Result<VsphereMetrics> {
        let credentials = Self::generate_v_sphere_token(client.clone()).await?;
        let identifier = Self::get_vsphere_resource_identifier(client.clone(), &input.resource_id)
            .await?
            .resource_list;

        let identifier = identifier[0].identifier.clone().unwrap();
        let input = VsphereMetricsQueryBody::new(input, identifier);

        client
            .post("https://vcops.nb.com/suite-api/api/resources/stats/query")
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!("vRealizeOpsToken {}", credentials.token).as_str(),
            )
            .json(&input)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn get_vsphere_server_properties(
        client: reqwest::Client,
        resource_id: String,
    ) -> anyhow::Result<VsphereSeverProperties> {
        let credentials = Self::generate_v_sphere_token(client.clone()).await?;

        let identifier = Self::get_vsphere_resource_identifier(client.clone(), &resource_id)
            .await?
            .resource_list;

        let identifier = identifier[0].identifier.clone().unwrap();
        let input = VsphereSeverPropertiesInput::new(identifier);

        client
            .post("https://vcops.nb.com/suite-api/api/resources/properties/latest/query")
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!("vRealizeOpsToken {}", credentials.token).as_str(),
            )
            .json(&input)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn post_to_itsm_portal<B>(
        client: reqwest::Client,
        url: &str,
        body: &B,
    ) -> anyhow::Result<String>
    where
        B: serde::Serialize + Send + Sync + std::fmt::Debug,
    {
        client
            .post(url)
            .header("Content-Type", "application/xml")
            .form(body)
            .send()
            .await?
            .text()
            .await
            .map_err(Into::into)
    }
}
