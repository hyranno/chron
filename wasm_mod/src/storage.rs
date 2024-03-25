use chrono::TimeDelta;

use crate::{external::{load_serialized, store}, task::{ChangeChecker, IntervalPlanner, TaskEnum, WatchUpdateTask}};


pub async fn store_tasks(tasks: Vec<TaskEnum>) -> Result<(), String> {
    store("tasks", &tasks).await
}
pub async fn load_tasks() -> Result<Vec<TaskEnum>, String> {
    let loaded = load_serialized("tasks").await?;
    serde_json::from_str(&loaded).map_err(|e| e.to_string())
}


pub fn dummy_tasks() -> Vec<TaskEnum> {
    vec![
        WatchUpdateTask::new(
            "time0",
            None,
            "https://www.time-j.net/worldtime/country/jp",
            ChangeChecker::new("string(//*[@id=\"currentTime\"])"),
            IntervalPlanner::new(TimeDelta::try_minutes(1).unwrap())
        ).into(),
        WatchUpdateTask::new(
            "time1",
            None,
            "https://time.is/ja/",
            ChangeChecker::new("string(//*[@id=\"clock\"])"),
            IntervalPlanner::new(TimeDelta::try_minutes(1).unwrap())
        ).into(),
    ]
}
