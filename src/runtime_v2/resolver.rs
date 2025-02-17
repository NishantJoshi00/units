use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathInfo {
    pub driver_name: String,
    pub driver_version: String,
    pub account_info: String,
}
