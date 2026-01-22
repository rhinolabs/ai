use colored::*;

pub struct Ui;

impl Ui {
    pub fn header(text: &str) {
        println!();
        println!("{}", "═".repeat(50).bright_blue());
        println!("{}", text.bright_blue().bold());
        println!("{}", "═".repeat(50).bright_blue());
        println!();
    }

    pub fn success(text: &str) {
        println!("{} {}", "✓".green().bold(), text.green());
    }

    pub fn error(text: &str) {
        println!("{} {}", "✗".red().bold(), text.red());
    }

    pub fn warning(text: &str) {
        println!("{} {}", "⚠".yellow().bold(), text.yellow());
    }

    pub fn info(text: &str) {
        println!("{} {}", "ℹ".blue().bold(), text);
    }

    pub fn step(text: &str) {
        println!("  {} {}", "→".cyan(), text);
    }

    pub fn section(title: &str) {
        println!();
        println!("{}", title.bold().underline());
    }
}
