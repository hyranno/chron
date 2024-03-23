use wasm_bindgen::prelude::*;

mod task;
mod storage;
mod external;

use task::{Task, TaskEnum,};

#[wasm_bindgen]
pub fn hello_content () {
    external::alert("Hello from the content script!");
}


#[wasm_bindgen]
pub fn hello_background () {
    external::log("Hello from the background script!");
}


#[wasm_bindgen]
pub fn dummy_task_infos() -> Vec<task::JsTaskInfo> {
    console_error_panic_hook::set_once();

    let json_str = serde_json::to_string(&storage::dummy_tasks()).unwrap();
    let val: Vec<TaskEnum> = serde_json::from_str(&json_str).unwrap();

    val.into_iter().map(|t| t.info().into()).collect()
}
