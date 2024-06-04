
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use enum_dispatch::enum_dispatch;

pub mod condition;
pub mod scheduler;
pub mod action;

use super::{Task, TaskInfo};
use condition::{Condition, ConditionEnum};
use scheduler::{Scheduler, SchedulerEnum};
use action::{Action, ActionEnum};

#[derive(Serialize, Deserialize)]
pub struct ConditionalTask {
    condition: ConditionEnum,
    scheduler: SchedulerEnum,
    action: ActionEnum,
    info: TaskInfo,
}
impl ConditionalTask {
    pub fn new(
        name: &str,
        next_run: Option<DateTime<Utc>>,
        condition: impl Into<ConditionEnum>,
        scheculer: impl Into<SchedulerEnum>,
        action: impl Into<ActionEnum>,
    ) -> Self {
        Self {
            condition: condition.into(),
            scheduler: scheculer.into(),
            action: action.into(),
            info: TaskInfo { name: String::from(name), next_run, last_result: None }
        }
    }
}
impl Task for ConditionalTask {
    fn info(&self) -> TaskInfo {
        self.info.clone()
    }
    async fn update(&mut self) -> TaskInfo {
        let is_scheduled = self.info().next_run.and_then(|next_run|
            if Utc::now() < next_run {None} else {Some(())}
        ).is_some();
        if is_scheduled {
            self.run().await
        } else {
            self.info()
        }
    }
    async fn run(&mut self) -> TaskInfo {
        let res = match self.condition.check().await {
            Err(e) => Err(e),
            Ok(false) => Ok(String::from("condition mismatch")),
            Ok(true) => self.action.run().await.map(|_| String::from("run")),
        };
        if res.is_ok() {
            self.info.next_run = self.scheduler.next().await;
        }
        self.info.last_result = Some(res);
        self.info()
    }
    fn unset(&mut self) {
        self.info.next_run = None;
    }
}
