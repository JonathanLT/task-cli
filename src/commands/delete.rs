use crate::storage::TaskStorage;
use clap::ArgMatches;
use colored::Colorize;
use std::io::{self, Write};
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

pub fn handle_delete(matches: &ArgMatches) {
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
            // Récupérer la tâche avant suppression pour affichage
            match storage.get_task(&task_id) {
                Ok(Some(task)) => {
                    // Demander confirmation si pas de flag --force
                    let force = matches.get_flag("force");

                    if !force {
                        println!(
                            "{}",
                            "⚠️  Êtes-vous sûr de vouloir supprimer cette tâche?".yellow()
                        );
                        println!("  {}: {}", "Description".cyan(), task.description);
                        println!("  {}: {}", "ID".cyan(), task.id);
                        print!("\n{} ", "Taper 'yes' pour confirmer:".yellow());
                        io::stdout().flush().ok();

                        let mut input = String::new();
                        if io::stdin().read_line(&mut input).is_err() || input.trim() != "yes" {
                            println!("{}", "Suppression annulée.".yellow());
                            return;
                        }
                    }

                    // Supprimer la tâche
                    match storage.delete_task(&task_id) {
                        Ok(_) => {
                            println!("{}", "✓ Tâche supprimée avec succès!".green());
                            println!("  {}: {}", "Description".cyan(), task.description);
                            println!("  {}: {}", "ID".cyan(), task.id);
                        }
                        Err(e) => {
                            eprintln!(
                                "{} Impossible de supprimer la tâche: {}",
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
