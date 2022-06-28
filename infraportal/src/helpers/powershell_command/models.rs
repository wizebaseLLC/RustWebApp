use super::{powershell_command::run_powershell_command, scripts::remove_cgi_node};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CgiNodeResult {
    pub results: Option<String>,
    pub errors: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CgiNodeRequestBody {
    pub name: String,
}

impl CgiNodeResult {
    pub async fn remove_cgi_node_warp_request(
        body: CgiNodeRequestBody,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = run_powershell_command::<CgiNodeResult>(remove_cgi_node(body.name.as_str()))
            .await
            .unwrap();

        let result = warp::reply::json(&result);
        Ok(result)
    }
}
