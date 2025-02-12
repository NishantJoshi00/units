use super::storage::{DriverStorage, Resolver, UserStorage};
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
    pub user: Box<dyn UserStorage>,
}

impl DriverRuntime {
    pub async fn init(_config: types::DriverConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing driver runtime");
        let engine = wasmtime::Engine::new(wasmtime::Config::new().async_support(true))?;
        let resolver = super::storage::sql::SqliteStorage::new("sqlite://units.db").await?;
        Ok(Self {
            engine,
            drivers: Box::new(resolver.clone()),
            resolver: Box::new(resolver.clone()),
            user: Box::new(resolver),
        })
    }

    pub async fn add_driver(
        &self,
        name: String,
        module: wasmtime::component::Component,
        version: String,
    ) -> anyhow::Result<()> {
        let driver_info = DriverInfo { name, version };
        self.drivers.insert(driver_info, module).await?;

        Ok(())
    }

    pub async fn remove_driver(&self, driver_info: DriverInfo) -> anyhow::Result<()> {
        self.drivers.remove(&driver_info).await?;

        Ok(())
    }
}
