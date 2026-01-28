use crate::task::{Priority, Status, Task};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Result as SqlResult, params};
use std::path::PathBuf;
use uuid::Uuid;

/// Gestionnaire de stockage des tasks dans SQLite
pub struct TaskStorage {
    #[allow(dead_code)]
    db_path: PathBuf,
    conn: Connection,
}

impl TaskStorage {
    /// Crée ou ouvre une base de données SQLite
    pub fn new(db_path: PathBuf) -> SqlResult<Self> {
        let conn = Connection::open(&db_path)?;
        let storage = TaskStorage { db_path, conn };
        storage.init_db()?;
        Ok(storage)
    }

    /// Initialise les tables si elles n'existent pas
    fn init_db(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                priority TEXT NOT NULL,
                status TEXT,
                due_date TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS task_tags (
                task_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (task_id, tag),
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
            );",
        )?;
        Ok(())
    }

    /// Ajoute une nouvelle task
    pub fn add_task(&mut self, task: &Task) -> SqlResult<()> {
        let priority_str = priority_to_string(&task.priority);
        let status_str = task.status.as_ref().map(status_to_string);

        self.conn.execute(
            "INSERT INTO tasks (id, description, priority, status, due_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task.id.to_string(),
                &task.description,
                priority_str,
                status_str,
                task.due_date.to_rfc3339(),
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
            ],
        )?;

        // Ajouter les tags si présents
        if let Some(tags) = &task.tags {
            for tag in tags {
                self.conn.execute(
                    "INSERT INTO task_tags (task_id, tag) VALUES (?1, ?2)",
                    params![task.id.to_string(), tag],
                )?;
            }
        }

        Ok(())
    }

    /// Récupère une task par son ID
    pub fn get_task(&self, id: &Uuid) -> SqlResult<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, priority, status, due_date, created_at, updated_at
             FROM tasks WHERE id = ?1",
        )?;

        let task = stmt
            .query_row(params![id.to_string()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                ))
            })
            .optional()?;

        if let Some((
            id_str,
            description,
            priority_str,
            status_str,
            due_date_str,
            created_at_str,
            updated_at_str,
        )) = task
        {
            let priority = string_to_priority(&priority_str);
            let status = status_str.as_ref().map(|s| string_to_status(s));
            let due_date = DateTime::parse_from_rfc3339(&due_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let task_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());

            // Récupérer les tags
            let tags = self.get_tags(&task_id)?;

            Ok(Some(Task {
                id: task_id,
                description,
                tags,
                status,
                priority,
                due_date,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Récupère toutes les tasks
    pub fn get_all_tasks(&self) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, priority, status, due_date, created_at, updated_at
             FROM tasks ORDER BY created_at DESC",
        )?;

        let tasks = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut result = Vec::new();
        for task_row in tasks {
            let (
                id_str,
                description,
                priority_str,
                status_str,
                due_date_str,
                created_at_str,
                updated_at_str,
            ) = task_row?;

            let priority = string_to_priority(&priority_str);
            let status = status_str.as_ref().map(|s| string_to_status(s));
            let due_date = DateTime::parse_from_rfc3339(&due_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let task_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());

            // Récupérer les tags
            let tags = self.get_tags(&task_id)?;

            result.push(Task {
                id: task_id,
                description,
                tags,
                status,
                priority,
                due_date,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }

    /// Met à jour une task existante
    pub fn update_task(&mut self, task: &Task) -> SqlResult<()> {
        let priority_str = priority_to_string(&task.priority);
        let status_str = task.status.as_ref().map(status_to_string);

        self.conn.execute(
            "UPDATE tasks SET description = ?1, priority = ?2, status = ?3, due_date = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                &task.description,
                priority_str,
                status_str,
                task.due_date.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.id.to_string(),
            ],
        )?;

        // Mettre à jour les tags
        self.conn.execute(
            "DELETE FROM task_tags WHERE task_id = ?1",
            params![task.id.to_string()],
        )?;
        if let Some(tags) = &task.tags {
            for tag in tags {
                self.conn.execute(
                    "INSERT INTO task_tags (task_id, tag) VALUES (?1, ?2)",
                    params![task.id.to_string(), tag],
                )?;
            }
        }

        Ok(())
    }

    /// Supprime une task
    pub fn delete_task(&mut self, id: &Uuid) -> SqlResult<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    /// Récupère les tags d'une task
    fn get_tags(&self, task_id: &Uuid) -> SqlResult<Option<Vec<String>>> {
        let mut stmt = self
            .conn
            .prepare("SELECT tag FROM task_tags WHERE task_id = ?1")?;
        let tags: Vec<String> = stmt
            .query_map(params![task_id.to_string()], |row| row.get(0))?
            .collect::<SqlResult<Vec<String>>>()?;

        Ok(if tags.is_empty() { None } else { Some(tags) })
    }

    /// Récupère les tasks par statut
    pub fn get_tasks_by_status(&self, status: &Status) -> SqlResult<Vec<Task>> {
        let status_str = status_to_string(status);
        let mut stmt = self.conn.prepare(
            "SELECT id, description, priority, status, due_date, created_at, updated_at
             FROM tasks WHERE status = ?1 ORDER BY created_at DESC",
        )?;

        let tasks = stmt.query_map(params![status_str], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut result = Vec::new();
        for task_row in tasks {
            let (
                id_str,
                description,
                priority_str,
                status_str,
                due_date_str,
                created_at_str,
                updated_at_str,
            ) = task_row?;

            let priority = string_to_priority(&priority_str);
            let status = status_str.as_ref().map(|s| string_to_status(s));
            let due_date = DateTime::parse_from_rfc3339(&due_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let task_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let tags = self.get_tags(&task_id)?;

            result.push(Task {
                id: task_id,
                description,
                tags,
                status,
                priority,
                due_date,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }

    /// Récupère les tasks par priorité
    pub fn get_tasks_by_priority(&self, priority: &Priority) -> SqlResult<Vec<Task>> {
        let priority_str = priority_to_string(priority);
        let mut stmt = self.conn.prepare(
            "SELECT id, description, priority, status, due_date, created_at, updated_at
             FROM tasks WHERE priority = ?1 ORDER BY created_at DESC",
        )?;

        let tasks = stmt.query_map(params![priority_str], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut result = Vec::new();
        for task_row in tasks {
            let (
                id_str,
                description,
                priority_str,
                status_str,
                due_date_str,
                created_at_str,
                updated_at_str,
            ) = task_row?;

            let priority = string_to_priority(&priority_str);
            let status = status_str.as_ref().map(|s| string_to_status(s));
            let due_date = DateTime::parse_from_rfc3339(&due_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let task_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let tags = self.get_tags(&task_id)?;

            result.push(Task {
                id: task_id,
                description,
                tags,
                status,
                priority,
                due_date,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }

    /// Récupère les tasks par tag
    pub fn get_tasks_by_tag(&self, tag: &str) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT t.id, t.description, t.priority, t.status, t.due_date, t.created_at, t.updated_at
             FROM tasks t
             INNER JOIN task_tags tt ON t.id = tt.task_id
             WHERE tt.tag = ?1
             ORDER BY t.created_at DESC",
        )?;

        let tasks = stmt.query_map(params![tag], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        let mut result = Vec::new();
        for task_row in tasks {
            let (
                id_str,
                description,
                priority_str,
                status_str,
                due_date_str,
                created_at_str,
                updated_at_str,
            ) = task_row?;

            let priority = string_to_priority(&priority_str);
            let status = status_str.as_ref().map(|s| string_to_status(s));
            let due_date = DateTime::parse_from_rfc3339(&due_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let task_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let tags = self.get_tags(&task_id)?;

            result.push(Task {
                id: task_id,
                description,
                tags,
                status,
                priority,
                due_date,
                created_at,
                updated_at,
            });
        }

        Ok(result)
    }
}

// Fonctions utilitaires de conversion
fn priority_to_string(priority: &Priority) -> &'static str {
    match priority {
        Priority::High => "High",
        Priority::Medium => "Medium",
        Priority::Low => "Low",
    }
}

fn string_to_priority(s: &str) -> Priority {
    match s {
        "High" => Priority::High,
        "Low" => Priority::Low,
        _ => Priority::Medium,
    }
}

fn status_to_string(status: &Status) -> &'static str {
    match status {
        Status::Completed => "Completed",
        Status::NotStarted => "NotStarted",
        Status::InProgress => "InProgress",
        Status::Canceled => "Canceled",
    }
}

fn string_to_status(s: &str) -> Status {
    match s {
        "Completed" => Status::Completed,
        "InProgress" => Status::InProgress,
        "Canceled" => Status::Canceled,
        _ => Status::NotStarted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_db() -> (TaskStorage, PathBuf) {
        let db_path = PathBuf::from(format!("/tmp/test_tasks_{}.db", uuid::Uuid::new_v4()));
        let _ = fs::remove_file(&db_path);
        let storage = TaskStorage::new(db_path.clone()).unwrap();
        (storage, db_path)
    }

    #[test]
    fn test_add_and_get_task() {
        let (mut storage, db_path) = create_test_db();
        let task = Task::new("Test task", None, None, None, None);
        let task_id = task.id;

        storage.add_task(&task).unwrap();
        let retrieved = storage.get_task(&task_id).unwrap();

        assert!(retrieved.is_some());
        let retrieved_task = retrieved.unwrap();
        assert_eq!(retrieved_task.description, "Test task");
        assert_eq!(retrieved_task.priority, Priority::Medium);

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_add_task_with_tags() {
        let (mut storage, db_path) = create_test_db();
        let task = Task::new(
            "Task with tags",
            Some(vec!["work".to_string(), "urgent".to_string()]),
            None,
            None,
            None,
        );
        let task_id = task.id;

        storage.add_task(&task).unwrap();
        let retrieved = storage.get_task(&task_id).unwrap().unwrap();

        // Les tags peuvent être dans n'importe quel ordre dans la base de données
        let mut retrieved_tags = retrieved.tags.unwrap();
        retrieved_tags.sort();
        let mut expected_tags = vec!["work".to_string(), "urgent".to_string()];
        expected_tags.sort();
        assert_eq!(retrieved_tags, expected_tags);

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_update_task() {
        let (mut storage, db_path) = create_test_db();
        let mut task = Task::new("Original description", None, None, None, None);
        let task_id = task.id;

        storage.add_task(&task).unwrap();

        task.set_description("Updated description");
        storage.update_task(&task).unwrap();

        let retrieved = storage.get_task(&task_id).unwrap().unwrap();
        assert_eq!(retrieved.description, "Updated description");

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_delete_task() {
        let (mut storage, db_path) = create_test_db();
        let task = Task::new("Task to delete", None, None, None, None);
        let task_id = task.id;

        storage.add_task(&task).unwrap();
        storage.delete_task(&task_id).unwrap();

        let retrieved = storage.get_task(&task_id).unwrap();
        assert!(retrieved.is_none());

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_get_all_tasks() {
        let (mut storage, db_path) = create_test_db();
        let task1 = Task::new("Task 1", None, None, None, None);
        let task2 = Task::new("Task 2", None, None, None, None);

        storage.add_task(&task1).unwrap();
        storage.add_task(&task2).unwrap();

        let all_tasks = storage.get_all_tasks().unwrap();
        assert_eq!(all_tasks.len(), 2);

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_get_tasks_by_status() {
        let (mut storage, db_path) = create_test_db();

        let mut task1 = Task::new("Task 1", None, None, None, None);
        task1.set_status(Some(Status::Completed));

        let mut task2 = Task::new("Task 2", None, None, None, None);
        task2.set_status(Some(Status::InProgress));

        let task3 = Task::new("Task 3", None, None, None, None);

        storage.add_task(&task1).unwrap();
        storage.add_task(&task2).unwrap();
        storage.add_task(&task3).unwrap();

        let completed = storage.get_tasks_by_status(&Status::Completed).unwrap();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].description, "Task 1");

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_get_tasks_by_priority() {
        let (mut storage, db_path) = create_test_db();

        let mut task1 = Task::new("Task 1", None, None, None, None);
        task1.set_priority(Priority::High);

        let mut task2 = Task::new("Task 2", None, None, None, None);
        task2.set_priority(Priority::Low);

        let mut task3 = Task::new("Task 3", None, None, None, None);
        // task3 garde la priorité par défaut: Medium
        task3.set_priority(Priority::Medium);

        storage.add_task(&task1).unwrap();
        storage.add_task(&task2).unwrap();
        storage.add_task(&task3).unwrap();

        let high_priority = storage.get_tasks_by_priority(&Priority::High).unwrap();
        assert_eq!(high_priority.len(), 1);
        assert_eq!(high_priority[0].description, "Task 1");

        let low_priority = storage.get_tasks_by_priority(&Priority::Low).unwrap();
        assert_eq!(low_priority.len(), 1);
        assert_eq!(low_priority[0].description, "Task 2");

        let medium_priority = storage.get_tasks_by_priority(&Priority::Medium).unwrap();
        assert_eq!(medium_priority.len(), 1);

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn test_get_tasks_by_tag() {
        let (mut storage, db_path) = create_test_db();

        let task1 = Task::new("Task 1", Some(vec!["work".to_string()]), None, None, None);
        let task2 = Task::new(
            "Task 2",
            Some(vec!["personal".to_string()]),
            None,
            None,
            None,
        );
        let task3 = Task::new(
            "Task 3",
            Some(vec!["work".to_string(), "urgent".to_string()]),
            None,
            None,
            None,
        );

        storage.add_task(&task1).unwrap();
        storage.add_task(&task2).unwrap();
        storage.add_task(&task3).unwrap();

        let work_tasks = storage.get_tasks_by_tag("work").unwrap();
        assert_eq!(work_tasks.len(), 2);

        let personal_tasks = storage.get_tasks_by_tag("personal").unwrap();
        assert_eq!(personal_tasks.len(), 1);

        let _ = fs::remove_file(&db_path);
    }
}
