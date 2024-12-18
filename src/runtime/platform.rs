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
