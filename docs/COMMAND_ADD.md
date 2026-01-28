# Documentation - Commande `add`

## Description

La commande `add` permet d'ajouter une nouvelle tâche à la base de données SQLite avec tous les attributs optionnels.

## Syntaxe

```bash
task-cli add <DESCRIPTION> [OPTIONS]
```

## Arguments

- **`DESCRIPTION`** (obligatoire): La description de la tâche
  - Type: String
  - Exemple: `"Implémenter la recherche par tag"`

## Options

### `-t, --tags <TAGS>`
Ajouter des tags à la tâche (séparés par des virgules)
- Type: String (optionnel)
- Format: `"tag1, tag2, tag3"`
- Exemple: `--tags "feature,important,urgent"`

### `-p, --priority <PRIORITY>`
Définir la priorité de la tâche
- Type: String (optionnel)
- Valeurs acceptées:
  - `high`: Priorité haute
  - `medium`: Priorité moyenne (défaut)
  - `low`: Priorité basse
- Exemple: `--priority high`

### `-d, --due <DUE_DATE>`
Définir la date limite de la tâche
- Type: String (optionnel)
- Formats acceptés:
  - `YYYY-MM-DD` (exemple: `2026-03-15`)
  - RFC3339 (exemple: `2026-03-15T10:30:00Z`)
- Défaut: J+1 (demain)
- Exemple: `--due "2026-02-15"`

## Valeurs par défaut

| Attribut | Défaut |
|----------|--------|
| Priorité | Medium |
| Date limite | J+1 |
| Tags | Aucun |
| Statut | NotStarted (aucun) |

## Comportement

1. **Création de la tâche**: Une tâche est créée en mémoire avec tous les paramètres
2. **Génération ID**: Un UUID v4 est généré automatiquement
3. **Timestamps**: Les dates `created_at` et `updated_at` sont définies à l'heure actuelle
4. **Persistance**: La tâche est enregistrée dans la base de données SQLite
5. **Affichage**: Les détails de la tâche créée sont affichés

## Affichage de succès

```
✓ Tâche ajoutée avec succès!
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
  Description: Ma tâche importante
  Tags: feature, urgent
  Priorité: High
  Date de fin: 2026-02-15
```

## Gestion des erreurs

### Erreur de priorité invalide
```
Erreur: Priorité invalide 'invalid'. Utilisez: high, medium ou low
```

### Erreur de format de date
```
Erreur: Format de date invalide 'invalid-date'. Utilisez: YYYY-MM-DD ou RFC3339
```

### Erreur de base de données
```
Erreur: Impossible d'ouvrir la base de données: [message d'erreur]
```

## Exemples d'utilisation

### Exemple 1: Tâche simple
```bash
$ task-cli add "Écrire la documentation"
✓ Tâche ajoutée avec succès!
  ID: abc123...
  Description: Écrire la documentation
  Date de fin: 2026-01-31
```

### Exemple 2: Tâche avec priorité
```bash
$ task-cli add "Corriger le bug critique" --priority high
✓ Tâche ajoutée avec succès!
  ID: def456...
  Description: Corriger le bug critique
  Priorité: High
  Date de fin: 2026-01-31
```

### Exemple 3: Tâche complète
```bash
$ task-cli add "Implémenter la recherche" \
    --priority high \
    --tags "feature,backend,database" \
    --due "2026-02-28"
✓ Tâche ajoutée avec succès!
  ID: ghi789...
  Description: Implémenter la recherche
  Tags: feature, backend, database
  Priorité: High
  Date de fin: 2026-02-28
```

### Exemple 4: Tâche avec date spécifique
```bash
$ task-cli add "Réunion d'équipe" --due "2026-02-05T14:00:00Z"
✓ Tâche ajoutée avec succès!
  ID: jkl012...
  Description: Réunion d'équipe
  Date de fin: 2026-02-05
```

### Exemple 5: Tags multiples
```bash
$ task-cli add "Refactoring du code" --tags "refactor, cleanup, optimization"
✓ Tâche ajoutée avec succès!
  ID: mno345...
  Description: Refactoring du code
  Tags: refactor, cleanup, optimization
  Date de fin: 2026-01-31
```

## Localisation de la base de données

La base de données est stockée dans le répertoire de données standard de l'utilisateur:

- **Linux/macOS** : `~/.local/share/task-cli/tasks.db`
- **Windows** : `%APPDATA%\task-cli\data\tasks.db`
- **Fallback** : `tasks.db` dans le répertoire courant

## Notes techniques

- Les UUID sont générés avec la cryptographie sécurisée (uuid::Uuid::new_v4())
- Les dates sont toujours stockées en UTC (Utc timezone)
- Les tags sont stockés de manière normalisée (espaces avant/après supprimés)
- Les tags vides sont filtrés (tags = "tag1, , tag2" → ["tag1", "tag2"])
- La description est obligatoire et ne peut pas être vide

## Cas d'usage

✅ Créer rapidement une nouvelle tâche avec la description uniquement
✅ Ajouter une tâche importante avec une priorité élevée
✅ Créer une tâche avec une date limite spécifique
✅ Organiser les tâches avec des tags (par projet, type, etc.)
✅ Créer une tâche complète avec tous les détails en une seule commande
