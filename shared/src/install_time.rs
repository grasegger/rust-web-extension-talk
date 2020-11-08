use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Date"])]
    fn now() -> JsValue;

    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"])]
    async fn get() -> JsValue;
}
#[derive(Serialize, Deserialize)]
pub struct InstallTime {
    pub install_time: i64,
}

impl InstallTime {
    pub fn get_new_install_date() -> InstallTime {
        #[allow(unused_unsafe)]
        unsafe {
            let install_time = now().as_f64().unwrap_or(0.0) as i64;
            InstallTime { install_time }
        }
    }

    pub async fn get_current () -> InstallTime {
        unsafe {
            let install_time_raw = get().await;
            let install_time  = install_time_raw.into_serde::<InstallTime>();

            match install_time {
                Ok(value) => value,
                Err(_) => InstallTime::get_new_install_date()
            }
        }
    }
}