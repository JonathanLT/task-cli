# Documentation - Commande `delete`

## Description

La commande `delete` permet de supprimer une tâche de la base de données SQLite. Par défaut, elle demande une confirmation avant de supprimer.

## Syntaxe

```bash
task-cli delete <ID> [OPTIONS]
```

## Arguments

- **`ID`** (obligatoire): L'identifiant UUID de la tâche à supprimer
  - Type: String (UUID)
  - Exemple: `671bc182-7f18-4f8d-a0c3-b29a7e506742`

## Options

### `-f, --force`
Supprimer sans demander de confirmation
- Type: Flag booléen
- Défaut: false (demande confirmation)
- Exemple: `--force` ou `-f`

## Comportement

1. **Validation UUID**: L'ID fourni doit être un UUID valide
2. **Récupération**: La tâche est récupérée pour affichage/confirmation
3. **Confirmation**: Un message de confirmation est affiché (sauf avec `--force`)
   - L'utilisateur doit taper `yes` pour confirmer
   - Toute autre entrée annule la suppression
4. **Suppression**: La tâche est supprimée de la base de données
   - Les tags associés sont supprimés automatiquement (suppression en cascade)
5. **Affichage**: Un message de succès est affiché

## Affichage de confirmation

```
⚠️  Êtes-vous sûr de vouloir supprimer cette tâche?
  Description: Tâche à supprimer
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742

Taper 'yes' pour confirmer:
```

## Affichage de succès

```
✓ Tâche supprimée avec succès!
  Description: Tâche à supprimer
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

## Affichage d'annulation

```
Suppression annulée.
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

### Erreur de suppression
```
Erreur: Impossible de supprimer la tâche: [message d'erreur]
```

## Exemples d'utilisation

### Exemple 1: Suppression avec confirmation
```bash
$ task-cli delete 671bc182-7f18-4f8d-a0c3-b29a7e506742
⚠️  Êtes-vous sûr de vouloir supprimer cette tâche?
  Description: Tâche à supprimer
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742

Taper 'yes' pour confirmer: yes
✓ Tâche supprimée avec succès!
```

### Exemple 2: Suppression annulée
```bash
$ task-cli delete 671bc182-7f18-4f8d-a0c3-b29a7e506742
⚠️  Êtes-vous sûr de vouloir supprimer cette tâche?
  Description: Tâche à supprimer
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742

Taper 'yes' pour confirmer: no
Suppression annulée.
```

### Exemple 3: Suppression sans confirmation (--force)
```bash
$ task-cli delete 671bc182-7f18-4f8d-a0c3-b29a7e506742 --force
✓ Tâche supprimée avec succès!
  Description: Tâche à supprimer
  ID: 671bc182-7f18-4f8d-a0c3-b29a7e506742
```

### Exemple 4: Suppression sans confirmation (flag court)
```bash
$ task-cli delete 671bc182-7f18-4f8d-a0c3-b29a7e506742 -f
✓ Tâche supprimée avec succès!
```

## Workflow typique

```bash
# 1. Lister les tâches pour trouver l'ID
$ task-cli list

# 2. Copier l'ID de la tâche à supprimer
# 3. Supprimer la tâche
$ task-cli delete <ID>
# Confirmez avec 'yes'

# 4. Vérifier la suppression
$ task-cli list
```

## Cas d'utilisation

✅ Supprimer une tâche terminée ou obsolète
✅ Corriger une tâche créée par erreur
✅ Nettoyer la base de données
✅ Supprimer rapidement en batch (avec --force)
✅ Supprimer en mode interactif (sans --force)

## Points importants

### Suppression en cascade
- Les tags associés à la tâche sont supprimés automatiquement
- La base de données reste cohérente
- Les données des autres tâches ne sont pas affectées

### Confirmation de sécurité
- Par défaut, une confirmation est requise
- Prévient les suppressions accidentelles
- L'utilisateur voit la description avant suppression
- Doit taper exactement `yes` (case-sensible)

### UUID sensible à la casse
- L'UUID doit être valide (format UUID v4)
- Il doit correspondre à une tâche existante
- Sensible à la casse

### Suppression définitive
- La suppression est définitive et irréversible
- Pas de corbeille ni d'historique
- Une sauvegarde de la base de données est recommandée avant suppressions en masse

## Notes techniques

- La suppression utilise `TaskStorage::delete_task()`
- Les tags sont supprimés automatiquement (clé étrangère avec cascade)
- La suppression est immédiate (pas de transaction asynchrone)
- La base de données est automatiquement créée si elle n'existe pas

## Combinaison avec d'autres commandes

```bash
# Supprimer toutes les tâches complétées (manuel)
$ task-cli list --status completed
# Puis supprimer une par une

# Supprimer rapidement en batch
$ task-cli delete <ID1> --force
$ task-cli delete <ID2> --force
$ task-cli delete <ID3> --force
```

## Sécurité

- ✅ Validation UUID complète
- ✅ Vérification existence tâche
- ✅ Confirmation avant suppression
- ✅ Messages d'erreur explicites
- ✅ Pas d'injection SQL (prepared statements)
