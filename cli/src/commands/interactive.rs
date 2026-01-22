use crate::ui::Ui;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};

const MENU_ITEMS: &[&str] = &[
    "Install plugin",
    "Update plugin",
    "Sync MCP configuration",
    "Check status",
    "Run diagnostics",
    "Uninstall plugin",
    "Exit",
];

pub async fn run() -> Result<()> {
    Ui::header("Rhinolabs AI Plugin Manager");

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(MENU_ITEMS)
            .default(0)
            .interact()?;

        match selection {
            0 => super::install::run(None, false).await?,
            1 => super::update::run(false).await?,
            2 => {
                Ui::info("Please use: rhinolabs sync-mcp --url <URL>");
                Ui::info("Or: rhinolabs sync-mcp --file <PATH>");
            }
            3 => super::status::run()?,
            4 => super::doctor::run().await?,
            5 => super::uninstall::run(false)?,
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => {}
        }

        println!();
    }

    Ok(())
}
