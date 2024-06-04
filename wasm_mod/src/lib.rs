use futures::{future::join_all, FutureExt};
use wasm_bindgen::prelude::*;

pub mod task;
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
pub fn update_tasks() -> Promise {    // Promise<void>
    console_error_panic_hook::set_once();
    let future_tasks = async {
        // let tasks = storage::dummy_tasks();
        let tasks = storage::load_tasks().await
            .map_err(|e| JsValue::from_str(&e))?
        ;
        let tasks: Vec<TaskEnum> = join_all(
            tasks.into_iter().map(|mut task| async move {task.update().await; task})
        ).await;
        let store_res = storage::store_tasks(tasks).await;
        store_res.map(|_| JsValue::null()).map_err(|e| JsValue::from_str(&e))
    };
    future_to_promise(future_tasks)
}


#[wasm_bindgen]
pub fn load_tasks_json() -> Promise {     // Promise<string>
    console_error_panic_hook::set_once();
    let future_json = async {
        storage::load_serialized_tasks().await.map(|s| JsValue::from_str(&s)).map_err(|s| JsValue::from_str(&s))
    };
    future_to_promise(future_json)
}

#[wasm_bindgen]
pub fn store_tasks_json(json_str: String) -> Promise {     // Promise<void>
    console_error_panic_hook::set_once();
    let tasks: Result<Vec<TaskEnum>, String> = serde_json::from_str(&json_str).map_err(|e| e.to_string());
    let future_result = async {
        let store_res = if let Ok(tasks) = tasks {
            storage::store_tasks(tasks).await
        } else {
            Err(tasks.err().unwrap())
        };
        store_res.map(|_| JsValue::null()).map_err(|s| JsValue::from_str(&s))
    };
    future_to_promise(future_result)
}

#[wasm_bindgen]
pub fn run_task(name: String) -> Promise {
    console_error_panic_hook::set_once();
    let future_tasks = modify_task(name, |task| {async{task.run().await};})
        .map(|r| r.map(|_| JsValue::null()).map_err(|e| JsValue::from_str(&e)))
    ;
    future_to_promise(future_tasks)
}

#[wasm_bindgen]
pub fn unset_task(name: String) -> Promise {
    console_error_panic_hook::set_once();
    let future_tasks = modify_task(name, |task| task.unset())
        .map(|r| r.map(|_| JsValue::null()).map_err(|e| JsValue::from_str(&e)))
    ;
    future_to_promise(future_tasks)
}

async fn modify_task(name: String, f: impl FnOnce(&mut TaskEnum)) -> Result<(), String> {
    let mut tasks = storage::load_tasks().await?;
    let task = tasks.iter_mut().find(|t| t.info().name == name).ok_or(String::from("invalid name"))?;
    f(task);
    let store_res = storage::store_tasks(tasks).await;
    store_res
}
