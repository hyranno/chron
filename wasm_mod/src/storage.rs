
use crate::{external::{load_serialized, store}, task::TaskEnum,};

pub fn task_key() -> String {String::from("tasks")}
pub async fn store_tasks(tasks: Vec<TaskEnum>) -> Result<(), String> {
    store(&task_key(), &tasks).await
}
pub async fn load_tasks() -> Result<Vec<TaskEnum>, String> {
    let loaded = load_serialized_tasks().await?;
    serde_json::from_str(&loaded).map_err(|e| e.to_string())
}
pub async fn load_serialized_tasks() -> Result<String, String> {
    load_serialized(&task_key()).await
}
