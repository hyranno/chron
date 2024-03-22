use futures::executor::block_on;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{js_sys::Promise, JsFuture};


#[wasm_bindgen]
extern {
    pub fn alert (s: &str);

    #[wasm_bindgen(js_namespace=console)]
    pub fn log (s: &str);

    #[wasm_bindgen(js_name=openWorkingTab)]
    fn open_working_tab(url: &str) -> Promise;
    #[wasm_bindgen(js_name=closeTab)]
    fn close_tab(tab: JsValue) -> Promise;
    #[wasm_bindgen(js_name=fetchStringByXpath)]
    fn fetch_string_by_xpath(tab: &JsValue, xpath: &str) -> Promise;
    #[wasm_bindgen(js_name=addToReadingList)]
    fn add_to_reading_list(tab: &JsValue);
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
