use futures::executor::block_on;
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
    fn add_to_reading_list(tab: &JsValue);

    fn store_serialized(key: &str, value: &str) -> Promise;
    #[wasm_bindgen(js_name="load_serialized")]
    fn load_serialized_jsv(key: &str) -> Promise;

    pub fn util_alert();
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
    pub fn open(url: &str) -> Self {
        let jsv = block_on(JsFuture::from(open_working_tab(url)));
        Tab(jsv.unwrap())
    }
    pub fn close(self) {
        block_on(JsFuture::from(close_tab(self.0))).unwrap();
    }
    pub fn fetch_string_by_xpath(&self, xpath: &str) -> Result<String, String> {
        let jsv = block_on(JsFuture::from(fetch_string_by_xpath(&self.0, xpath)));
        jsv.ok().and_then(|v| v.as_string()).ok_or(String::from("Failed to fetch."))
    }
    pub fn add_to_reading_list(&self) {
        add_to_reading_list(&self.0)
    }
}
