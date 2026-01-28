# Documentation - Commande `search`

## Description

La commande `search` permet de rechercher des tâches selon différents critères : pattern dans la description, tags, priorité et statut.

## Syntaxe

```bash
task-cli search <PATTERN> [OPTIONS]
```

## Arguments

- **`PATTERN`** (obligatoire): Le motif de recherche dans la description
  - Type: String
  - Recherche case-insensitive
  - Exemple: `"API"`, `"bug"`, `"feature"`

## Options

### `-t, --tag <TAG>`
Filtrer par tag spécifique
- Type: String (optionnel)
- Case-insensitive
- Exemple: `--tag "urgent"`

### `-p, --priority <PRIORITY>`
Filtrer par priorité
- Type: String (optionnel)
- Valeurs acceptées:
  - `high`: Priorité haute
  - `medium`: Priorité moyenne
  - `low`: Priorité basse
- Exemple: `--priority high`

### `-s, --status <STATUS>`
Filtrer par statut (peut être répété)
- Type: String (optionnel, répétable)
- Valeurs acceptées:
  - `completed`: Tâches complétées
  - `notstarted` ou `not_started`: Tâches non commencées
  - `inprogress` ou `in_progress`: Tâches en cours
  - `canceled` ou `cancelled`: Tâches annulées
- Exemple: `--status completed --status inprogress`

## Comportement

1. **Récupération**: Toutes les tâches sont récupérées de la base de données
2. **Filtrage pattern**: Les tâches dont la description contient le pattern (case-insensitive)
3. **Filtrage tag**: Optionnellement, filtre sur un tag spécifique
4. **Filtrage priorité**: Optionnellement, filtre sur une priorité
5. **Filtrage statut**: Optionnellement, filtre sur un ou plusieurs statuts
6. **Affichage**: Les résultats sont affichés avec tous leurs détails

## Affichage des résultats

```
2 tâche(s) trouvée(s):

○  [1] Implémenter API REST (ID: 5c5052fc-...) (HIGH)
     Tags: backend, api
     Échéance: 2026-02-15

✓  [2] Documenter API (ID: abc123-...) (LOW)
     Tags: documentation
     Statut: Completed
     Échéance: 2026-01-30
```

## Affichage aucun résultat

```
Aucune tâche trouvée correspondant aux critères.
```

## Gestion des erreurs

### Priorité invalide
```
Erreur: Priorité invalide 'invalid'. Utilisez: high, medium ou low
```

### Erreur de base de données
```
Erreur: Impossible d'ouvrir la base de données: [message d'erreur]
```

### Erreur de récupération
```
Erreur: Impossible de récupérer les tâches: [message d'erreur]
```

## Exemples d'utilisation

### Exemple 1: Recherche simple
```bash
$ task-cli search "API"
2 tâche(s) trouvée(s):
○  [1] Implémenter API REST (HIGH)
○  [2] Documenter API (LOW)
```

### Exemple 2: Recherche avec tag
```bash
$ task-cli search "bug" --tag "urgent"
1 tâche(s) trouvée(s):
○  [1] Corriger bug de connexion (HIGH)
     Tags: bug, urgent
```

### Exemple 3: Recherche avec priorité
```bash
$ task-cli search "feature" --priority high
1 tâche(s) trouvée(s):
○  [1] Implémenter nouvelle feature (HIGH)
```

### Exemple 4: Recherche avec statut
```bash
$ task-cli search "API" --status completed
1 tâche(s) trouvée(s):
✓  [1] Documenter API (LOW)
     Statut: Completed
```

### Exemple 5: Recherche multi-critères
```bash
$ task-cli search "API" --tag "backend" --priority high --status inprogress
1 tâche(s) trouvée(s):
⚙  [1] Implémenter API REST (HIGH)
     Tags: backend, api
     Statut: InProgress
```

### Exemple 6: Recherche avec plusieurs statuts
```bash
$ task-cli search "tâche" --status completed --status inprogress
2 tâche(s) trouvée(s):
✓  [1] Tâche 1 (Completed)
⚙  [2] Tâche 2 (InProgress)
```

## Cas d'utilisation

✅ Trouver toutes les tâches contenant un mot-clé
✅ Rechercher par tag spécifique
✅ Filtrer par priorité
✅ Voir les tâches complétées d'un projet
✅ Combiner plusieurs critères de recherche
✅ Audit rapide des tâches

## Points importants

### Recherche case-insensitive
- Le pattern et les tags sont recherchés sans distinction majuscule/minuscule
- `"API"` trouvera `"api"`, `"Api"`, `"API"`

### Filtres cumulatifs
- Tous les filtres sont appliqués en AND
- Une tâche doit correspondre à tous les critères pour être affichée

### Symboles de statut
Les mêmes symboles que `list`:
- `○` = NotStarted (blanc)
- `✓` = Completed (vert)
- `⚙` = InProgress (jaune)
- `✗` = Canceled (rouge)

### Couleurs de priorité
- 🔴 HIGH (rouge)
- 🟡 MEDIUM (jaune)
- 🟢 LOW (vert)

## Workflow typique

```bash
# 1. Rechercher des tâches liées à une fonctionnalité
$ task-cli search "authentication"

# 2. Affiner avec un tag
$ task-cli search "authentication" --tag "backend"

# 3. Voir seulement les tâches en cours
$ task-cli search "authentication" --tag "backend" --status inprogress

# 4. Travailler sur une tâche trouvée
$ task-cli edit <ID> --status inprogress
```

## Comparaison avec list

### `list`
- Affiche toutes les tâches
- Filtrage uniquement par statut
- Ordre chronologique

### `search`
- Affiche les tâches correspondant à un pattern
- Filtrage multi-critères (pattern + tag + priorité + statut)
- Ordre chronologique des résultats

## Cas d'usage avancés

### Recherche de tâches urgentes
```bash
$ task-cli search "" --priority high --status notstarted
# Toutes les tâches HIGH pas encore commencées
```

### Recherche par projet (via tags)
```bash
$ task-cli search "" --tag "projet-x"
# Toutes les tâches du projet X
```

### Audit des tâches complétées
```bash
$ task-cli search "" --status completed
# Toutes les tâches complétées (équivalent à list --status completed)
```

### Recherche combinée
```bash
$ task-cli search "bug" --priority high --status inprogress
# Bugs HIGH en cours de correction
```

## Notes techniques

- La recherche utilise `TaskStorage::get_all_tasks()` puis filtre en mémoire
- Tous les filtres sont appliqués séquentiellement (AND logique)
- Les statuts invalides sont ignorés silencieusement lors du parsing
- La recherche est performante jusqu'à plusieurs milliers de tâches

## Limitations

- Pas de recherche par date (due_date)
- Pas de recherche par date de création/modification
- Pas de recherche par ID partiel
- Pas de regex ou wildcard dans le pattern
- Pas d'opérateur OR entre critères

## Sécurité

- ✅ Validation des priorités
- ✅ Validation des statuts
- ✅ Pas d'injection SQL (récupération complète puis filtrage)
- ✅ Messages d'erreur explicites
