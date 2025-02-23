use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

use anyhow::ensure;
use smol::process::driver;
use crate::runtime_v2::types::{CairoInput, ProgramComponent};

use self::types::ServerConfig;

pub mod driver;
pub mod glue;
pub mod integration;
pub mod platform;
pub mod process;
pub mod provable;
pub mod resolver;
pub mod service;
pub mod storage;
pub mod types;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConfig {
    pub process: types::ProcessConfig,
    pub driver: types::DriverConfig,
    pub platform: types::PlatformConfig,
    pub server: ServerConfig,
    pub provable: types::ProvableConfig,
}

#[derive(Clone)]
pub struct Runtime {
    pub process_layer: process::ProcessRuntime,
    pub driver_layer: driver::DriverRuntime,
    pub platform_layer: platform::Platform,
    pub event_sender: Arc<mpsc::Sender<types::Event>>,
    pub provable_layer: provable::ProvableRuntime,
}

impl Runtime {
    pub async fn init(config: RuntimeConfig) -> anyhow::Result<Self> {
        tracing::debug!("Initializing runtime");

        let (tx, _rx) = mpsc::channel();

        Ok(Self {
            process_layer: process::ProcessRuntime::init(config.process).await?,
            driver_layer: driver::DriverRuntime::init(config.driver).await?,
            platform_layer: platform::Platform::init(config.platform)?,
            event_sender: Arc::new(tx),
            provable_layer: provable::ProvableRuntime::new(config.provable),
        })
    }

    pub async fn exec(
        self,
        ctx: types::UserCtx,
        module: ProgramComponent,
        input: String,
    ) -> anyhow::Result<String> {
        let output = match module { 
            ProgramComponent::WASM(program) => {
                let mut state = wasmtime::Store::new(
                    &self.process_layer.engine,
                    types::ProcessState::new(
                        ctx,
                        self.driver_layer,
                        self.platform_layer,
                        self.event_sender,
                        self.provable_layer
                    ),
                );
                let mut linker = wasmtime::component::Linker::new(&self.process_layer.engine);

                types::component::module::ModuleWorld::add_to_linker(
                    &mut linker,
                    |state: &mut types::ProcessState| state,
                )?;

                wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
                    types::component::module::component::units::driver::DriverError::SystemError(
                        err.to_string(),
                    )
                })?;
                let instance =
                    types::component::module::ModuleWorld::instantiate_async(&mut state, &program.module, &linker)
                        .await?;

                tracing::info!(runtime = "process", input = %input, "executing module");

                let result = instance.call_main(state, &input).await?;
                match result {
                    Ok(output) => output,
                    Err(e) => {
                        tracing::error!(?e, "Error while executing module");
                        anyhow::bail!("Error while executing module")
                    }
                }
            }
            ProgramComponent::Cairo(program) => {
                let cairo_input: CairoInput = serde_json::from_str(input.as_str()).unwrap();

                let program_input = cairo_input.program_input;
                let program_input_w_driver_details = format!(
                    "0x1,{},{}",
                    program.program_address,
                    program_input
                );

                self.provable_layer
                    .execute_program(
                        ctx.user_id,
                        program_input_w_driver_details,
                        cairo_input.user_signature,
                    )
                    .await?
            }
        };
        
        Ok(output)
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
