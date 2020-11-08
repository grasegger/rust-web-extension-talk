use wasm_bindgen::prelude::*;
use chrono::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(message: &str);
}

pub struct InstallTimeNotification {
    datetime: DateTime<Utc>
}

impl InstallTimeNotification {
    pub fn create (datetime: DateTime<Utc> ) -> InstallTimeNotification {
      InstallTimeNotification {
        datetime
      }
    }

    pub fn send (&self) {
        unsafe {
            alert(&format!("You installed this extension on {}", self.datetime.format("%Y-%m-%d")));
        }
    }
}