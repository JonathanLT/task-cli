use crate::storage::TaskStorage;
use crate::task::{Priority, Status};
use chrono::DateTime;
use clap::ArgMatches;
use colored::Colorize;
use std::path::PathBuf;
use uuid::Uuid;

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

pub fn handle_edit(matches: &ArgMatches) {
    let id_str = matches.get_one::<String>("id").unwrap();

    // Parser l'UUID
    let task_id = match Uuid::parse_str(id_str) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("{} UUID invalide: '{}'", "Erreur:".red(), id_str);
            return;
        }
    };

    // Ouvrir la base de données
    let db_path = get_db_path();
    match TaskStorage::new(db_path) {
        Ok(mut storage) => {
            // Récupérer la tâche existante
            match storage.get_task(&task_id) {
                Ok(Some(mut task)) => {
                    // Mettre à jour la description si fournie
                    if let Some(description) = matches.get_one::<String>("description") {
                        task.set_description(description);
                    }

                    // Mettre à jour la priorité si fournie
                    if let Some(priority_str) = matches.get_one::<String>("priority") {
                        match parse_priority(priority_str) {
                            Ok(priority) => {
                                task.set_priority(priority);
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Erreur:".red(), e);
                                return;
                            }
                        }
                    }

                    // Mettre à jour le statut si fourni
                    if let Some(status_str) = matches.get_one::<String>("status") {
                        match parse_status(status_str) {
                            Ok(status) => {
                                task.set_status(Some(status));
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Erreur:".red(), e);
                                return;
                            }
                        }
                    }

                    // Mettre à jour les tags si fournis
                    if let Some(tags_str) = matches.get_one::<String>("tags") {
                        let new_tags: Vec<String> = tags_str
                            .split(',')
                            .map(|t| t.trim().to_string())
                            .filter(|t| !t.is_empty())
                            .collect();

                        task.tags = if new_tags.is_empty() {
                            None
                        } else {
                            Some(new_tags)
                        };
                        task.updated_at = chrono::Utc::now();
                    }

                    // Mettre à jour la date limite si fournie
                    if let Some(due_str) = matches.get_one::<String>("due") {
                        match parse_due_date(due_str) {
                            Ok(due_date) => {
                                task.set_due_date(due_date);
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Erreur:".red(), e);
                                return;
                            }
                        }
                    }

                    // Sauvegarder les modifications
                    match storage.update_task(&task) {
                        Ok(_) => {
                            println!("{}", "✓ Tâche mise à jour avec succès!".green());
                            println!("  {}: {}", "ID".cyan(), task.id);
                            println!("  {}: {}", "Description".cyan(), task.description);
                            if let Some(tags) = &task.tags {
                                println!("  {}: {}", "Tags".cyan(), tags.join(", "));
                            }
                            println!("  {}: {:?}", "Priorité".cyan(), task.priority);
                            if let Some(status) = &task.status {
                                println!("  {}: {:?}", "Statut".cyan(), status);
                            }
                            println!(
                                "  {}: {}",
                                "Échéance".cyan(),
                                task.due_date.format("%Y-%m-%d")
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "{} Impossible de mettre à jour la tâche: {}",
                                "Erreur:".red(),
                                e
                            );
                        }
                    }
                }
                Ok(None) => {
                    eprintln!(
                        "{} Tâche introuvable avec l'ID: {}",
                        "Erreur:".red(),
                        id_str
                    );
                }
                Err(e) => {
                    eprintln!(
                        "{} Impossible de récupérer la tâche: {}",
                        "Erreur:".red(),
                        e
                    );
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
