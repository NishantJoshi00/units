use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::{driver::DriverInfo, process::Program, resolver::PathInfo};

#[derive(Clone, Default)]
pub struct PersistentStorage {
    mount_points: Arc<RwLock<HashMap<String, PathInfo>>>,
    pub programs: Arc<RwLock<HashMap<String, Program>>>,
    pub drivers: Arc<RwLock<HashMap<DriverInfo, wasmtime::component::Component>>>,
}

mod private {
    pub trait Safety: Send + Sync + 'static {}
    impl<T: Send + Sync + 'static> Safety for T {}
}

pub trait Resolver: dyn_clone::DynClone + private::Safety {
    fn remove(&self, path: &str) -> Option<PathInfo>;
    fn list(&self) -> Vec<(String, PathInfo)>;
    fn get(&self, path: &str) -> Option<PathInfo>;
    fn insert(&self, path: String, path_info: PathInfo) -> Option<()>;
}

pub trait ProgramStorage: dyn_clone::DynClone + private::Safety {
    fn insert(&self, id: &str, program: super::process::Program) -> anyhow::Result<()>;
    fn get(&self, id: &str) -> Result<Option<Program>, anyhow::Error>;
    fn list(&self) -> Result<Vec<(String, Program)>, anyhow::Error>;
}

pub trait DriverStorage: dyn_clone::DynClone + private::Safety {
    fn insert(
        &self,
        driver_info: DriverInfo,
        module: wasmtime::component::Component,
    ) -> anyhow::Result<()>;
    fn get(
        &self,
        driver_info: &DriverInfo,
    ) -> Result<Option<wasmtime::component::Component>, anyhow::Error>;
    fn list(&self) -> Result<Vec<(DriverInfo, wasmtime::component::Component)>, anyhow::Error>;
    fn remove(&self, driver_info: &DriverInfo) -> anyhow::Result<()>;
}

impl Resolver for PersistentStorage {
    fn remove(&self, path: &str) -> Option<PathInfo> {
        self.mount_points.write().ok()?.remove(path)
    }

    fn list(&self) -> Vec<(String, PathInfo)> {
        self.mount_points
            .read()
            .unwrap()
            .iter()
            .map(|(path, path_info)| (path.clone(), path_info.clone()))
            .collect()
    }

    fn get(&self, path: &str) -> Option<PathInfo> {
        self.mount_points.read().ok()?.get(path).cloned()
    }

    fn insert(&self, path: String, path_info: PathInfo) -> Option<()> {
        self.mount_points.write().ok()?.insert(path, path_info);
        Some(())
    }
}

impl ProgramStorage for PersistentStorage {
    fn insert(&self, id: &str, program: super::process::Program) -> anyhow::Result<()> {
        self.programs
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(id.to_string(), program);
        Ok(())
    }
    fn get(&self, id: &str) -> Result<Option<Program>, anyhow::Error> {
        Ok(self
            .programs
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .get(id)
            .cloned())
    }
    fn list(&self) -> Result<Vec<(String, Program)>, anyhow::Error> {
        Ok(self
            .programs
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .iter()
            .map(|(id, program)| (id.clone(), program.clone()))
            .collect())
    }
}

impl DriverStorage for PersistentStorage {
    fn insert(
        &self,
        driver_info: DriverInfo,
        module: wasmtime::component::Component,
    ) -> anyhow::Result<()> {
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(driver_info, module);
        Ok(())
    }
    fn get(
        &self,
        driver_info: &DriverInfo,
    ) -> Result<Option<wasmtime::component::Component>, anyhow::Error> {
        Ok(self
            .drivers
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .get(driver_info)
            .cloned())
    }

    fn list(&self) -> Result<Vec<(DriverInfo, wasmtime::component::Component)>, anyhow::Error> {
        Ok(self
            .drivers
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .iter()
            .map(|(driver_info, component)| (driver_info.clone(), component.clone()))
            .collect())
    }

    fn remove(&self, driver_info: &DriverInfo) -> anyhow::Result<()> {
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .remove(driver_info);
        Ok(())
    }
}

impl PersistentStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

dyn_clone::clone_trait_object!(Resolver);
dyn_clone::clone_trait_object!(ProgramStorage);
dyn_clone::clone_trait_object!(DriverStorage);

pub mod sql;
