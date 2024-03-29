use futures::{future::join_all, Future, FutureExt, TryFutureExt};
use wasm_bindgen::prelude::*;

mod task;
mod storage;
mod external;

use task::{JsTaskInfo, Task, TaskEnum};
use wasm_bindgen_futures::{future_to_promise, js_sys::Promise};

#[wasm_bindgen]
pub fn hello_content () {
    external::alert("Hello from the content script!");
}


#[wasm_bindgen]
pub fn hello_background () {
    external::log("Hello from the background script!");
}


#[wasm_bindgen]
pub fn task_infos() -> Promise {    //Promise<Vec<task::JsTaskInfo>>
    console_error_panic_hook::set_once();
    let future_tasks = async {
        storage::load_tasks().await.and_then(|tasks| {
            let infos: Vec<JsTaskInfo> = tasks.iter().map(|task| task.info().into()).collect();
            serde_wasm_bindgen::to_value(&infos).map_err(|e| e.to_string())
        }).map_err(|s| JsValue::from_str(&s))
    };

    future_to_promise(future_tasks)
}


#[wasm_bindgen]
pub fn run_tasks() -> Promise {    // Promise<void>
    console_error_panic_hook::set_once();
    let future_tasks = async {
        let tasks = storage::dummy_tasks();
        let tasks: Vec<TaskEnum> = join_all(
            tasks.into_iter().map(|mut task| async move {task.run().await; task})
        ).await;
        let store_res = storage::store_tasks(tasks).await;
        store_res.map(|_| JsValue::null()).map_err(|e| JsValue::from_str(&e))
    };
    future_to_promise(future_tasks)
}
