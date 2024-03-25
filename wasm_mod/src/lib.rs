use futures::{Future, FutureExt, TryFutureExt};
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

    let future_tasks = async {
        let tasks = storage::dummy_tasks();
        storage::store_tasks(tasks).await.unwrap();
        let tasks = storage::load_tasks().await.unwrap();
        let tasks_jsv = serde_wasm_bindgen::to_value(&tasks).unwrap();
        Ok(tasks_jsv)
    };

    /*
    let future_tasks = storage::store_tasks(tasks).and_then(|_|
        storage::load_tasks()
    ).map_ok(|tasks|
        tasks.into_iter().map(|t| t.info().into()).collect()
    ).map(|r|
        r.and_then(|v: Vec<task::JsTaskInfo>|
            serde_wasm_bindgen::to_value(&v).map_err(|e| e.to_string())
        ).map_err(|e| JsValue::from(e))
    );
    */

    future_to_promise(future_tasks)
}
