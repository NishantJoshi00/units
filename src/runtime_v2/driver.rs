use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{resolver, types};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
}

#[derive(Clone)]
pub struct DriverRuntime {
    pub engine: wasmtime::Engine,
    pub drivers: Arc<RwLock<HashMap<DriverInfo, wasmtime::component::Component>>>,
    pub resolver: resolver::Resolver,
}

impl DriverRuntime {
    pub fn init(_config: types::DriverConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing driver runtime");
        let engine = wasmtime::Engine::default();
        let drivers = Arc::new(RwLock::new(HashMap::new()));
        let resolver = resolver::Resolver::init();
        Ok(Self {
            engine,
            drivers,
            resolver,
        })
    }

    pub fn add_driver(
        &self,
        name: String,
        module: wasmtime::component::Component,
        version: String,
    ) -> anyhow::Result<()> {
        let driver_info = DriverInfo { name, version };
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(driver_info, module);

        Ok(())
    }

    pub fn remove_driver(&self, driver_info: DriverInfo) -> anyhow::Result<()> {
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .remove(&driver_info);

        Ok(())
    }
}
