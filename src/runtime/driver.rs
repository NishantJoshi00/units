//!
//! # Driver
//!
//! The driver are dynamically loaded libraries that provides an abstraction over the
//! underlying platform. To bring into existance, the concept of assets that are represented on the
//! platform, and manipulated in the process layer.
//!

use super::{resolver, types};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct DriverRuntime {
    pub engine: wasmtime::Engine,
    pub drivers: Arc<RwLock<HashMap<String, wasmtime::Module>>>,
    pub resolver: resolver::Resolver,
    pub config: super::types::DriverConfig,
}

impl DriverRuntime {
    pub fn init(config: types::DriverConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing driver runtime");
        let engine = wasmtime::Engine::default();
        let drivers = Arc::new(RwLock::new(HashMap::new()));
        let resolver = resolver::Resolver::init();
        Ok(Self {
            engine,
            drivers,
            resolver,
            config,
        })
    }
}
