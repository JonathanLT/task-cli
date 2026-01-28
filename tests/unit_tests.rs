use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

// Tests d'intégration pour Task CLI
// Ces tests vérifient que les commandes fonctionnent correctement avec la persistance SQLite

fn setup_test_env() {
    // Utiliser un répertoire temporaire pour la base de données de test
    unsafe {
        env::set_var("HOME", "/tmp/task-cli-test");
    }
}

#[test]
fn test_add_command_creates_task() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("add")
        .arg("Ma tâche de test")
        .arg("-t")
        .arg("home,urgent")
        .arg("-p")
        .arg("high")
        .arg("-d")
        .arg("2026-02-15");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Tâche ajoutée avec succès"))
        .stdout(predicate::str::contains("Ma tâche de test"))
        .stdout(predicate::str::contains("High"))
        .stdout(predicate::str::contains("2026-02-15"));
}

#[test]
fn test_add_command_with_minimal_args() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("add").arg("Tâche simple");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Tâche ajoutée avec succès"))
        .stdout(predicate::str::contains("Tâche simple"));
}

#[test]
fn test_add_command_with_invalid_priority() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("add").arg("Test").arg("-p").arg("invalid");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Priorité invalide"));
}

#[test]
fn test_list_command_shows_tasks() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("list");

    cmd.assert().success().stdout(
        predicate::str::contains("tâche(s) trouvée(s)")
            .or(predicate::str::contains("Aucune tâche trouvée")),
    );
}

#[test]
fn test_list_command_with_status_filter() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("list").arg("--status").arg("completed");

    cmd.assert().success().stdout(
        predicate::str::contains("tâche(s) trouvée(s)")
            .or(predicate::str::contains("Aucune tâche trouvée")),
    );
}

#[test]
fn test_search_command_with_pattern() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("search").arg("test");

    cmd.assert().success().stdout(
        predicate::str::contains("tâche(s) trouvée(s)")
            .or(predicate::str::contains("Aucune tâche trouvée")),
    );
}

#[test]
fn test_search_command_with_filters() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("search")
        .arg("important")
        .arg("-t")
        .arg("work")
        .arg("-p")
        .arg("high");

    cmd.assert().success().stdout(
        predicate::str::contains("tâche(s) trouvée(s)")
            .or(predicate::str::contains("Aucune tâche trouvée")),
    );
}

#[test]
fn test_delete_command_with_invalid_uuid() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("delete").arg("invalid-uuid").arg("--force");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("UUID invalide"));
}

#[test]
fn test_complete_command_with_invalid_uuid() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("complete").arg("not-a-uuid");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("UUID invalide"));
}

#[test]
fn test_edit_command_with_invalid_uuid() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("edit").arg("bad-uuid").arg("New description");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("UUID invalide"));
}

#[test]
fn test_workflow_add_list_delete() {
    setup_test_env();

    // 1. Ajouter une tâche
    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("add")
        .arg("Test workflow task")
        .arg("-p")
        .arg("high");

    let output = cmd.assert().success();
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Extraire l'ID (format: "ID: <uuid>")
    if let Some(id_line) = stdout.lines().find(|l| l.contains("ID:")) {
        let id = id_line.split_whitespace().last().unwrap_or("");

        // 2. Vérifier que la tâche apparaît dans list
        let mut cmd = Command::cargo_bin("task-cli").unwrap();
        cmd.arg("list");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Test workflow task"));

        // 3. Supprimer la tâche
        let mut cmd = Command::cargo_bin("task-cli").unwrap();
        cmd.arg("delete").arg(id).arg("--force");
        cmd.assert().success();
    }
}

#[test]
fn test_add_command_with_medium_priority() {
    setup_test_env();

    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.arg("add")
        .arg("Task with medium priority")
        .arg("-p")
        .arg("medium");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Tâche ajoutée avec succès"))
        .stdout(predicate::str::contains("Medium"));
}

#[test]
fn test_cli_requires_subcommand() {
    let mut cmd = Command::cargo_bin("task-cli").unwrap();
    cmd.assert().failure();
}
