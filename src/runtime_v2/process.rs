
use super::{storage::ProgramStorage, types};

#[derive(Clone)]
pub struct ProcessRuntime {
    pub engine: wasmtime::Engine,
    pub config: types::ProcessConfig,
    pub programs: Box<dyn ProgramStorage>,
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
            programs: Box::new(super::storage::PersistentStorage::new()),
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
        self.programs.insert(&id, program)?;

        Ok(id)
    }

    pub fn find_program(&self, id: &str) -> anyhow::Result<Option<Program>> {
        self.programs.get(id)
    }
}
