# Stockage SQLite des Tasks - Documentation

## Vue d'ensemble

L'implémentation du stockage SQLite pour les tasks offre une persistance complète des données dans une base de données SQLite. Le système gère le stockage et la récupération des tâches avec tous leurs attributs (description, priorité, statut, tags, dates).

## Architecture

### Structure `TaskStorage`

```rust
pub struct TaskStorage {
    #[allow(dead_code)]
    db_path: PathBuf,
    conn: Connection,
}
```

Le gestionnaire de stockage encapsule une connexion SQLite et maintient le chemin d'accès à la base de données.

### Tables de la base de données

#### Table `tasks`
Stocke les informations principales des tâches :
- `id` (TEXT PRIMARY KEY): Identifiant unique UUID
- `description` (TEXT NOT NULL): Description de la tâche
- `priority` (TEXT NOT NULL): Priorité (High, Medium, Low)
- `status` (TEXT): Statut (Completed, NotStarted, InProgress, Canceled)
- `due_date` (TEXT NOT NULL): Date limite (format RFC3339)
- `created_at` (TEXT NOT NULL): Date de création (format RFC3339)
- `updated_at` (TEXT NOT NULL): Date de dernière modification (format RFC3339)

#### Table `task_tags`
Stocke les tags associés aux tâches (relation many-to-many) :
- `task_id` (TEXT NOT NULL): Référence à la tâche
- `tag` (TEXT NOT NULL): Valeur du tag
- Clé primaire composée: (task_id, tag)
- Clé étrangère avec suppression en cascade

## API du gestionnaire de stockage

### Opérations CRUD

#### `new(db_path: PathBuf) -> SqlResult<Self>`
Crée ou ouvre une base de données SQLite. Initialise les tables si elles n'existent pas.

#### `add_task(&mut self, task: &Task) -> SqlResult<()>`
Ajoute une nouvelle tâche à la base de données, y compris ses tags.

#### `get_task(&self, id: &Uuid) -> SqlResult<Option<Task>>`
Récupère une tâche spécifique par son ID.

#### `get_all_tasks(&self) -> SqlResult<Vec<Task>>`
Récupère toutes les tâches, ordonnées par date de création (descendant).

#### `update_task(&mut self, task: &Task) -> SqlResult<()>`
Met à jour une tâche existante et ses tags associés.

#### `delete_task(&mut self, id: &Uuid) -> SqlResult<()>`
Supprime une tâche et ses tags associés (suppression en cascade).

### Opérations de requête

#### `get_tasks_by_status(&self, status: &Status) -> SqlResult<Vec<Task>>`
Récupère toutes les tâches avec un statut spécifique.

#### `get_tasks_by_priority(&self, priority: &Priority) -> SqlResult<Vec<Task>>`
Récupère toutes les tâches avec une priorité spécifique.

#### `get_tasks_by_tag(&self, tag: &str) -> SqlResult<Vec<Task>>`
Récupère toutes les tâches associées à un tag spécifique.

## Gestion des dates

Les dates sont stockées en format RFC3339 pour assurer la compatibilité et la lisibilité. Elles sont automatiquement converties vers le fuseau horaire UTC.

## Gestion des tags

Les tags sont stockés dans une table séparée pour une flexibilité maximale :
- Une tâche peut avoir zéro, un ou plusieurs tags
- Les tags sont représentés comme `Option<Vec<String>>` au niveau de l'application
- `None` signifie pas de tags, au lieu d'un vecteur vide

## Exemple d'utilisation

```rust
use task_cli::task::{Priority, Status, Task};
use task_cli::storage::TaskStorage;
use std::path::PathBuf;

// Créer ou ouvrir la base de données
let mut storage = TaskStorage::new(PathBuf::from("tasks.db"))?;

// Créer une nouvelle tâche
let task = Task::new(
    "Implémenter la feature X",
    Some(vec!["rust".to_string(), "feature".to_string()]),
    Some(Status::InProgress),
    Some(Priority::High),
    None
);

// Ajouter à la base de données
storage.add_task(&task)?;

// Récupérer la tâche
let retrieved = storage.get_task(&task.id)?;

// Récupérer toutes les tâches haute priorité
let high_priority = storage.get_tasks_by_priority(&Priority::High)?;

// Mettre à jour la tâche
let mut updated_task = retrieved.unwrap();
updated_task.set_status(Some(Status::Completed));
storage.update_task(&updated_task)?;

// Supprimer la tâche
storage.delete_task(&task.id)?;
```

## Tests

L'implémentation inclut une suite complète de tests unitaires :

```bash
cargo test --lib storage
```

Tests inclus :
- `test_add_and_get_task`: Ajout et récupération d'une tâche
- `test_add_task_with_tags`: Gestion des tags
- `test_update_task`: Mise à jour des tâches
- `test_delete_task`: Suppression des tâches
- `test_get_all_tasks`: Récupération de toutes les tâches

Tous les tests utilisent des bases de données temporaires uniques pour éviter les conflits.

## Exemple de démonstration

Un exemple complet est disponible :

```bash
cargo run --example storage_demo
```

Cet exemple démontre :
1. Création et insertion de tâches
2. Récupération de tâches
3. Mise à jour de tâches
4. Suppression de tâches
5. Requêtes par statut, priorité et tag

## Dépendances

- `rusqlite`: Bindings Rust pour SQLite
- `chrono`: Gestion des dates et heures
- `uuid`: Génération d'identifiants uniques

## Gestion des erreurs

Toutes les opérations retournent `Result<T, SqlError>` pour permettre une gestion appropriée des erreurs de base de données.

## Performance

- Les requêtes utilisent des prepared statements pour éviter les injections SQL
- Les tags sont récupérés de manière efficace avec des requêtes séparées
- La base de données est créée avec les contraintes appropriées (clés primaires, clés étrangères)
