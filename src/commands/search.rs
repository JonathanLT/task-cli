use crate::storage::TaskStorage;
use crate::task::{Priority, Status};
use clap::ArgMatches;
use colored::Colorize;
use std::path::PathBuf;

/// Obtient le chemin de la base de données
fn get_db_path() -> PathBuf {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "task-cli") {
        let data_dir = proj_dirs.data_dir();
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

/// Convertit une chaîne de statut en enum Status
fn parse_status(status_str: &str) -> Result<Status, String> {
    match status_str.to_lowercase().as_str() {
        "completed" => Ok(Status::Completed),
        "notstarted" | "not_started" | "not started" => Ok(Status::NotStarted),
        "inprogress" | "in_progress" | "in progress" => Ok(Status::InProgress),
        "canceled" | "cancelled" => Ok(Status::Canceled),
        _ => Err(format!(
            "Statut invalide '{}'. Utilisez: completed, notstarted, inprogress ou canceled",
            status_str
        )),
    }
}

pub fn handle_search(matches: &ArgMatches) {
    let pattern = matches.get_one::<String>("pattern").unwrap();

    // Ouvrir la base de données
    let db_path = get_db_path();
    match TaskStorage::new(db_path) {
        Ok(storage) => {
            // Récupérer toutes les tâches
            let mut tasks = match storage.get_all_tasks() {
                Ok(tasks) => tasks,
                Err(e) => {
                    eprintln!(
                        "{} Impossible de récupérer les tâches: {}",
                        "Erreur:".red(),
                        e
                    );
                    return;
                }
            };

            // Filtrer par pattern dans la description (case-insensitive)
            let pattern_lower = pattern.to_lowercase();
            tasks.retain(|task| task.description.to_lowercase().contains(&pattern_lower));

            // Filtrer par tag si fourni
            if let Some(tag) = matches.get_one::<String>("tag") {
                let tag_lower = tag.to_lowercase();
                tasks.retain(|task| {
                    if let Some(tags) = &task.tags {
                        tags.iter().any(|t| t.to_lowercase() == tag_lower)
                    } else {
                        false
                    }
                });
            }

            // Filtrer par priorité si fournie
            if let Some(priority_str) = matches.get_one::<String>("priority") {
                match parse_priority(priority_str) {
                    Ok(priority) => {
                        tasks.retain(|task| task.priority == priority);
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Erreur:".red(), e);
                        return;
                    }
                }
            }

            // Filtrer par statut si fourni
            if let Some(statuses) = matches.get_many::<String>("status") {
                let status_filters: Vec<Status> =
                    statuses.filter_map(|s| parse_status(s).ok()).collect();

                if !status_filters.is_empty() {
                    tasks.retain(|task| {
                        if let Some(status) = &task.status {
                            status_filters.contains(status)
                        } else {
                            false
                        }
                    });
                }
            }

            // Afficher les résultats
            if tasks.is_empty() {
                println!(
                    "{}",
                    "Aucune tâche trouvée correspondant aux critères.".yellow()
                );
            } else {
                println!(
                    "{}",
                    format!("\n{} tâche(s) trouvée(s):\n", tasks.len()).bold()
                );

                for (i, task) in tasks.iter().enumerate() {
                    let status_str = match &task.status {
                        Some(Status::Completed) => "✓".green(),
                        Some(Status::InProgress) => "⚙".yellow(),
                        Some(Status::Canceled) => "✗".red(),
                        _ => "○".white(),
                    };

                    println!(
                        "{}  {} {} {} ({})",
                        status_str,
                        format!("[{}]", i + 1).cyan(),
                        task.description,
                        format!("(ID: {})", task.id).dimmed(),
                        match &task.priority {
                            Priority::High => "HIGH".red(),
                            Priority::Medium => "MEDIUM".yellow(),
                            Priority::Low => "LOW".green(),
                        }
                    );

                    if let Some(tags) = &task.tags {
                        println!("     {}: {}", "Tags".cyan(), tags.join(", "));
                    }

                    if let Some(status) = &task.status {
                        println!("     {}: {:?}", "Statut".cyan(), status);
                    }

                    println!(
                        "     {}: {}",
                        "Échéance".cyan(),
                        task.due_date.format("%Y-%m-%d")
                    );
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{} Impossible d'ouvrir la base de données: {}",
                "Erreur:".red(),
                e
            );
        }
    }
}
