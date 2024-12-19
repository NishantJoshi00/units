//!
//! # Platform
//!
//! This module contains the implementation of the platform layer. This layer implements the
//! functions that are mentioned in the PAL. The platform contains implementations for accessing
//! storage, networks and other runtimes providing abstraction like inter-ledger communication,
//! immutable proof stores, etc.
//!
//!

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::types;

#[derive(Clone)]
pub struct Platform {
    pub storage: Storage,
    pub config: types::PlatformConfig,
}

#[derive(Clone)]
pub struct Storage {
    pub kev: Arc<RwLock<HashMap<String, String>>>,
}

impl Platform {
    pub fn init(config: types::PlatformConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing platform");
        Ok(Self {
            storage: Storage {
                kev: Arc::new(RwLock::new(HashMap::new())),
            },
            config,
        })
    }
}

impl Storage {
    pub fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.kev
            .read()
            .map(|x| x.get(key).cloned())
            .map_err(|e| anyhow::anyhow!("Error reading storage: {:?}", e))
    }

    pub fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        self.kev
            .write()
            .map(|mut x| {
                x.insert(key.to_string(), value.to_string());
            })
            .map_err(|e| anyhow::anyhow!("Error writing to storage: {:?}", e))
    }

    pub fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.kev
            .write()
            .map(|mut x| {
                x.remove(key);
            })
            .map_err(|e| anyhow::anyhow!("Error deleting from storage: {:?}", e))
    }
}
