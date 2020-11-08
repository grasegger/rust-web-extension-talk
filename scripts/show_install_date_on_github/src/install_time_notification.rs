use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use shared::install_time::InstallTime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(message: &str);
}

pub struct InstallTimeNotification {
    datetime: DateTime<Utc>
}

impl InstallTimeNotification {
    pub fn create (datetime: InstallTime ) -> InstallTimeNotification {
                
    let timestamp = Utc.timestamp_millis(datetime.install_time);
      InstallTimeNotification {
        datetime: timestamp
      }
    }

    pub fn send (&self) {
        #[allow(unused_unsafe)]
        unsafe {
            alert(&format!("You installed this extension on {}", self.datetime.format("%Y-%m-%d")));
        }
    }
}