use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::types;

#[derive(Clone)]
pub struct ProcessRuntime {
    pub engine: wasmtime::Engine,
    pub config: types::ProcessConfig,
    pub programs: Arc<RwLock<HashMap<String, Program>>>,
}

#[derive(Clone)]
pub struct Program {
    pub component: wasmtime::component::Component,
    pub name: String,
    pub version: String,
}

impl ProcessRuntime {
    pub fn init(config: types::ProcessConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing process runtime");
        let engine = wasmtime::Engine::default();
        Ok(Self {
            engine,
            config,
            programs: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn store_program(
        &self,
        name: String,
        version: String,
        component: wasmtime::component::Component,
    ) -> anyhow::Result<String> {
        let program = Program {
            name,
            version,
            component,
        };
        let id = crate::utils::id::new();
        self.programs
            .write()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .insert(id.clone(), program);

        Ok(id)
    }

    pub fn find_program(&self, id: &str) -> anyhow::Result<Option<Program>> {
        Ok(self
            .programs
            .read()
            .map_err(|e| anyhow::anyhow!("Poisoned Lock {:?}", e))?
            .get(id)
            .cloned())
    }
}
