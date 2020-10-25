use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

#[wasm_bindgen(start)]
pub fn main() {
    log_1(&JsValue::from("Hello, Linz!"));
}
