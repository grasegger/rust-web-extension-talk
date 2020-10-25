use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
pub fn main () {
    log_1(&JsValue::from("Hello, Linz!"));    
}

