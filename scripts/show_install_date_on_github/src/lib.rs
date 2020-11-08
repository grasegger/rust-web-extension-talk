mod install_time_notification;

use wasm_bindgen::prelude::*;
use std::panic;
use shared::install_time::InstallTime;
use install_time_notification::InstallTimeNotification;

#[wasm_bindgen(start)]
pub async fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    unsafe {
        let install_time = InstallTime::get_current().await;
        let notification = InstallTimeNotification::create(install_time);
        notification.send();
    }
}
