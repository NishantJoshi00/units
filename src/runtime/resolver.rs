use std::collections::HashMap;
use std::sync::{Arc, RwLock};
#[derive(Clone)]
pub struct PathInfo{
    pub driver_name: String,
    pub driver_version: String,
    pub account_info: String
}

#[derive(Clone)]
pub struct Resolver {
    pub mount_points: Arc<RwLock<HashMap<String, PathInfo>>>, // path -> (driver_name,driver_version
                                                                              // account_info)
}

impl Resolver {
    pub fn init() -> Self {
        Self {
            mount_points: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
