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
use tonic::{Status, Response};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UserInfo{
    pub name: String,
    pub email: String,
    pub password: blake3::Hash,
}

#[derive(Clone)]
pub struct DriverRuntime {
    pub engine: wasmtime::Engine,
    pub drivers: Arc<RwLock<HashMap<DriverInfo, wasmtime::Module>>>,
    pub user_store:Arc<RwLock<HashMap<String,UserInfo>>>,
    pub resolver: resolver::Resolver,
    pub config: super::types::DriverConfig,
}

impl DriverRuntime {
    pub fn init(config: types::DriverConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing driver runtime");
        let engine = wasmtime::Engine::default();
        let drivers = Arc::new(RwLock::new(HashMap::new()));
        let user_store= Arc::new(RwLock::new(HashMap::new()));
        let resolver = resolver::Resolver::init();
        Ok(Self {
            engine,
            drivers,
            user_store,
            resolver,
            config,
        })
    }

    pub fn add_driver(
        &self,
        name: String,
        module: wasmtime::Module,
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

    pub fn add_user(
        &self,
        username: String,
        name: String,
        password: String,
        email: String,
    ) -> anyhow::Result<()> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(password.as_bytes()); 
        let hash_pass = hasher.finalize(); 
        let user_info = UserInfo { name,email,password: hash_pass };
        self.user_store
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(username, user_info);
        Ok(())
    }

    pub fn verify_user(
        &self,
        username: String,
        password: String,
    ) -> Result<Response<String>, Status> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(password.as_bytes());
        let hash_pass = hasher.finalize();

        let read_guard = self.user_store.read().map_err(|_| Status::internal("Failed to lock map"))?;
        let user_info = read_guard.get(&username);

        match user_info {
            Some(user) => {
                if hash_pass == user.password{
                    Ok(Response::new(String::from("user verified successfully")))
                } else {
                    Err(Status::invalid_argument("Password is not correct"))
                }
            }
            None => Err(Status::not_found("User not found!!!")),
        }
    }
}
