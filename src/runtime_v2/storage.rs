use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tonic::async_trait;

use super::{driver::DriverInfo, platform::users::User, process::Program, resolver::PathInfo};

#[derive(Eq, Hash, PartialEq)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Default)]
pub struct PersistentStorage {
    mount_points: Arc<RwLock<HashMap<String, PathInfo>>>,
    pub programs: Arc<RwLock<HashMap<String, Program>>>,
    pub drivers: Arc<RwLock<HashMap<DriverInfo, wasmtime::component::Component>>>,
    pub user: Arc<RwLock<HashMap<UserInfo, String>>>,
}

mod private {
    pub trait Safety: Send + Sync + 'static {}
    impl<T: Send + Sync + 'static> Safety for T {}
}

#[async_trait]
pub trait Resolver: dyn_clone::DynClone + private::Safety {
    async fn remove(&self, path: &str) -> Option<PathInfo>;
    async fn list(&self) -> Vec<(String, PathInfo)>;
    async fn get(&self, path: &str) -> Option<PathInfo>;
    async fn insert(&self, path: String, path_info: PathInfo) -> Option<()>;
}

#[async_trait]
pub trait ProgramStorage: dyn_clone::DynClone + private::Safety {
    async fn insert(&self, id: &str, program: super::process::Program) -> anyhow::Result<()>;
    async fn get(
        &self,
        id: &str,
        engine: wasmtime::Engine,
    ) -> Result<Option<Program>, anyhow::Error>;
    async fn list(&self, engine: wasmtime::Engine)
        -> Result<Vec<(String, Program)>, anyhow::Error>;
}

#[async_trait]
pub trait DriverStorage: dyn_clone::DynClone + private::Safety {
    async fn insert(
        &self,
        driver_info: DriverInfo,
        module: wasmtime::component::Component,
    ) -> anyhow::Result<()>;
    async fn get(
        &self,
        driver_info: &DriverInfo,
        engine: wasmtime::Engine,
    ) -> Result<Option<wasmtime::component::Component>, anyhow::Error>;
    async fn list(
        &self,
        engine: wasmtime::Engine,
    ) -> Result<Vec<(DriverInfo, wasmtime::component::Component)>, anyhow::Error>;
    async fn remove(&self, driver_info: &DriverInfo) -> anyhow::Result<()>;
}

#[async_trait]
pub trait UserStorage: dyn_clone::DynClone + private::Safety {
    async fn insert(&self, username: &str, password: &str) -> anyhow::Result<String>;
    async fn get(&self, username: &str, password: &str) -> anyhow::Result<Option<String>>;
}

#[async_trait]
impl Resolver for PersistentStorage {
    async fn remove(&self, path: &str) -> Option<PathInfo> {
        self.mount_points.write().ok()?.remove(path)
    }

    async fn list(&self) -> Vec<(String, PathInfo)> {
        self.mount_points
            .read()
            .unwrap()
            .iter()
            .map(|(path, path_info)| (path.clone(), path_info.clone()))
            .collect()
    }

    async fn get(&self, path: &str) -> Option<PathInfo> {
        self.mount_points.read().ok()?.get(path).cloned()
    }

    async fn insert(&self, path: String, path_info: PathInfo) -> Option<()> {
        self.mount_points.write().ok()?.insert(path, path_info);
        Some(())
    }
}

#[async_trait]
impl ProgramStorage for PersistentStorage {
    async fn insert(&self, id: &str, program: super::process::Program) -> anyhow::Result<()> {
        self.programs
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(id.to_string(), program);
        Ok(())
    }
    async fn get(
        &self,
        id: &str,
        _engine: wasmtime::Engine,
    ) -> Result<Option<Program>, anyhow::Error> {
        Ok(self
            .programs
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .get(id)
            .cloned())
    }
    async fn list(
        &self,
        _engine: wasmtime::Engine,
    ) -> Result<Vec<(String, Program)>, anyhow::Error> {
        Ok(self
            .programs
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .iter()
            .map(|(id, program)| (id.clone(), program.clone()))
            .collect())
    }
}

#[async_trait]
impl DriverStorage for PersistentStorage {
    async fn insert(
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
    async fn get(
        &self,
        driver_info: &DriverInfo,
        _engine: wasmtime::Engine,
    ) -> Result<Option<wasmtime::component::Component>, anyhow::Error> {
        Ok(self
            .drivers
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .get(driver_info)
            .cloned())
    }

    async fn list(
        &self,
        _engine: wasmtime::Engine,
    ) -> Result<Vec<(DriverInfo, wasmtime::component::Component)>, anyhow::Error> {
        Ok(self
            .drivers
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .iter()
            .map(|(driver_info, component)| (driver_info.clone(), component.clone()))
            .collect())
    }

    async fn remove(&self, driver_info: &DriverInfo) -> anyhow::Result<()> {
        self.drivers
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .remove(driver_info);
        Ok(())
    }
}

#[async_trait]
impl UserStorage for PersistentStorage {
    async fn insert(&self, username: &str, password: &str) -> anyhow::Result<String> {
        let user_info = UserInfo {
            username: username.to_string(),
            password: password.to_string(),
        };
        let user_id = username.to_string();
        self.user
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(user_info, user_id.clone());
        Ok(user_id)
    }
    async fn get(&self, username: &str, password: &str) -> anyhow::Result<Option<String>> {
        let user_info = UserInfo {
            username: username.to_string(),
            password: password.to_string(),
        };
        let user_map = self
            .user
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?;

        Ok(user_map.get(&user_info).cloned())
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
dyn_clone::clone_trait_object!(UserStorage);

pub mod sql;
