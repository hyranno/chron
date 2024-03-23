use chrono::TimeDelta;

use crate::task::{ChangeChecker, IntervalPlanner, TaskEnum, WatchUpdateTask};


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
