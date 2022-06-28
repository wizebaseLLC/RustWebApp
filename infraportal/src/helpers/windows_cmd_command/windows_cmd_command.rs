use std::ffi::OsStr;
use tokio::process::Command;

pub async fn run_windows_command<'a>(
    program: impl AsRef<OsStr>,
    args: Vec<&str>,
) -> anyhow::Result<String> {
    let output = Command::new(program).args(args).output().await?;
    let output = output.stdout;
    String::from_utf8(output).map_err(Into::into)
}
