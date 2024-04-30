
use crate::{external::{load_serialized, store}, task::TaskEnum,};


pub async fn store_tasks(tasks: Vec<TaskEnum>) -> Result<(), String> {
    store("tasks", &tasks).await
}
pub async fn load_tasks() -> Result<Vec<TaskEnum>, String> {
    let loaded = load_serialized("tasks").await?;
    serde_json::from_str(&loaded).map_err(|e| e.to_string())
}
