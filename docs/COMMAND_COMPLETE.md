# Documentation - Commande `complete`

## Description

La commande `complete` permet de marquer rapidement une tâche comme complétée. C'est un raccourci pour `edit <ID> --status completed`.

## Syntaxe

```bash
task-cli complete <ID>
```

## Arguments

- **`ID`** (obligatoire): L'identifiant UUID de la tâche à marquer comme complétée
  - Type: String (UUID)
  - Exemple: `671bc182-7f18-4f8d-a0c3-b29a7e506742`

## Comportement

1. **Validation UUID**: L'ID fourni doit être un UUID valide
2. **Récupération**: La tâche est récupérée de la base de données
3. **Vérification**: Vérifie si la tâche est déjà complétée
4. **Modification**: Change le statut à `Completed`
5. **Timestamp**: `updated_at` est automatiquement mis à jour
6. **Persistance**: Les modifications sont sauvegardées dans la base de données
7. **Affichage**: Les détails de la tâche complétée sont affichés

## Affichage de succès

```
✓ Tâche marquée comme complétée!
  Description: Ma tâche importante
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
  Tags: work, urgent
  Priorité: High
  Échéance: 2026-02-15
```

## Affichage déjà complétée

```
⚠️  Cette tâche est déjà complétée!
  Description: Ma tâche
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

## Gestion des erreurs

### UUID invalide
```
Erreur: UUID invalide: 'invalid-uuid'
```

### Tâche introuvable
```
Erreur: Tâche introuvable avec l'ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

### Erreur de base de données
```
Erreur: Impossible d'ouvrir la base de données: [message d'erreur]
```

### Erreur de récupération
```
Erreur: Impossible de récupérer la tâche: [message d'erreur]
```

### Erreur de mise à jour
```
Erreur: Impossible de mettre à jour la tâche: [message d'erreur]
```

## Exemples d'utilisation

### Exemple 1: Marquer une tâche comme complétée
```bash
$ task-cli complete 671bc182-7f18-4f8d-a0c3-b29a7e506742
✓ Tâche marquée comme complétée!
  Description: Implémenter la fonctionnalité X
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
  Priorité: High
  Échéance: 2026-02-15
```

### Exemple 2: Tâche déjà complétée
```bash
$ task-cli complete 671bc182-7f18-4f8d-a0c3-b29a7e506742
⚠️  Cette tâche est déjà complétée!
  Description: Implémenter la fonctionnalité X
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

### Exemple 3: Workflow complet
```bash
# 1. Lister les tâches
$ task-cli list

# 2. Copier l'ID d'une tâche
# 3. La marquer comme complétée
$ task-cli complete 671bc182-7f18-4f8d-a0c3-b29a7e506742

# 4. Vérifier avec list (symbole ✓)
$ task-cli list
✓  [1] Implémenter la fonctionnalité X (ID: 671bc182-...) (HIGH)
```

## Cas d'utilisation

✅ Marquer rapidement une tâche comme terminée
✅ Workflow plus rapide que `edit --status completed`
✅ Voir immédiatement les détails de la tâche complétée
✅ Prévenir la double completion
✅ Suivre les tâches terminées

## Points importants

### Symbole dans list
- Après completion, la tâche affiche le symbole ✓ (vert) dans `task-cli list`
- Facilite l'identification visuelle des tâches terminées

### Timestamp automatique
- `updated_at` est automatiquement mis à jour
- Permet de savoir quand la tâche a été complétée

### Idempotence
- Marquer une tâche déjà complétée ne génère pas d'erreur
- Affiche simplement un message d'avertissement
- Ne modifie pas la tâche

### UUID
- L'UUID doit être valide (format UUID v4)
- Il doit correspondre à une tâche existante
- Sensible à la casse

## Intégration avec d'autres commandes

```bash
# Workflow typique
$ task-cli add "Ma nouvelle tâche"
$ task-cli list
$ task-cli complete <ID>
$ task-cli list --status completed
```

## Comparaison avec edit

### Avec complete (raccourci)
```bash
$ task-cli complete 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

### Avec edit (équivalent)
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 --status completed
```

**Avantage de complete**:
- Plus rapide à taper
- Intention claire (marquer comme terminée)
- Évite les erreurs de typo sur "completed"

## Notes techniques

- La commande utilise `TaskStorage::get_task()` et `TaskStorage::update_task()`
- Le statut est changé à `Status::Completed`
- Les autres attributs restent inchangés
- La base de données est automatiquement créée si elle n'existe pas

## Filtrage des tâches complétées

```bash
# Voir toutes les tâches complétées
$ task-cli list --status completed

# Voir toutes les tâches non complétées
$ task-cli list --status notstarted --status inprogress
```

## Cas d'usage avancés

### Marquer plusieurs tâches
```bash
# En séquence
$ task-cli complete <ID1>
$ task-cli complete <ID2>
$ task-cli complete <ID3>

# Avec script
for id in ID1 ID2 ID3; do
  task-cli complete $id
done
```

### Workflow GTD (Getting Things Done)
```bash
# 1. Lister les tâches en cours
$ task-cli list --status inprogress

# 2. Compléter les tâches terminées
$ task-cli complete <ID>

# 3. Vérifier les progrès
$ task-cli list --status completed
```

## Limitations

- Impossible de "dé-compléter" une tâche avec cette commande
- Pour changer le statut à autre chose que "Completed", utiliser `edit --status`
- Pas de bulk completion (une tâche à la fois)

## Sécurité

- ✅ Validation UUID complète
- ✅ Vérification existence tâche
- ✅ Pas d'injection SQL (prepared statements)
- ✅ Messages d'erreur explicites
