
use chrono::{DateTime, TimeDelta, Utc};
use futures::Future;
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;
use wasm_bindgen::prelude::*;

use crate::external::Tab;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsTaskInfo {
    pub name: String,
    pub next_run: Option<String>,   // rfc_2822
    pub err: Option<String>,
    pub last_result: Option<String>,
}
impl From<TaskInfo> for JsTaskInfo {
    fn from(value: TaskInfo) -> Self {
        Self {
            name: value.name,
            next_run: value.next_run.map(|date| date.to_rfc2822()),
            err: value.last_result.clone().and_then(|res| res.err()),
            last_result: value.last_result.and_then(|res| res.ok()),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub name: String,
    pub next_run: Option<DateTime<Utc>>,
    pub last_result: Option<Result<String, String>>,
}


#[enum_dispatch(TaskEnum)]
pub trait Task {
    fn update(&mut self) -> impl Future<Output = TaskInfo>;
    fn run(&mut self) -> impl Future<Output = TaskInfo>;
    fn info(&self) -> TaskInfo;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum TaskEnum {
    ConditionalTask,
}

#[enum_dispatch(ConditionEnum)]
pub trait Checker {
    fn check(&mut self) -> impl Future<Output = Result<bool, String>>;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum ConditionEnum {
    ChangeCondition,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeCondition {
    url: String,
    xpath: String,
    previous: Option<String>,
}
impl ChangeCondition {
    pub fn new(url: &str, xpath: &str) -> Self {
        Self {
            url: String::from(url),
            xpath: String::from(xpath),
            previous: None,
        }
    }
}
impl Checker for ChangeCondition {
    async fn check(&mut self) -> Result<bool, String> {
        let tab = Tab::open(&self.url).await;
        let fetch_res = tab.fetch_string_by_xpath_w_retry(&self.xpath, 0).await;
        tab.close().await;
        fetch_res.map(|fetched| {
            let res = if let Some(prev) = self.previous.clone() {
                prev != fetched
            } else {
                true
            };
            self.previous = Some(fetched.clone());
            res
        })
    }
}

#[enum_dispatch(SchedulerEnum)]
pub trait Planner {
    fn next(&mut self) -> impl Future<Output = DateTime<Utc>>;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum SchedulerEnum {
    IntervalScheduler,
}


#[derive(Serialize, Deserialize)]
pub struct IntervalScheduler {
    interval_seconds: i64,  // TimeDelta is not Serializable
    previous: Option<DateTime<Utc>>,
}
impl IntervalScheduler {
    pub fn new(interval: TimeDelta) -> Self {
        Self { interval_seconds: interval.num_seconds(), previous: None }
    }
}
impl Planner for IntervalScheduler {
    async fn next(&mut self) -> DateTime<Utc> {
        let previous = self.previous.unwrap_or(Utc::now());
        let delay = Utc::now() - previous;
        let interval = TimeDelta::try_seconds(self.interval_seconds).unwrap();
        let mut next = previous + interval;
        while next < previous + delay {
            next += interval;
        }
        self.previous = Some(next);
        next
    }
}


#[enum_dispatch(ActionEnum)]
pub trait Action {
    fn run(&mut self) -> impl Future<Output = Result<(), String>>;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum ActionEnum {
    AddReadingListAction,
}

#[derive(Serialize, Deserialize)]
pub struct AddReadingListAction {
    url: String,
    title: String,
}
impl AddReadingListAction {
    pub fn new(url: &str, title: &str) -> Self {
        Self { url: String::from(url), title: String::from(title) }
    }
}
impl Action for AddReadingListAction {
    async fn run(&mut self) -> Result<(),String> {
        #[allow(unused_must_use)]
        {crate::external::add_to_reading_list(&self.url, &self.title).await;}  // May fail if the entry exists.
        Ok(())
    }
}



#[derive(Serialize, Deserialize)]
pub struct ConditionalTask {
    checker: ConditionEnum,
    planner: SchedulerEnum,
    action: ActionEnum,
    info: TaskInfo,
}
impl ConditionalTask {
    pub fn new(
        name: &str,
        next_run: Option<DateTime<Utc>>,
        checker: impl Into<ConditionEnum>,
        planner: impl Into<SchedulerEnum>,
        action: impl Into<ActionEnum>,
    ) -> Self {
        Self {
            checker: checker.into(),
            planner: planner.into(),
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
        let cond_res = self.checker.check().await;
        self.info.last_result = Some(match cond_res {
            Err(e) => Err(e),
            Ok(false) => Ok(String::from("condition mismatch")),
            Ok(true) => {
                let action_res = self.action.run().await;
                if action_res.is_ok() {
                    self.info.next_run = Some(self.planner.next().await);
                }
                action_res.map(|_| String::from("run"))
            },
        });
        self.info()
    }
}
