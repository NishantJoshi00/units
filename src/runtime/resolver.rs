use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Resolver {
    pub mount_points: Arc<RwLock<HashMap<String, String>>>,
}

impl Resolver {
    pub fn init() -> Self {
        Self {
            mount_points: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
