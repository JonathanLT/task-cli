use crate::storage::TaskStorage;
use crate::task::Status;
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

pub fn handle_complete(matches: &ArgMatches) {
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
                    // Vérifier si déjà complétée
                    if let Some(Status::Completed) = task.status {
                        println!("{}", "⚠️  Cette tâche est déjà complétée!".yellow());
                        println!("  {}: {}", "Description".cyan(), task.description);
                        println!("  {}: {}", "ID".cyan(), task.id);
                        return;
                    }

                    // Marquer comme complétée
                    task.set_status(Some(Status::Completed));

                    // Sauvegarder les modifications
                    match storage.update_task(&task) {
                        Ok(_) => {
                            println!("{}", "✓ Tâche marquée comme complétée!".green());
                            println!("  {}: {}", "Description".cyan(), task.description);
                            println!("  {}: {}", "ID".cyan(), task.id);
                            if let Some(tags) = &task.tags {
                                println!("  {}: {}", "Tags".cyan(), tags.join(", "));
                            }
                            println!("  {}: {:?}", "Priorité".cyan(), task.priority);
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
