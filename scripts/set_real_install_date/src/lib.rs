use wasm_bindgen::prelude::*;
use shared::install_time::InstallTime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"], js_name = "set")]
    async fn saveSetting(value: JsValue);
}

#[wasm_bindgen(start)]
pub async fn main () {
    unsafe {
        let install_time = InstallTime::get_current().await;
        let setting = JsValue::from_serde(&install_time).unwrap();
        saveSetting(setting).await;    
    }
}