
use chrono::{DateTime, TimeDelta, Utc};
use wasm_bindgen::prelude::*;

use crate::external::Tab;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct TaskInfo {
    pub name: String,
    pub next_run: Option<DateTime<Utc>>,
    pub last_result: Option<Result<String, String>>,
}


pub trait Task {
    fn run(&mut self);
    fn info(&self) -> TaskInfo;
}

pub trait Checker {
    fn check(&mut self, tab: &Tab) -> Result<bool, String>;
}

pub struct ChangeChecker {
    xpath: String,
    previous: Option<String>,
}
impl ChangeChecker {
    pub fn new(xpath: &str) -> Self {
        Self {
            xpath: String::from(xpath),
            previous: None,
        }
    }
}
impl Checker for ChangeChecker {
    fn check(&mut self, tab: &Tab) -> Result<bool, String> {
        let fetch_res = tab.fetch_string_by_xpath(&self.xpath);
        fetch_res.map(|fetched| {
            self.previous = Some(fetched.clone());
            if let Some(prev) = self.previous.clone() {
                prev == fetched
            } else {
                true
            }
        })
    }
}

pub trait Planner {
    fn next(&mut self, tab: &Tab) -> DateTime<Utc>;
}

pub struct IntervalPlanner {
    interval: TimeDelta,
    previous: Option<DateTime<Utc>>,
}
impl IntervalPlanner {
    pub fn new(interval: TimeDelta) -> Self {
        Self { interval, previous: None }
    }
}
impl Planner for IntervalPlanner {
    fn next(&mut self, _tab: &Tab) -> DateTime<Utc> {
        let previous = self.previous.unwrap_or(Utc::now());
        let delay = Utc::now() - previous;
        let mut next = previous + self.interval;
        while next < previous + delay {
            next += self.interval;
        }
        self.previous = Some(next);
        next
    }
}


pub struct WatchUpdateTask {
    url: String,
    checker: Box<dyn Checker>,
    planner: Box<dyn Planner>,
    info: TaskInfo,
}
impl WatchUpdateTask {
    pub fn new(name: &str, next_run: Option<DateTime<Utc>>, url: &str, checker: impl Checker + 'static, planner: impl Planner + 'static) -> Self {
        Self {
            url: String::from(url),
            checker: Box::new(checker),
            planner: Box::new(planner),
            info: TaskInfo { name: String::from(name), next_run, last_result: None }
        }
    }
}
impl Task for WatchUpdateTask {
    fn info(&self) -> TaskInfo {
        self.info.clone()
    }
    fn run(&mut self) {
        let tab = Tab::open(&self.url);
        let check_res = self.checker.check(&tab);
        if let Ok(pass_check) = check_res {
            if pass_check { tab.add_to_reading_list() };
            self.info.next_run = Some(self.planner.next(&tab));
        }
        self.info.last_result = Some(check_res.map(|updated| String::from(
            if updated {"updated"} else {"no update"}
        )));
        tab.close();
    }
}
