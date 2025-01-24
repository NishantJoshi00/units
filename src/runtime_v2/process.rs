use super::types;

#[derive(Clone)]
pub struct ProcessRuntime {
    pub engine: wasmtime::Engine,
    pub config: types::ProcessConfig,
}

impl ProcessRuntime {
    pub fn init(config: types::ProcessConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing process runtime");
        let engine = wasmtime::Engine::default();
        Ok(Self { engine, config })
    }
}
