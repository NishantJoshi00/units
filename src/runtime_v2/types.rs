use std::collections::HashMap;
use std::sync::{mpsc, Arc};

use super::driver::{self, DriverInfo};
use super::platform::Platform;

pub mod component {
    pub mod driver {
        wasmtime::component::bindgen!({
            world: "driver-world",
            path: "wit"
        });
    }

    pub mod module {
        wasmtime::component::bindgen!({
            world: "module-world",
            path: "wit"
        });
    }
}

#[derive(Clone)]
pub struct UserCtx {
    pub user_id: String,
}

#[derive(Clone)]
pub struct DriverCtx {
    pub driver_info: DriverInfo,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DriverConfig {
    pub driver_limit: u32,
    pub driver_timeout: u32, // in seconds
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProcessConfig {}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlatformConfig {}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Event {
    pub loc: Loc,
    pub event_type: EventType,
    pub level: Level,
    pub call_type: CallType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Level {
    Platform,
    Driver,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum CallType {
    Intend,
    Done,
    Transfer,
    View,
    Set,
    Get,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub enum EventType {
    #[default]
    Info,
    Error,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub enum Loc {
    Start,
    #[default]
    Event,
    End,
}

#[derive(Clone)]
pub struct ProcessState {
    pub ctx: UserCtx,
    pub driver_runtime: driver::DriverRuntime,
    pub platform: Platform,
    pub event_sender: Arc<mpsc::Sender<Event>>,
    pub descriptors: HashMap<String, Descriptor>,
}

#[derive(Clone)]
pub struct Descriptor {
    pub driver_name: String,
    pub driver_version: String,
    pub account_info: serde_json::Value,
}

#[derive(Clone)]
pub struct DriverState {
    pub ctx: UserCtx,
    pub driver_ctx: DriverCtx,
    pub platform: Platform,
    pub event_sender: Arc<mpsc::Sender<Event>>,
}

impl ProcessState {
    pub fn new(
        ctx: UserCtx,
        driver_runtime: driver::DriverRuntime,
        platform: Platform,
        event_sender: Arc<mpsc::Sender<Event>>,
    ) -> Self {
        Self {
            ctx,
            driver_runtime,
            platform,
            event_sender,
            descriptors: HashMap::new(),
        }
    }

    pub fn get_path_info(
        &self,
        input: String,
    ) -> Result<super::resolver::PathInfo, component::module::components::units::driver::DriverError>
    {
        let path_info = self
            .driver_runtime
            .resolver
            .mount_points
            .read()
            .map_err(|_| {
                component::module::components::units::driver::DriverError::SystemError(
                    "Failed while finding path".to_string(),
                )
            })?
            .get(input.as_str())
            .cloned()
            .ok_or(
                component::module::components::units::driver::DriverError::InvalidInput(
                    "Failed while resolving path".to_string(),
                ),
            )?;
        Ok(path_info)
    }

    pub fn get_driver(
        &self,
        input: &DriverInfo,
    ) -> Result<
        wasmtime::component::Component,
        component::module::components::units::driver::DriverError,
    > {
        let driver_list = self.driver_runtime.drivers.read().map_err(|_| {
            component::module::components::units::driver::DriverError::SystemError(
                "Failed while finding driver".to_string(),
            )
        })?;

        let driver = driver_list.get(input).ok_or(
            component::module::components::units::driver::DriverError::InvalidInput(
                "Failed while finding driver".to_string(),
            ),
        )?;

        Ok(driver.clone())
    }

    pub fn get_lower_runtime(
        &self,
        driver_info: DriverInfo,
    ) -> Result<
        (
            wasmtime::component::Linker<DriverState>,
            wasmtime::Store<DriverState>,
        ),
        component::module::components::units::driver::DriverError,
    > {
        let state = wasmtime::Store::new(
            &self.driver_runtime.engine,
            DriverState::new(
                self.ctx.clone(),
                DriverCtx { driver_info },
                self.platform.clone(),
                self.event_sender.clone(),
            ),
        );

        let mut linker = wasmtime::component::Linker::new(&self.driver_runtime.engine);

        component::driver::DriverWorld::add_to_linker(&mut linker, |state: &mut DriverState| state)
            .map_err(|_| {
                component::module::components::units::driver::DriverError::SystemError(
                    "Failed while adding driver to linker".to_string(),
                )
            })?;

        Ok((linker, state))
    }

    pub fn get_descriptor(
        &self,
        key: String,
    ) -> Result<&Descriptor, component::module::components::units::driver::DriverError> {
        self.descriptors.get(&key).ok_or(
            component::module::components::units::driver::DriverError::InvalidInput(
                "Failed while finding descriptor".to_string(),
            ),
        )
    }

    pub fn delete_descriptor(
        &mut self,
        key: String,
    ) -> Result<(), component::module::components::units::driver::DriverError> {
        self.descriptors.remove(&key).ok_or(
            component::module::components::units::driver::DriverError::InvalidInput(
                "Failed while deleting descriptor".to_string(),
            ),
        )?;
        Ok(())
    }
}

impl DriverState {
    pub fn new(
        ctx: UserCtx,
        driver_ctx: DriverCtx,
        platform: Platform,
        event_sender: Arc<mpsc::Sender<Event>>,
    ) -> Self {
        Self {
            ctx,
            driver_ctx,
            platform,
            event_sender,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
