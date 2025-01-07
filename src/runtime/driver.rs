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

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub version: String,
    pub module:wasmtime::Module,
  
}

#[derive(Clone)]
pub struct DriverRuntime {
    pub engine: wasmtime::Engine,
    pub drivers: Arc<RwLock<HashMap<String, DriverInfo>>>,
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

    pub fn add_driver(&self, name: String, module: wasmtime::Module,version:String) -> anyhow::Result<()> {
          let driver_info=DriverInfo{
            version,
            module,
          };
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(name, driver_info);

        Ok(())
    }

    pub fn remove_driver(&self, name: String) -> anyhow::Result<()> {
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .remove(&name);

        Ok(())
    }
}
