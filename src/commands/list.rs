use crate::storage::TaskStorage;
use crate::task::Status;
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

pub fn handle_list(matches: &ArgMatches) {
    let db_path = get_db_path();

    match TaskStorage::new(db_path) {
        Ok(storage) => {
            // Déterminer si on filtre par statut
            let status_filters: Vec<Status> =
                if let Some(statuses) = matches.get_many::<String>("status") {
                    statuses.filter_map(|s| parse_status(s).ok()).collect()
                } else {
                    Vec::new()
                };

            // Récupérer les tâches
            let tasks = match status_filters.is_empty() {
                true => match storage.get_all_tasks() {
                    Ok(tasks) => tasks,
                    Err(e) => {
                        eprintln!(
                            "{} Impossible de récupérer les tâches: {}",
                            "Erreur:".red(),
                            e
                        );
                        return;
                    }
                },
                false => {
                    // Récupérer les tâches pour chaque statut
                    let mut all_tasks = Vec::new();
                    for status in &status_filters {
                        match storage.get_tasks_by_status(status) {
                            Ok(mut tasks) => all_tasks.append(&mut tasks),
                            Err(e) => {
                                eprintln!(
                                    "{} Impossible de récupérer les tâches: {}",
                                    "Erreur:".red(),
                                    e
                                );
                                return;
                            }
                        }
                    }
                    all_tasks
                }
            };

            // Afficher les tâches
            if tasks.is_empty() {
                println!("{}", "Aucune tâche trouvée.".yellow());
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
                            crate::task::Priority::High => "HIGH".red(),
                            crate::task::Priority::Medium => "MEDIUM".yellow(),
                            crate::task::Priority::Low => "LOW".green(),
                        }
                    );

                    if let Some(tags) = &task.tags {
                        println!("     {}: {}", "Tags".cyan(), tags.join(", "));
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
