use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Completed,
    NotStarted,
    InProgress,
    Canceled,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub tags: Option<Vec<String>>,
    pub status: Option<Status>,
    pub priority: Priority,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Crée une nouvelle task.
    /// - `description` obligatoire.
    /// - `tags`, `status`, `priority`, `due_date_opt` sont optionnels.
    ///   - priorité par défaut = `Medium`
    ///   - date de fin par défaut = now + 1 jour
    ///   - pas de tags par défaut (None)
    pub fn new<D: Into<String>>(
        description: D,
        tags: Option<Vec<String>>,
        status: Option<Status>,
        priority: Option<Priority>,
        due_date_opt: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        let due_date = due_date_opt.unwrap_or_else(|| now + Duration::days(1));
        Task {
            id: Uuid::new_v4(),
            description: description.into(),
            tags,
            status,
            priority: priority.unwrap_or_default(),
            due_date,
            created_at: now,
            updated_at: now,
        }
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_description<D: Into<String>>(&mut self, description: D) {
        self.description = description.into();
        self.touch();
    }

    pub fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
        self.touch();
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.touch();
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) {
        self.due_date = due_date;
        self.touch();
    }

    pub fn add_tag<S: Into<String>>(&mut self, tag: S) {
        match &mut self.tags {
            Some(vec) => {
                vec.push(tag.into());
            }
            None => {
                self.tags = Some(vec![tag.into()]);
            }
        }
        self.touch();
    }

    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(vec) = &mut self.tags {
            vec.retain(|t| t != tag);
            if vec.is_empty() {
                self.tags = None;
            }
            self.touch();
        }
    }
}
