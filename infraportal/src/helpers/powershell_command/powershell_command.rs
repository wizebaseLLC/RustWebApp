use powershell_script;
use serde::de::DeserializeOwned;

pub async fn run_powershell_command<T>(command: String) -> anyhow::Result<T>
where
    T: DeserializeOwned + Sync + Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let output = powershell_script::run(command.as_str(), false)?;

        match output.stdout() {
            Some(value) => Ok(serde_json::from_str(value)?),
            None => Err(anyhow::anyhow!("Command not successful - {}", command)),
        }
    })
    .await?
    .map_err(Into::into)
}
