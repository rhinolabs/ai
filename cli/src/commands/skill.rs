use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::{CreateSkillInput, SkillCategory, Skills};

/// Parse category string to SkillCategory enum
fn parse_category(category: &str) -> SkillCategory {
    match category.to_lowercase().as_str() {
        "corporate" => SkillCategory::Corporate,
        "frontend" => SkillCategory::Frontend,
        "testing" => SkillCategory::Testing,
        "ai-sdk" | "aisdk" => SkillCategory::AiSdk,
        "utilities" => SkillCategory::Utilities,
        _ => SkillCategory::Custom,
    }
}

/// Format category for display
fn category_display(category: &SkillCategory) -> &'static str {
    match category {
        SkillCategory::Corporate => "Corporate",
        SkillCategory::Backend => "Backend",
        SkillCategory::Frontend => "Frontend",
        SkillCategory::Testing => "Testing",
        SkillCategory::AiSdk => "AI SDK",
        SkillCategory::Utilities => "Utilities",
        SkillCategory::Custom => "Custom",
    }
}

/// List all skills
pub fn list(json: bool) -> Result<()> {
    let skills = Skills::list()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&skills)?);
        return Ok(());
    }

    Ui::header("Skills");

    if skills.is_empty() {
        Ui::info("No skills installed yet.");
        Ui::info("Create skills in the GUI or use 'rhinolabs skill create'.");
        return Ok(());
    }

    // Group by category
    let mut current_category: Option<SkillCategory> = None;

    for skill in &skills {
        // Print category header when it changes
        if current_category.as_ref() != Some(&skill.category) {
            current_category = Some(skill.category.clone());
            println!();
            println!("  {}", category_display(&skill.category).bold().underline());
        }

        let status = if skill.enabled {
            "●".green()
        } else {
            "○".dimmed()
        };

        let custom_badge = if skill.is_custom {
            " [custom]".dimmed()
        } else {
            "".normal()
        };
        let source_badge = if let Some(source) = &skill.source_name {
            format!(" [{}]", source).dimmed()
        } else {
            "".normal()
        };

        println!(
            "    {} {}{}{}",
            status, skill.name, custom_badge, source_badge
        );
        println!("      ID: {}", skill.id.dimmed());
    }

    println!();
    Ok(())
}

/// Create a new skill
pub fn create(
    id: String,
    name: String,
    category: String,
    description: Option<String>,
) -> Result<()> {
    Ui::header("Create Skill");

    let category_enum = parse_category(&category);
    let desc = description.unwrap_or_else(|| format!("Custom skill: {}", name));

    Ui::step(&format!("Creating skill '{}'...", id));
    Ui::step(&format!("Category: {}", category_display(&category_enum)));

    let input = CreateSkillInput {
        id: id.clone(),
        name: name.clone(),
        description: desc,
        category: category_enum,
        content: format!("# {}\n\nYour skill instructions here.", name),
    };

    let skill = Skills::create(input)?;

    println!();
    Ui::success(&format!("Skill '{}' created successfully!", skill.id));
    Ui::info(&format!("Path: {}", skill.path));
    Ui::info("Edit the SKILL.md file to add your instructions.");

    Ok(())
}

/// Set the category for an existing skill
pub fn set_category(skill_id: String, category: String) -> Result<()> {
    Ui::header("Set Skill Category");

    let category_enum = parse_category(&category);

    Ui::step(&format!(
        "Setting category for '{}' to '{}'...",
        skill_id,
        category_display(&category_enum)
    ));

    Skills::set_category(&skill_id, category_enum.clone())?;

    println!();
    Ui::success(&format!(
        "Category for '{}' set to '{}'",
        skill_id,
        category_display(&category_enum)
    ));

    Ok(())
}

/// Show details of a specific skill
pub fn show(skill_id: &str, json: bool) -> Result<()> {
    let skill = Skills::get(skill_id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&skill)?);
        return Ok(());
    }

    match skill {
        Some(skill) => {
            Ui::header(&format!("Skill: {}", skill.name));

            let status = if skill.enabled { "Enabled" } else { "Disabled" };
            let status_color = if skill.enabled {
                status.green()
            } else {
                status.red()
            };

            println!("  ID:          {}", skill.id);
            println!("  Name:        {}", skill.name);
            println!("  Category:    {}", category_display(&skill.category));
            println!("  Status:      {}", status_color);
            println!(
                "  Custom:      {}",
                if skill.is_custom { "Yes" } else { "No" }
            );
            println!("  Description: {}", skill.description);
            println!("  Path:        {}", skill.path);

            if let Some(source) = &skill.source_name {
                println!("  Source:      {}", source);
            }

            if skill.is_modified {
                println!();
                Ui::warning("This skill has been modified from its original source.");
            }

            println!();
        }
        None => {
            Ui::error(&format!("Skill '{}' not found", skill_id));
        }
    }

    Ok(())
}
