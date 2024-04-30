use async_recursion::async_recursion;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{js_sys::Promise, JsFuture};


#[wasm_bindgen]
extern {
    pub fn alert (s: &str);

    #[wasm_bindgen(js_namespace=console)]
    pub fn log (s: &str);
}

#[wasm_bindgen(module="/ts/index.ts")]
extern {
    fn open_working_tab(url: &str) -> Promise;
    fn close_tab(tab: JsValue) -> Promise;
    fn fetch_string_by_xpath(tab: &JsValue, xpath: &str) -> Promise;
    #[wasm_bindgen(js_name="add_to_reading_list")]
    fn add_to_reading_list_js(url: &str, title: &str) -> Promise;

    fn store_serialized(key: &str, value: &str) -> Promise;
    #[wasm_bindgen(js_name="load_serialized")]
    fn load_serialized_jsv(key: &str) -> Promise;

    pub fn util_alert();
}

pub async fn add_to_reading_list(url: &str, title: &str) -> Result<(), String> {
    JsFuture::from(add_to_reading_list_js(url, title)).await
        .map(|_| ())
        .map_err(|_| String::from("Failed to add to reading list."))
}

pub async fn store(key: &str, value: &impl Serialize) -> Result<(), String> {
    let serialized = serde_json::to_string(value).map_err(|e| e.to_string())?;
    let res = JsFuture::from(store_serialized(key, &serialized)).await;
    res.map(|_| ()).map_err(|_| String::from("Failed to store."))
}
pub async fn load_serialized(key: &str) -> Result<String, String> {
    let loaded_jsv = JsFuture::from(load_serialized_jsv(key)).await.map_err(|_| String::from("Failed to load."))?;
    loaded_jsv.as_string().ok_or(String::from("Loaded value is not string."))
}

pub struct Tab(JsValue);
impl Tab {
    pub async fn open(url: &str) -> Self {
        let jsv = JsFuture::from(open_working_tab(url)).await;
        Tab(jsv.unwrap())
    }
    pub async fn close(self) {
        JsFuture::from(close_tab(self.0)).await.unwrap();
    }
    pub async fn fetch_string_by_xpath(&self, xpath: &str) -> Result<String, String> {
        let jsv = JsFuture::from(fetch_string_by_xpath(&self.0, xpath)).await;
        jsv.ok().and_then(|v| v.as_string()).ok_or(String::from("Failed to fetch."))
    }
    #[async_recursion(?Send)]
    pub async fn fetch_string_by_xpath_w_retry(&self, xpath: &String, count: usize) -> Result<String, String> {
        let fetch_res = self.fetch_string_by_xpath(xpath).await;
        if count < 10 && (fetch_res.is_err() || fetch_res == Ok(String::new())) {
            async_std::task::sleep(std::time::Duration::from_secs_f32(0.5)).await;
            self.fetch_string_by_xpath_w_retry(xpath, count+1).await
        } else {fetch_res}
    }
}
