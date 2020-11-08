use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct InstallTime {
    pub install_time: i64
}
