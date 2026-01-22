use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::{Doctor, diagnostics::CheckStatus};

pub async fn run() -> Result<()> {
    Ui::header("🔍 Running Diagnostics");

    println!();
    let report = Doctor::run().await?;

    for check in &report.checks {
        let (icon, name_colored) = match check.status {
            CheckStatus::Pass => ("✓", check.name.green()),
            CheckStatus::Fail => ("✗", check.name.red()),
            CheckStatus::Warning => ("⚠", check.name.yellow()),
        };

        println!("{} {}: {}",
            icon.bold(),
            name_colored,
            check.message
        );
    }

    println!();
    println!("{}", "━".repeat(50).bright_black());
    println!("{} passed, {} failed, {} warnings",
        report.passed.to_string().green(),
        report.failed.to_string().red(),
        report.warnings.to_string().yellow(),
    );
    println!();

    if report.failed > 0 {
        Ui::info("Some checks failed. Please address the issues above.");
    } else if report.warnings > 0 {
        Ui::info("All critical checks passed, but there are warnings.");
    } else {
        Ui::success("All checks passed!");
    }

    Ok(())
}
