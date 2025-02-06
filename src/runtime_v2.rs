use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

use anyhow::ensure;

use self::types::ServerConfig;

pub mod driver;
pub mod glue;
pub mod platform;
pub mod process;
pub mod resolver;
pub mod service;
pub mod types;
pub mod integration;
pub mod storage;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConfig {
    pub process: types::ProcessConfig,
    pub driver: types::DriverConfig,
    pub platform: types::PlatformConfig,
    pub server: ServerConfig,
}

#[derive(Clone)]
pub struct Runtime {
    pub process_layer: process::ProcessRuntime,
    pub driver_layer: driver::DriverRuntime,
    pub platform_layer: platform::Platform,
    pub event_sender: Arc<mpsc::Sender<types::Event>>,
}

impl Runtime {
    pub fn init(config: RuntimeConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing runtime");

        let (tx, _rx) = mpsc::channel();

        Ok(Self {
            process_layer: process::ProcessRuntime::init(config.process)?,
            driver_layer: driver::DriverRuntime::init(config.driver)?,
            platform_layer: platform::Platform::init(config.platform)?,
            event_sender: Arc::new(tx),
        })
    }

    pub fn exec(
        self,
        ctx: types::UserCtx,
        module: wasmtime::component::Component,
        input: String,
    ) -> anyhow::Result<String> {
        let mut state = wasmtime::Store::new(
            &self.process_layer.engine,
            types::ProcessState::new(
                ctx,
                self.driver_layer,
                self.platform_layer,
                self.event_sender,
            ),
        );

        let mut linker = wasmtime::component::Linker::new(&self.process_layer.engine);

        

        types::component::module::ModuleWorld::add_to_linker(
            &mut linker,
            |state: &mut types::ProcessState| state,
        )?;

        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let instance =
            types::component::module::ModuleWorld::instantiate(&mut state, &module, &linker)?;

        tracing::info!(runtime = "process", input = %input, "executing module");

        let result = instance.call_main(state, &input)?;

        match result {
            Ok(output) => Ok(output),
            Err(e) => {
                tracing::error!(?e, "Error while executing module");
                anyhow::bail!("Error while executing module")
            }
        }
    }
}

impl RuntimeConfig {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        ensure!(path.exists(), "Config file does not exist");
        ensure!(path.is_file(), "Config file is not a file");

        match path.extension() {
            Some(ext) if ext == "toml" => {
                tracing::info!(?path, "Reading TOML config file");
                let content = std::fs::read_to_string(path)?;
                let config: Self = toml::from_str(&content)?;
                Ok(config)
            }
            Some(ext) if ext == "json" => {
                tracing::info!(?path, "Reading JSON config file");
                let content = std::fs::read_to_string(path)?;
                let config: Self = serde_json::from_str(&content)?;
                Ok(config)
            }
            Some(ext) => anyhow::bail!("Invalid extension: {}", ext.to_string_lossy()),
            None => anyhow::bail!("path must be either a `.json` or `.toml` file"),
        }
    }
}
