use futures::{future::join_all, Future, FutureExt, TryFutureExt};
use wasm_bindgen::prelude::*;

mod task;
mod storage;
mod external;

use task::{JsTaskInfo, Task};
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
pub fn dummy_task_infos() -> Promise {    //Promise<Vec<task::JsTaskInfo>>
    console_error_panic_hook::set_once();

    /*
    let future_tasks = async {
        let tasks = storage::dummy_tasks();
        storage::store_tasks(tasks).await.unwrap();
        let tasks = storage::load_tasks().await.unwrap();
        let tasks_jsv = serde_wasm_bindgen::to_value(&tasks).unwrap();
        Ok(tasks_jsv)
    };
    */

    let future_tasks = async {
        let tasks = storage::dummy_tasks();
        let infos: Vec<JsTaskInfo> = join_all(
            tasks.into_iter().map(|mut task| async move {task.run().await.into()})
        ).await;
        serde_wasm_bindgen::to_value(&infos).map_err(|e| JsValue::from_str(&e.to_string()))
    };

    future_to_promise(future_tasks)
}
