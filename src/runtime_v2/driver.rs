
use super::storage::{DriverStorage, Resolver};
use super::types;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
}

#[derive(Clone)]
pub struct DriverRuntime {
    pub engine: wasmtime::Engine,
    pub drivers: Box<dyn DriverStorage>,
    pub resolver: Box<dyn Resolver>,
}

impl DriverRuntime {
    pub fn init(_config: types::DriverConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing driver runtime");
        let engine = wasmtime::Engine::default();
        let resolver = super::storage::PersistentStorage::new();
        Ok(Self {
            engine,
            drivers: Box::new(resolver.clone()),
            resolver: Box::new(resolver),
        })
    }

    pub fn add_driver(
        &self,
        name: String,
        module: wasmtime::component::Component,
        version: String,
    ) -> anyhow::Result<()> {
        let driver_info = DriverInfo { name, version };
        self.drivers.insert(driver_info, module)?;

        Ok(())
    }

    pub fn remove_driver(&self, driver_info: DriverInfo) -> anyhow::Result<()> {
        self.drivers.remove(&driver_info)?;

        Ok(())
    }
}
