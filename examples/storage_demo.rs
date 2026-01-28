use std::path::PathBuf;
use task_cli::storage::TaskStorage;
use task_cli::task::{Priority, Status, Task};

fn main() {
    // Créer une base de données de test
    let db_path = PathBuf::from("/tmp/demo_tasks.db");
    let mut storage =
        TaskStorage::new(db_path.clone()).expect("Impossible de créer la base de données");

    println!("=== Démonstration du système de stockage des tasks ===\n");

    // Créer une nouvelle task
    println!("1. Création d'une nouvelle task...");
    let mut task = Task::new(
        "Implémenter le système de stockage",
        Some(vec!["rust".to_string(), "database".to_string()]),
        None,
        Some(Priority::High),
        None,
    );

    println!("   Task créée: ID = {}", task.id);
    println!("   Description: {}", task.description);
    println!("   Tags: {:?}", task.tags);
    println!("   Priorité: {:?}", task.priority);
    println!(
        "   Date de fin: {}",
        task.due_date.format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Ajouter la task à la base de données
    storage
        .add_task(&task)
        .expect("Impossible d'ajouter la task");
    println!("   ✓ Task ajoutée à la base de données\n");

    // Récupérer la task
    println!("2. Récupération de la task depuis la base de données...");
    let retrieved = storage
        .get_task(&task.id)
        .expect("Erreur lors de la récupération")
        .expect("Task non trouvée");
    println!("   Description: {}", retrieved.description);
    println!("   Tags: {:?}", retrieved.tags);
    println!("   Priorité: {:?}", retrieved.priority);
    println!("   ✓ Task récupérée avec succès\n");

    // Créer une deuxième task
    println!("3. Création d'une deuxième task...");
    let task2 = Task::new(
        "Ajouter les commandes CLI",
        Some(vec!["clap".to_string()]),
        Some(Status::InProgress),
        Some(Priority::Medium),
        None,
    );
    storage
        .add_task(&task2)
        .expect("Impossible d'ajouter la deuxième task");
    println!("   ✓ Deuxième task ajoutée\n");

    // Récupérer toutes les tasks
    println!("4. Récupération de toutes les tasks...");
    let all_tasks = storage
        .get_all_tasks()
        .expect("Erreur lors de la récupération de toutes les tasks");
    println!("   Nombre de tasks: {}", all_tasks.len());
    for (i, t) in all_tasks.iter().enumerate() {
        println!(
            "   Task {}: {} (Priorité: {:?})",
            i + 1,
            t.description,
            t.priority
        );
    }
    println!("   ✓ Toutes les tasks récupérées\n");

    // Mettre à jour la première task
    println!("5. Mise à jour de la première task...");
    task.set_status(Some(Status::Completed));
    task.set_description("Implémentation du système de stockage SQLite terminée");
    storage
        .update_task(&task)
        .expect("Impossible de mettre à jour la task");
    println!("   ✓ Task mise à jour\n");

    // Vérifier la mise à jour
    println!("6. Vérification de la mise à jour...");
    let updated = storage
        .get_task(&task.id)
        .expect("Erreur lors de la récupération")
        .expect("Task non trouvée");
    println!("   Description: {}", updated.description);
    println!("   Statut: {:?}", updated.status);
    println!("   ✓ Mise à jour confirmée\n");

    // Supprimer la deuxième task
    println!("7. Suppression de la deuxième task...");
    storage
        .delete_task(&task2.id)
        .expect("Impossible de supprimer la task");
    println!("   ✓ Task supprimée\n");

    // Vérifier que la deuxième task a été supprimée
    println!("8. Vérification de la suppression...");
    let all_tasks = storage
        .get_all_tasks()
        .expect("Erreur lors de la récupération de toutes les tasks");
    println!("   Nombre de tasks restantes: {}", all_tasks.len());
    println!("   ✓ Suppression confirmée\n");

    // Récupérer les tasks par statut
    println!("9. Récupération des tasks complétées...");
    let completed = storage
        .get_tasks_by_status(&Status::Completed)
        .expect("Erreur lors de la récupération");
    println!("   Nombre de tasks complétées: {}", completed.len());
    for t in completed {
        println!("   - {}", t.description);
    }
    println!("   ✓ Tasks par statut récupérées\n");

    // Récupérer les tasks par priorité
    println!("10. Récupération des tasks haute priorité...");
    let high_priority = storage
        .get_tasks_by_priority(&Priority::High)
        .expect("Erreur lors de la récupération");
    println!("   Nombre de tasks haute priorité: {}", high_priority.len());
    for t in high_priority {
        println!("   - {} (Statut: {:?})", t.description, t.status);
    }
    println!("   ✓ Tasks par priorité récupérées\n");

    // Récupérer les tasks par tag
    println!("11. Récupération des tasks avec le tag 'rust'...");
    let rust_tasks = storage
        .get_tasks_by_tag("rust")
        .expect("Erreur lors de la récupération");
    println!(
        "   Nombre de tasks avec le tag 'rust': {}",
        rust_tasks.len()
    );
    for t in rust_tasks {
        println!("   - {} (Tags: {:?})", t.description, t.tags);
    }
    println!("   ✓ Tasks par tag récupérées\n");

    println!("=== Démonstration complètée avec succès ! ===");

    // Nettoyer
    let _ = std::fs::remove_file(&db_path);
}
