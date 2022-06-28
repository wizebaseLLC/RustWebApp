use async_graphql::SimpleObject;
use dns_lookup::lookup_host;
use dotenv::dotenv;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::IpAddr;
use winping::{Buffer, Pinger};

use crate::helpers::windows_cmd_command::windows_cmd_command::run_windows_command;

pub async fn get_ip_from_hostname(hostname: String) -> Option<String> {
    let result = tokio::spawn(async move {
        let ips = lookup_host(hostname.as_str()).ok();
        if let Some(ips) = ips {
            Some(ips[0].to_string())
        } else {
            None
        }
    });

    match result.await {
        Ok(data) => data,
        Err(_) => None,
    }
}

pub async fn ping_host(hostname: String) -> Option<bool> {
    let result = tokio::spawn(async move {
        let ip = get_ip_from_hostname(hostname).await?;
        let dst = std::env::args()
            .nth(1)
            .unwrap_or(String::from(ip))
            .parse::<IpAddr>()
            .ok()?;

        let pinger = Pinger::new().expect("failed to create Pinger builder");
        let mut buffer = Buffer::new();

        match pinger.send(dst, &mut buffer) {
            Ok(_) => Some(true),
            Err(_) => Some(false),
        }
    });

    match result.await {
        Ok(data) => data,
        Err(_) => Some(false),
    }
}

pub async fn run_ssh_ping(hostname: &str) -> anyhow::Result<bool> {
    dotenv().ok();
    let username = env::var("SVC_ACCOUNT").expect("SVC_ACCOUNT must be set");
    let password = env::var("SVC_ACCOUNT_PASS").expect("SVC_ACCOUNT_PASS must be set");
    let output = run_windows_command(
        "plink",
        vec![
            "-ssh",
            format!("{}@twplits201", username).as_str(),
            "-pw",
            password.as_str(),
            "ping",
            hostname,
            "-c",
            "1",
        ],
    )
    .await?;

    Ok(output.contains("1 received, 0% packet loss"))
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct PingResult {
    pub name: String,
    pub result: bool,
}

impl PingResult {
    pub fn new(name: String, result: bool) -> Self {
        Self { name, result }
    }
}
pub async fn run_ssh_ping_many(hosts: &[String]) -> anyhow::Result<Vec<PingResult>> {
    dotenv().ok();
    let username = env::var("SVC_ACCOUNT").expect("SVC_ACCOUNT must be set");
    let password = env::var("SVC_ACCOUNT_PASS").expect("SVC_ACCOUNT_PASS must be set");
    let server = format!("{}@twplits201", username);
    let mut futures = Vec::new();

    for hostname in hosts {
        let args = vec![
            "-ssh",
            server.as_str(),
            "-pw",
            password.as_str(),
            "ping",
            hostname,
            "-c",
            "1",
        ];

        futures.push(run_windows_command("plink", args));
    }

    let results = join_all(futures).await;

    let results = results
        .into_iter()
        .enumerate()
        .map(|(index, output)| {
            if let Ok(output) = output {
                PingResult::new(
                    hosts[index].clone(),
                    output.contains("1 received, 0% packet loss"),
                )
            } else {
                PingResult::new(hosts[index].clone(), false)
            }
        })
        .collect::<Vec<PingResult>>();

    Ok(results)
}
