use serde::{Deserialize, Serialize};
use crate::runtime_v2::types::ProgramComponent;
use super::{storage::ProgramStorage, types};

#[derive(Clone)]
pub struct ProcessRuntime {
    pub engine: wasmtime::Engine,
    pub config: types::ProcessConfig,
    pub programs: Box<dyn ProgramStorage>,
}

#[derive(Clone)]
pub struct Program {
    pub component: ProgramComponent,
    pub name: String,
    pub version: String,
}

impl ProcessRuntime {
    pub async fn init(config: types::ProcessConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing process runtime");
        let engine = wasmtime::Engine::new(wasmtime::Config::new().async_support(true))?;
        Ok(Self {
            engine,
            config,
            programs: Box::new(
                super::storage::sql::SqliteStorage::new("sqlite://units_zk.db").await?,
            ),
        })
    }

    pub async fn store_program(
        &self,
        name: String,
        version: String,
        component: ProgramComponent,
    ) -> anyhow::Result<String> {
        let program = Program {
            name,
            version,
            component,
        };
        let id = crate::utils::id::new();
        self.programs.insert(&id, program).await?;

        Ok(id)
    }

    pub async fn find_program(
        &self,
        id: &str,
        engine: wasmtime::Engine,
    ) -> anyhow::Result<Option<Program>> {
        self.programs.get(id, engine).await
    }
}
