use crate::ui::Ui;
use anyhow::Result;
use colored::*;
use rhinolabs_core::{Doctor, diagnostics::CheckStatus};

pub async fn run() -> Result<()> {
    Ui::header("🔍 Running Diagnostics");

    println!();
    let report = Doctor::run().await?;

    for check in &report.checks {
        let (icon, color): (&str, fn(&str) -> ColoredString) = match check.status {
            CheckStatus::Pass => ("✓", str::green),
            CheckStatus::Fail => ("✗", str::red),
            CheckStatus::Warning => ("⚠", str::yellow),
        };

        println!("{} {}: {}",
            icon.bold(),
            color(&check.name),
            check.message
        );
    }

    println!();
    println!("━".repeat(50).bright_black());
    println!("{} passed, {} failed, {} warnings",
        format!("{}", report.passed).green(),
        format!("{}", report.failed).red(),
        format!("{}", report.warnings).yellow(),
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
