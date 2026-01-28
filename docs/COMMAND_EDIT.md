# Documentation - Commande `edit`

## Description

La commande `edit` permet de modifier une tâche existante dans la base de données SQLite.

## Syntaxe

```bash
task-cli edit <ID> [DESCRIPTION] [OPTIONS]
```

## Arguments

- **`ID`** (obligatoire): L'identifiant UUID de la tâche à modifier
  - Type: String (UUID)
  - Exemple: `671bc182-7f18-4f8d-a0c3-b29a7e506742`

- **`DESCRIPTION`** (optionnel): Nouvelle description de la tâche
  - Type: String
  - Si fourni, remplace la description actuelle
  - Si non fourni, conserve la description existante

## Options

### `-t, --tags <TAGS>`
Remplacer les tags de la tâche
- Type: String (optionnel)
- Format: `"tag1, tag2, tag3"`
- Exemple: `--tags "updated,modified,important"`
- **Note**: Remplace complètement les tags existants (ajoute un vide pour supprimer tous les tags)

### `-p, --priority <PRIORITY>`
Modifier la priorité de la tâche
- Type: String (optionnel)
- Valeurs acceptées:
  - `high`: Priorité haute
  - `medium`: Priorité moyenne
  - `low`: Priorité basse
- Exemple: `--priority high`

### `-s, --status <STATUS>`
Modifier le statut de la tâche
- Type: String (optionnel)
- Valeurs acceptées:
  - `completed`: Tâche complétée
  - `notstarted` ou `not_started`: Tâche non commencée
  - `inprogress` ou `in_progress`: Tâche en cours
  - `canceled` ou `cancelled`: Tâche annulée
- Exemple: `--status completed`

### `-d, --due <DUE_DATE>`
Modifier la date limite de la tâche
- Type: String (optionnel)
- Formats acceptés:
  - `YYYY-MM-DD` (exemple: `2026-02-28`)
  - RFC3339 (exemple: `2026-02-28T14:00:00Z`)
- Exemple: `--due "2026-02-28"`

## Comportement

1. **Validation UUID**: L'ID fourni doit être un UUID valide
2. **Récupération**: La tâche est récupérée de la base de données
3. **Modification**: Les champs fournis sont mises à jour
4. **Timestamp**: `updated_at` est automatiquement mis à jour
5. **Persistance**: Les modifications sont sauvegardées dans la base de données
6. **Affichage**: Les détails de la tâche modifiée sont affichés

## Affichage de succès

```
✓ Tâche mise à jour avec succès!
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
  Description: Nouvelle description
  Tags: tag1, tag2
  Priorité: High
  Statut: InProgress
  Échéance: 2026-02-28
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

### Priorité invalide
```
Erreur: Priorité invalide 'invalid'. Utilisez: high, medium ou low
```

### Statut invalide
```
Erreur: Statut invalide 'invalid'. Utilisez: completed, notstarted, inprogress ou canceled
```

### Erreur de format date
```
Erreur: Format de date invalide 'invalid-date'. Utilisez: YYYY-MM-DD ou RFC3339
```

### Erreur de base de données
```
Erreur: Impossible d'ouvrir la base de données: [message d'erreur]
```

## Exemples d'utilisation

### Exemple 1: Modifier uniquement la description
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 "Nouvelle description"
✓ Tâche mise à jour avec succès!
```

### Exemple 2: Modifier uniquement la priorité
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 --priority high
✓ Tâche mise à jour avec succès!
```

### Exemple 3: Modifier le statut et la date
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 \
    --status inprogress \
    --due "2026-02-28"
✓ Tâche mise à jour avec succès!
```

### Exemple 4: Modification complète
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 \
    "Tâche révisée" \
    --priority high \
    --status inprogress \
    --tags "updated,revised" \
    --due "2026-02-28"
✓ Tâche mise à jour avec succès!
```

### Exemple 5: Modifier et supprimer les tags
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 --tags ""
✓ Tâche mise à jour avec succès!
  Tags: (aucun)
```

### Exemple 6: Changer le statut à Completed
```bash
$ task-cli edit 671bc182-7f18-4f8d-a0c3-b29a7e506742 --status completed
✓ Tâche mise à jour avec succès!
  Statut: Completed
```

## Cas d'utilisation

✅ Mettre à jour la description d'une tâche
✅ Changer la priorité d'une tâche urgente
✅ Marquer une tâche comme en cours
✅ Ajouter des tags ou modifier les tags existants
✅ Décaler la date limite
✅ Corriger une tâche mal saisie
✅ Mettre une tâche à jour avec plusieurs modifications

## Points importants

### Modifications partielles
- Seuls les champs fournis sont modifiés
- Les champs non fournis conservent leur valeur actuelle
- **Exception**: Les tags remplacent complètement les tags existants

### Timestamp automatique
- `updated_at` est toujours mis à jour
- Cela permet de suivre quand la tâche a été modifiée pour la dernière fois

### Tags
- Les tags sont remplissés après trimming (espaces avant/après supprimés)
- Les tags vides sont filtrés
- Un tag vide (ou avec uniquement des espaces) supprime tous les tags

### UUID
- L'UUID doit être valide (format UUID v4)
- Il doit correspondre à une tâche existante
- Sensible à la casse

## Intégration avec d'autres commandes

```bash
# 1. Lister les tâches pour trouver l'ID
$ task-cli list

# 2. Copier l'ID de la tâche à modifier
# 3. Utiliser edit pour la modifier
$ task-cli edit <ID> --priority high

# 4. Vérifier le changement avec list
$ task-cli list
```

## Notes techniques

- La commande recrée le timestamp `updated_at`
- Les dates sont converties en UTC
- Les priorités sont case-insensitive
- Les statuts sont case-insensitive
- Les tags sont sensibles à la casse (par défaut)
- La base de données est automatiquement créée si elle n'existe pas

## Limitations actuelles

- Impossible de modifier uniquement un tag sans remplacer les autres
- Impossible de modifier `created_at` (il est permanent)
- Impossible d'ajouter des tags sans remplacer les existants

Ces limitations peuvent être adressées dans une version ultérieure.
