use crate::storage::TaskStorage;
use crate::task::{Priority, Task};
use chrono::DateTime;
use clap::ArgMatches;
use colored::Colorize;
use std::path::PathBuf;

/// Obtient le chemin de la base de données
fn get_db_path() -> PathBuf {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "task-cli") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir).ok();
        data_dir.join("tasks.db")
    } else {
        PathBuf::from("tasks.db")
    }
}

/// Convertit une chaîne de priorité en enum Priority
fn parse_priority(priority_str: &str) -> Result<Priority, String> {
    match priority_str.to_lowercase().as_str() {
        "high" => Ok(Priority::High),
        "medium" => Ok(Priority::Medium),
        "low" => Ok(Priority::Low),
        _ => Err(format!(
            "Priorité invalide '{}'. Utilisez: high, medium ou low",
            priority_str
        )),
    }
}

/// Convertit une chaîne de date en DateTime<Utc>
fn parse_due_date(date_str: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    // Essayer le format RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }

    // Essayer le format YYYY-MM-DD
    if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            naive_datetime,
            chrono::Utc,
        ));
    }

    Err(format!(
        "Format de date invalide '{}'. Utilisez: YYYY-MM-DD ou RFC3339",
        date_str
    ))
}

pub fn handle_add(matches: &ArgMatches) {
    let description = matches.get_one::<String>("description").unwrap();

    // Parser les tags
    let tags = matches.get_one::<String>("tags").map(|tags_str| {
        tags_str
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
    });

    // Parser la priorité
    let priority = if let Some(priority_str) = matches.get_one::<String>("priority") {
        match parse_priority(priority_str) {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("{} {}", "Erreur:".red(), e);
                return;
            }
        }
    } else {
        None
    };

    // Parser la date de fin
    let due_date = if let Some(due_str) = matches.get_one::<String>("due") {
        match parse_due_date(due_str) {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("{} {}", "Erreur:".red(), e);
                return;
            }
        }
    } else {
        None
    };

    // Créer la task
    let task = Task::new(description, tags.clone(), None, priority.clone(), due_date);

    // Ouvrir la base de données et ajouter la task
    let db_path = get_db_path();
    match TaskStorage::new(db_path) {
        Ok(mut storage) => match storage.add_task(&task) {
            Ok(_) => {
                println!("{}", "✓ Tâche ajoutée avec succès!".green());
                println!("  {}: {}", "ID".cyan(), task.id);
                println!("  {}: {}", "Description".cyan(), description);
                if let Some(t) = &tags {
                    if !t.is_empty() {
                        println!("  {}: {}", "Tags".cyan(), t.join(", "));
                    }
                }
                if let Some(p) = &priority {
                    println!("  {}: {:?}", "Priorité".cyan(), p);
                }
                println!(
                    "  {}: {}",
                    "Date de fin".cyan(),
                    task.due_date.format("%Y-%m-%d")
                );
            }
            Err(e) => {
                eprintln!("{} Impossible d'ajouter la tâche: {}", "Erreur:".red(), e);
            }
        },
        Err(e) => {
            eprintln!(
                "{} Impossible d'ouvrir la base de données: {}",
                "Erreur:".red(),
                e
            );
        }
    }
}
