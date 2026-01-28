# Documentation - Commande `list`

## Description

La commande `list` affiche toutes les tâches enregistrées ou filtrées par statut.

## Syntaxe

```bash
task-cli list [OPTIONS]
```

## Options

### `-s, --status <STATUS>`
Filtrer les tâches par statut (peut être utilisé plusieurs fois)
- Type: String (optionnel)
- Valeurs acceptées:
  - `completed`: Tâches complétées
  - `notstarted` ou `not_started`: Tâches non commencées
  - `inprogress` ou `in_progress`: Tâches en cours
  - `canceled` ou `cancelled`: Tâches annulées
- Exemple: `--status completed --status inprogress`

## Affichage

Chaque tâche est affichée avec les informations suivantes:

```
[Statut] [Numéro] Description (ID: uuid) (PRIORITÉ)
     Tags: tag1, tag2, ...
     Échéance: YYYY-MM-DD
```

### Symboles de statut

| Symbole | Statut | Couleur |
|---------|--------|--------|
| ✓ | Completed | Vert |
| ⚙ | InProgress | Jaune |
| ✗ | Canceled | Rouge |
| ○ | NotStarted | Blanc |

### Couleurs de priorité

| Couleur | Priorité |
|---------|----------|
| 🔴 Rouge | HIGH |
| 🟡 Jaune | MEDIUM |
| 🟢 Vert | LOW |

## Comportement

1. **Récupération**: Les tâches sont récupérées de la base de données SQLite
2. **Filtrage**: Optionnellement filtrées par statut(s)
3. **Tri**: Affichées dans l'ordre de création décroissant (plus récentes en premier)
4. **Affichage**: Chaque tâche est affichée avec ses détails complets

## Exemples de sortie

### Exemple 1: Pas de tâches
```
Aucune tâche trouvée.
```

### Exemple 2: Liste complète
```
3 tâche(s) trouvée(s):

○  [1] Tâche simple (ID: 85416604-0071-46ab-b76c-601a3df0f308) (MEDIUM)
     Échéance: 2026-01-30

✓  [2] Tâche complétée (ID: abc123...) (HIGH)
     Tags: done, archived
     Échéance: 2026-01-29

⚙  [3] Tâche en cours (ID: def456...) (HIGH)
     Tags: wip, important
     Échéance: 2026-02-15
```

### Exemple 3: Affichage avec tags
```
2 tâche(s) trouvée(s):

○  [1] Implémenter la recherche (ID: 671bc182-...) (HIGH)
     Tags: feature, backend, database
     Échéance: 2026-02-15

○  [2] Bugfix critère de recherche (ID: 6e3d1364-...) (LOW)
     Tags: bug, search
     Échéance: 2026-01-30
```

## Exemples d'utilisation

### Exemple 1: Afficher toutes les tâches
```bash
$ task-cli list
```

### Exemple 2: Afficher les tâches complétées
```bash
$ task-cli list --status completed
```

### Exemple 3: Afficher les tâches en cours et non commencées
```bash
$ task-cli list --status inprogress --status notstarted
```

### Exemple 4: Afficher les tâches annulées
```bash
$ task-cli list -s canceled
```

## Gestion des erreurs

### Erreur d'ouverture de base de données
```
Erreur: Impossible d'ouvrir la base de données: [message d'erreur]
```

### Erreur de statut invalide
Le filtre invalide est ignoré silencieusement

## Tri et ordre

Les tâches sont toujours affichées dans l'ordre suivant:
1. **Trier par**: Date de création (descendant - plus récentes en premier)
2. **Numérotation**: De 1 à N (pour référence dans d'autres commandes)

## Localisation de la base de données

La base de données est stockée dans le répertoire de données standard de l'utilisateur:

- **Linux/macOS**: `~/.local/share/task-cli/tasks.db`
- **Windows**: `%APPDATA%\task-cli\data\tasks.db`

## Notes techniques

- Les tâches sans statut défini (statut = None) sont affichées avec le symbole ○
- Les tags sont affichés dans l'ordre de stockage (qui peut varier)
- Le numéro affiché [N] est juste un numéro séquentiel pour la sortie actuelle
- L'UUID complet est affiché pour permettre l'utilisation dans d'autres commandes

## Cas d'usage

✅ Obtenir une vue d'ensemble de toutes les tâches
✅ Voir les tâches en attente
✅ Filtrer les tâches complétées
✅ Afficher les tâches actuellement en cours
✅ Voir les tâches avec toutes leurs métadonnées (priorité, tags, dates)
