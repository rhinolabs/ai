use crate::ui::Ui;
use anyhow::{Result, anyhow};
use rhinolabs_core::McpSync;

pub async fn run(url: Option<String>, file: Option<String>, dry_run: bool) -> Result<()> {
    Ui::header("🔄 Syncing MCP Configuration");

    let sync = match (url, file) {
        (Some(url), None) => {
            Ui::step(format!("Fetching from: {}", url).as_str());
            McpSync::from_remote(url)
        }
        (None, Some(file)) => {
            Ui::step(format!("Reading from: {}", file).as_str());
            McpSync::from_local(file)
        }
        (None, None) => {
            return Err(anyhow!("Must specify either --url or --file"));
        }
        (Some(_), Some(_)) => {
            return Err(anyhow!("Cannot specify both --url and --file"));
        }
    };

    let sync = sync.dry_run(dry_run);
    sync.sync().await?;

    println!();
    Ui::success("MCP configuration synced successfully");
    println!();
    Ui::info("Next step: Restart Claude Code to apply changes");

    Ok(())
}
