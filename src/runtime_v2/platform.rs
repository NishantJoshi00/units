use crate::runtime_v2::integration;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
pub mod users;

use super::types;

pub trait Persistance: DynClone + Send + Sync + users::Users {}
dyn_clone::clone_trait_object!(Persistance);

impl Persistance for () {}

#[derive(Clone)]
pub struct Platform {
    pub storage: Storage,
    // pub persistance: Box<dyn Persistance>
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub redis: Arc<Mutex<redis::Client>>,
    pub kev: Arc<RwLock<HashMap<String, String>>>,
}

impl Platform {
    pub fn init(_config: types::PlatformConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing platform");
        Ok(Self {
            storage: Storage {
                redis: Arc::new(Mutex::new(redis::Client::open("redis://127.0.0.1/")?)),
                kev: Arc::new(RwLock::new(HashMap::new())),
            },
            // persistance: Box::new(()),
        })
    }
}

impl Storage {
    pub fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        match key {
            key if key.starts_with("sol:") => self.get_sol(key),
            key => self.get_redis(key),
        }
    }

    pub fn get_sol(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.kev
            .read()
            .map(|x| x.get(key).cloned())
            .map_err(|e| anyhow::anyhow!("Error reading storage: {:?}", e))
    }

    pub fn get_redis(&self, key: &str) -> anyhow::Result<Option<String>> {
        let client = self
            .redis
            .lock()
            .map_err(|e| anyhow::anyhow!("Error getting redis client: {:?}", e))?;
        let mut con = client.get_connection()?;
        let _ = redis::cmd("GET").arg(key).query::<Option<String>>(&mut con);

        self.kev
            .read()
            .map(|x| x.get(key).cloned())
            .map_err(|e| anyhow::anyhow!("Error reading storage: {:?}", e))
    }

    pub fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        match key {
            key if key.starts_with("sol:") => self.set_sol(key, value),
            key => self.set_redis(key, value),
        }
    }

    pub fn set_redis(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let client = self
            .redis
            .lock()
            .map_err(|e| anyhow::anyhow!("Error getting redis client: {:?}", e))?;
        let mut con = client.get_connection()?;
        redis::cmd("SET").arg(key).arg(value).exec(&mut con)?;

        self.kev
            .write()
            .map(|mut x| {
                x.insert(key.to_string(), value.to_string());
            })
            .map_err(|e| anyhow::anyhow!("Error writing to storage: {:?}", e))
    }

    pub fn set_sol(&self, key: &str, value: &str) -> anyhow::Result<()> {
        tracing::info!(?key, ?value, "submitting proof");
        let output = integration::solana::transfer_token(key.to_string(), value.to_string());
        tracing::info!(signature = ?output, "triggering solana transfer");

        self.kev
            .write()
            .map(|mut x| {
                x.insert(key.to_string(), value.to_string());
            })
            .map_err(|e| anyhow::anyhow!("Error writing to storage: {:?}", e))
    }

    pub fn delete(&self, key: &str) -> anyhow::Result<()> {
        {
            let client = self
                .redis
                .lock()
                .map_err(|e| anyhow::anyhow!("Error getting redis client: {:?}", e))?;
            let mut con = client.get_connection()?;
            redis::cmd("DEL").arg(key).exec(&mut con)?;
        }

        self.kev
            .write()
            .map(|mut x| {
                x.remove(key);
            })
            .map_err(|e| anyhow::anyhow!("Error deleting from storage: {:?}", e))
    }
}
