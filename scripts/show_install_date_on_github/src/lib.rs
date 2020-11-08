mod install_time;
mod install_time_notification;

use wasm_bindgen::prelude::*;
use std::panic;
use chrono::prelude::*;
use install_time::InstallTime;
use install_time_notification::InstallTimeNotification;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["console"])]
    fn log(message: &str);

    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"])]
    async fn get() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["browser", "storage", "local"], js_name = "set")]
    async fn saveSetting(value: JsValue);

    #[wasm_bindgen(js_namespace = ["Date"])]
    fn now() -> JsValue;
}

#[wasm_bindgen(start)]
pub async fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window = web_sys::window().expect("no global `window` exists");
    let _document = window.document().expect("should have a document on window");

    unsafe {
        let install_time_raw = get().await;
        let install_time  = install_time_raw.into_serde::<InstallTime>();

        let install_time = match install_time {
            Ok(value) => value,
            Err(_) => get_new_install_date()
        };

        let setting = JsValue::from_serde(&install_time).unwrap();
        saveSetting(setting).await;
        
        let timestamp = Utc.timestamp_millis(install_time.install_time);
        let notification = InstallTimeNotification::create(timestamp);
        notification.send();
        
    }
}

 fn get_new_install_date () -> InstallTime {
    unsafe {
        let install_time = now().as_f64().unwrap_or(0.0) as i64;
        InstallTime{install_time}
    }
}
