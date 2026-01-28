# Changelog - Task CLI

## [0.1.0] - 2026-01-29

### Ajouté

#### Commandes
- ✅ Commande `add` : Ajouter une tâche (description, tags, priorité, date)
- ✅ Commande `list` : Lister les tâches avec filtrage par statut
- ✅ Commande `edit` : Éditer une tâche (description, tags, priorité, statut, date)
- ✅ Commande `delete` : Supprime une tâche
- ✅ Commande `complete` : Termine une tâche
- ✅ Commande `search` : Recherche de tâches par pattern, tag, priorité, statut

#### Module Task (`src/task.rs`)
- ✅ Structure `Task` avec tous les attributs nécessaires
  - `id`: UUID unique
  - `description`: Description obligatoire
  - `tags`: Tags optionnels
  - `status`: Statut optionnel
  - `priority`: Priorité (défaut: Medium)
  - `due_date`: Date limite (défaut: J+1)
  - `created_at` et `updated_at`: Timestamps automatiques
- ✅ Enums
  - `Priority`: High, Medium, Low
  - `Status`: Completed, NotStarted, InProgress, Canceled
- ✅ Méthodes
  - Constructeur `new()` avec valeurs par défaut
  - Setters avec mise à jour automatique de `updated_at`
  - Gestion des tags (`add_tag()`, `remove_tag()`)

#### Module Storage (`src/storage.rs`)
- ✅ Gestionnaire `TaskStorage` pour persistance SQLite
- ✅ 2 tables: `tasks` et `task_tags`
- ✅ Opérations CRUD complètes
  - `add_task()`
  - `get_task()`
  - `get_all_tasks()`
  - `update_task()`
  - `delete_task()`
- ✅ Requêtes spécialisées
  - `get_tasks_by_status()`
  - `get_tasks_by_priority()`
  - `get_tasks_by_tag()`

#### Tests
- ✅ 8 tests unitaires du module storage

#### Documentation
- ✅ `docs/STORAGE.md`: Documentation technique du système de stockage
- ✅ `docs/COMMAND_ADD.md`: Documentation détaillée de la commande add
- ✅ `docs/COMMAND_LIST.md`: Documentation détaillée de la commande list
- ✅ `docs/COMMAND_EDIT.md`: Documentation détaillée de la commande edit
- ✅ `docs/COMMAND_DELETE.md`: Documentation détaillée de la commande delete
- ✅ `docs/COMMAND_COMPLETE.md`: Documentation détaillée de la commande complete
- ✅ `docs/COMMAND_SEARCH.md`: Documentation détaillée de la commande search

### Modifications

- ✅ `src/main.rs`: Dispatcher des commandes
- ✅ `src/lib.rs`: Exports des modules publics
- ✅ `Cargo.toml`: Dépendances (rusqlite, uuid v4, chrono, clap, etc.)

### Notes de version

- Version initiale avec persistance SQLite et 6 commandes (add, list, edit, delete, complete, search)

## Statut des tests

### Tests unitaires
```
test storage::tests::test_add_and_get_task ... ok
test storage::tests::test_add_task_with_tags ... ok
test storage::tests::test_delete_task ... ok
test storage::tests::test_get_all_tasks ... ok
test storage::tests::test_get_tasks_by_priority ... ok
test storage::tests::test_get_tasks_by_status ... ok
test storage::tests::test_get_tasks_by_tag ... ok
test storage::tests::test_update_task ... ok

Result: 8 passed; 0 failed
```

### Compilation
```
✓ cargo build: Succès
✓ cargo check: Succès
✓ cargo test --lib: 8 tests passants
```

## Problèmes connus

Aucun problème signalé pour cette version.

## Notes de développement

### Choix de conception

1. **SQLite** : Choisi pour la persistance locale et l'absence de dépendance serveur
2. **UUID v4** : Pour les IDs uniques et décentralisées
3. **Chrono** : Pour la gestion robuste des dates en UTC
4. **Rusqlite** : Bindings Rust natifs pour SQLite
5. **Colored** : Pour l'affichage ergonomique du terminal

### Orthographe

- "Medium" pour la priorité moyenne (orthographe standard)
- "NotStarted" en CamelCase: Convention Rust pour les énums

### Architecture

- **Separation of concerns** : Modules séparés pour task, storage, commandes
- **Library vs Binary** : Code partagé dans lib.rs, main.rs reste minimal
- **Error handling** : Messages clairs avec la crate `colored`
- **XDG compliance** : Respects les conventions des répertoires utilisateur

## Métriques

| Métrique | Valeur |
|----------|--------|
| Lignes de code | ~1300 |
| Fichiers Rust | 10 |
| Fichiers de documentation | 7 |
| Tests | 8 |
| Dépendances principales | 7 |
| Compilation | ~0.3s |

## Ressources

- [Documentation Rust](https://doc.rust-lang.org/)
- [Clap - CLI parsing](https://docs.rs/clap/latest/clap/)
- [Rusqlite](https://docs.rs/rusqlite/latest/rusqlite/)
- [Chrono](https://docs.rs/chrono/latest/chrono/)
- [UUID](https://docs.rs/uuid/latest/uuid/)

---

**Auteur** : [Jonathan LOQUET](https://jonathanlt.github.io)  
**Date de création** : 2026-01-29  
**Licence** : MIT OR Apache-2.0
