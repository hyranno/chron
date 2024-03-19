use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert (s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log (s: &str);
}


#[wasm_bindgen]
pub fn hello_content () {
    alert("Hello from the content script!");
}


#[wasm_bindgen]
pub fn hello_background () {
    log("Hello from the background script!");
}
