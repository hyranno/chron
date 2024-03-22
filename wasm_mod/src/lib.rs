use wasm_bindgen::prelude::*;

mod task;
mod storage;
mod external;


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
    storage::dummy_tasks().into_iter().map(|t| t.info().into()).collect()
}
