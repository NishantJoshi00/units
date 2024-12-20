//!
//! # Runtime
//! This module provides the complete runtime for interacting with finternet.
//! The runtime exposes 2 APIs:
//! 1. privilaged API: This is used to load drivers that can be used by the process layer to perform operations on the assets.
//! 2. DSL runtime: This allows users to access/execute programss that they have submitted or that
//!    are available in the system. These programs interface with the VAL to perform operations on
//!    the assets.
//!

use crate::types::WasmString;

use self::binding::Binding;

pub mod binding;
pub mod driver;
pub mod platform;
pub mod process;
pub mod resolver;
mod service;
pub mod types;

#[derive(Clone)]
pub struct Runtime {
    pub process_layer: process::ProcessRuntime,
    pub driver_layer: driver::DriverRuntime, // the resolver is part of the driver layer
    pub platform_layer: platform::Platform,
    pub config: types::RuntimeConfig,
}

impl Runtime {
    pub fn init(config: types::Config) -> anyhow::Result<Self> {
        tracing::debug!("Initializing runtime");
        Ok(Self {
            process_layer: process::ProcessRuntime::init(config.process)?,
            driver_layer: driver::DriverRuntime::init(config.driver)?,
            platform_layer: platform::Platform::init(config.platform)?,
            config: config.runtime,
        })
    }

    pub fn exec(self, module: wasmtime::Module, input: String) -> anyhow::Result<String> {
        let mut store = wasmtime::Store::new(
            &self.process_layer.engine,
            binding::State::new(self.driver_layer.resolver.clone()),
        );
        let mut linker = wasmtime::Linker::new(&self.process_layer.engine);
        (self.driver_layer, self.platform_layer).bind(&mut linker)?;

        wasi_common::sync::add_to_linker(&mut linker, |state| &mut state.wasi)?;

        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow::anyhow!("No memory"))?;

        let input = WasmString::new(&input);
        let loaded_str = input.allocate_on_wasm(&memory, &mut store)?;

        let main = instance.get_typed_func::<(i32, i32), (i32, i32)>(&mut store, "main")?;
        let result = main.call(&mut store, loaded_str)?;

        let output = WasmString::from_memory(&memory, &store, result)?;

        Ok(output.into_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_init() {
        let config = types::Config {
            server: types::ServerConfig {
                host: "localhost".to_string(),
                port: 50051,
            },
            runtime: types::RuntimeConfig {
                name: "finternet".to_string(),
                version: "0.1.0".to_string(),
            },
            driver: types::DriverConfig {
                driver_limit: 10,
                driver_timeout: 100,
            },
            process: types::ProcessConfig {},
            platform: types::PlatformConfig {},
        };

        let runtime = Runtime::init(config).unwrap();
        assert_eq!(runtime.config.name, "finternet");
        assert_eq!(runtime.config.version, "0.1.0");
        assert_eq!(runtime.driver_layer.config.driver_limit, 10);
        assert_eq!(runtime.driver_layer.config.driver_timeout, 100);
    }
}
