
use chrono::{DateTime, Utc};
use futures::Future;
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;
use wasm_bindgen::prelude::*;

pub mod conditional;
use conditional::ConditionalTask;

/*
 * Naive json cant hold the type and `Box<dyn Trait>` cannot be handled.
 * `typetag` or `enum_dispatch` can add type info to json.
 * `typetag` does not work with wasm.
 * `enum_dispatch` does not work with fn returning `impl Trait` for now. (https://gitlab.com/antonok/enum_dispatch/-/issues/75)
 * So, write `enum_dispatch` by hands.
 */

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
    fn unset(&mut self);
    fn info(&self) -> TaskInfo;
}
#[enum_dispatch]
#[derive(Serialize, Deserialize)]
pub enum TaskEnum {
    ConditionalTask,
}

