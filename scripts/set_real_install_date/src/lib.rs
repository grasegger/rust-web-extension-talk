mod install_time;

use wasm_bindgen::prelude::*;
use install_time::InstallTime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"])]
    async fn get() -> JsValue;

    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"], js_name = "set")]
    async fn saveSetting(value: JsValue);

    #[wasm_bindgen(js_namespace = ["Date"])]
    fn now() -> JsValue;
}

#[wasm_bindgen(start)]
pub async fn main () {
    unsafe {
        let install_time_raw = get().await;
        let install_time  = install_time_raw.into_serde::<InstallTime>();

        let install_time = match install_time {
            Ok(value) => value,
            Err(_) => get_new_install_date()
        };

        let setting = JsValue::from_serde(&install_time).unwrap();
        saveSetting(setting).await;    
    }
}

fn get_new_install_date () -> InstallTime {
    unsafe {
        let install_time = now().as_f64().unwrap_or(0.0) as i64;
        InstallTime{install_time}
    }
}
