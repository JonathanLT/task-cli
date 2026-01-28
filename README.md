# 🦀 Task CLI - Gestionnaire de Tâches en Rust

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

Un gestionnaire de tâches performant et élégant en ligne de commande, écrit en Rust avec persistance SQLite.

## ✨ Fonctionnalités

- ✅ **Gestion complète des tâches** : Ajouter, lister, modifier, supprimer, compléter
- 🎯 **Priorités** : High, Medium, Low (défaut: Medium)
- 📅 **Dates d'échéance** : Format YYYY-MM-DD ou RFC3339 (défaut: J+1)
- 🏷️ **Tags multiples** : Classez vos tâches avec des tags séparés par virgules
- 🔍 **Recherche avancée** : Recherche par pattern, tag, priorité, statut
- 📊 **Statuts** : NotStarted, InProgress, Completed, Canceled
- 💾 **Persistance SQLite** : Vos données sont sauvegardées automatiquement
- 🎨 **Interface colorée** : Symboles (○ ✓ ⚙ ✗) et couleurs pour chaque statut
- 🔑 **UUID unique** : Chaque tâche a un identifiant unique

## 🚀 Installation

### Prérequis

- Rust 1.70+ ([installer Rust](https://www.rust-lang.org/tools/install))

### Depuis les sources

```bash
# Cloner le repository
git clone https://github.com/JonathanLT/task-cli.git
cd task-cli

# Compiler et installer
cargo install --path .
```

### Depuis Cargo (à venir)

```bash
cargo install task-cli
```

## 📖 Utilisation

### Commandes disponibles

#### `add` - Ajouter une tâche

```bash
# Tâche simple
task-cli add "Apprendre Rust"

# Avec priorité (high, medium, low)
task-cli add "Finir le projet" --priority high

# Avec date d'échéance
task-cli add "Réviser pour l'examen" --due 2026-02-15

# Avec tags
task-cli add "Lire un livre" --tags lecture,personnel

# Complet
task-cli add "Préparer présentation" -p high -d 2026-02-10 -t travail,urgent
```

#### `list` - Lister les tâches

```bash
# Toutes les tâches
task-cli list

# Filtrer par statut
task-cli list --status completed
task-cli list --status inprogress
task-cli list --status notstarted

# Filtrer par plusieurs statuts
task-cli list -s completed -s inprogress
```

#### `complete` - Marquer une tâche comme complétée

```bash
# Utiliser l'UUID de la tâche
task-cli complete <UUID>
```

#### `edit` - Modifier une tâche

```bash
# Changer la description
task-cli edit <UUID> "Nouvelle description"

# Changer la priorité
task-cli edit <UUID> --priority medium

# Changer le statut
task-cli edit <UUID> --status inprogress

# Changer les tags
task-cli edit <UUID> --tags nouveau,tag

# Changer la date
task-cli edit <UUID> --due 2026-03-01

# Modifications multiples
task-cli edit <UUID> "Nouvelle desc" -p high -s inprogress -t work,urgent
```

#### `delete` - Supprimer une tâche

```bash
# Avec confirmation interactive
task-cli delete <UUID>

# Sans confirmation
task-cli delete <UUID> --force
```

#### `search` - Rechercher des tâches

```bash
# Recherche simple
task-cli search "rust"

# Avec filtres
task-cli search "projet" --tag travail
task-cli search "bug" --priority high
task-cli search "feature" --status inprogress

# Filtres multiples
task-cli search "api" --tag backend --priority high --status inprogress
```

## 🎨 Exemple d'affichage

```
3 tâche(s) trouvée(s):

○  [1] Apprendre Rust (ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742) (HIGH)
     Tags: dev, apprentissage
     Échéance: 2026-02-15

○  [2] Faire les courses (ID: abc123...) (LOW)
     Tags: personnel
     Échéance: 2026-01-30

✓  [3] Finir le rapport (ID: def456...) (MEDIUM)
     Tags: travail
     Échéance: 2026-01-20
```

### Légende des symboles

- `○` = NotStarted (Non commencée)
- `✓` = Completed (Complétée) - vert
- `⚙` = InProgress (En cours) - jaune
- `✗` = Canceled (Annulée) - rouge

### Couleurs des priorités

- 🔴 **HIGH** (rouge)
- 🟡 **MEDIUM** (jaune)
- 🟢 **LOW** (vert)

## 🛠️ Architecture du projet

```
task-cli/
├── src/
│   ├── commands/        # Implémentation des commandes
│   │   ├── mod.rs          # Export des modules
│   │   ├── add.rs          # ✅ Commande add
│   │   ├── list.rs         # ✅ Commande list
│   │   ├── edit.rs         # ✅ Commande edit
│   │   ├── delete.rs       # ✅ Commande delete
│   │   ├── complete.rs     # ✅ Commande complete
│   │   └── search.rs       # ✅ Commande search
│   ├── main.rs          # Point d'entrée et CLI
│   ├── task.rs          # Structure Task et enums (Priority, Status)
│   ├── storage.rs       # Persistance SQLite
│   ├── display.rs       # Utilitaires d'affichage
│   └── lib.rs           # Module principal
├── tests/
│   └── unit_tests.rs          # Tests d'intégration des commandes (13 tests)
├── examples/
│   └── storage_demo.rs        # Démonstration du système de storage
├── docs/
│   ├── COMMAND_ADD.md         # Documentation commande add
│   ├── COMMAND_LIST.md        # Documentation commande list
│   ├── COMMAND_EDIT.md        # Documentation commande edit
│   ├── COMMAND_DELETE.md      # Documentation commande delete
│   ├── COMMAND_COMPLETE.md    # Documentation commande complete
│   ├── COMMAND_SEARCH.md      # Documentation commande search
│   └── STORAGE.md             # Documentation système de stockage
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE-APACHE
├── LICENSE-MIT
└── README.md
```

## 🧪 Tests

```bash
# Lancer tous les tests
cargo test

# Tests unitaires (storage uniquement)
cargo test --lib

# Tests d'intégration (commandes)
cargo test --test unit_tests

# Tests avec sortie détaillée
cargo test -- --nocapture
```

Résultats des tests :
- **8 tests unitaires** (module storage) : CRUD, filtrage par statut/priorité/tag
- **13 tests d'intégration** : validation des 6 commandes + workflow complet
- **Total : 21 tests** - 100% de réussite ✅

## 🧪 Exemple de démonstration

Un exemple complet est disponible :

```bash
cargo run --example storage_demo
```

## 📚 Documentation

- `docs/COMMAND_ADD.md`
- `docs/COMMAND_LIST.md`
- `docs/COMMAND_EDIT.md`
- `docs/COMMAND_DELETE.md`
- `docs/COMMAND_COMPLETE.md`
- `docs/COMMAND_SEARCH.md`
- `docs/STORAGE.md`

## 🔧 Développement

### Compiler en mode debug

```bash
cargo build
```

### Compiler en mode release

```bash
cargo build --release
```

### Lancer le programme

```bash
cargo run -- add "Ma tâche"
```

### Format et lint

```bash
# Formater le code
cargo fmt

# Vérifier le style
cargo clippy
```

## 📦 Dépendances

- **clap** (4.5.23) : Parsing des arguments CLI
- **chrono** (0.4.43) : Gestion des dates et timestamps
- **rusqlite** (0.32.1) : Base de données SQLite embarquée
- **uuid** (1.20.0) : Génération d'identifiants uniques (v4)
- **colored** (3.1.1) : Couleurs et styles dans le terminal
- **directories** (6.0.0) : Chemins système multiplateformes
- **serde** (1.0) : Sérialisation/désérialisation
- **serde_json** (1.0) : Support JSON

## 🗺️ Roadmap

### Version 0.1.0 (Complétée ✅)
- [x] 6 commandes essentielles (add, list, edit, delete, complete, search)
- [x] Persistance SQLite
- [x] Tests complets (21 tests)
- [x] Documentation exhaustive

### Version 0.2.0 (À venir)
- [ ] Commande `stats` pour les statistiques
- [ ] Export JSON/CSV
- [ ] Import de tâches
- [ ] Configuration personnalisée (fichier config)
- [ ] Filtres avancés de recherche (date, priorité, tags combinés)

### Version 1.0.0 (Future)
- [ ] Interface TUI interactive avec `ratatui`
- [ ] Sous-tâches et dépendances
- [ ] Récurrence des tâches (quotidien, hebdomadaire, etc.)
- [ ] Notifications système
- [ ] Synchronisation cloud
- [ ] Intégration Git (tâches depuis issues)
- [ ] Plugin system
- [ ] Support multi-langues
- [ ] Filtres regex


## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :

1. Fork le projet
2. Créer une branche (`git checkout -b feature/amazing-feature`)
3. Commit vos changements (`git commit -m 'Add amazing feature'`)
4. Push sur la branche (`git push origin feature/amazing-feature`)
5. Ouvrir une Pull Request

### Guidelines

- Suivre les conventions Rust (rustfmt, clippy)
- Ajouter des tests pour les nouvelles fonctionnalités
- Mettre à jour la documentation

## 👤 Auteur

**JonathanLT**

- GitHub: [@JonathanLT](https://github.com/JonathanLT)
- GitHub.io: [JonathanLT](https://JonathanLT.github.io/)

## 🙏 Remerciements

- La communauté Rust pour l'écosystème incroyable
- Tous les contributeurs du projet

## 📞 Support

Pour toute question ou problème :
- Ouvrir une [issue](https://github.com/JonathanLT/task-cli/issues)
- Consulter la documentation dans `docs/`

---

⭐ N'oubliez pas de mettre une étoile si ce projet vous a aidé !
