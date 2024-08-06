use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PrivateTask {
    pub id: String,
    pub name: String,
}

impl From<Task> for PrivateTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id.id.to_raw(),
            name: value.name,
        }
    }
}
