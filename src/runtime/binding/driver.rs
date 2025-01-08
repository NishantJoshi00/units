use anyhow::ensure;
use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;

use crate::runtime::driver::DriverRuntime;
use crate::runtime::platform::Platform;
use crate::types::WasmString;
use crate::runtime::driver::DriverInfo;

use super::{Binding, Descriptor, State};

impl Binding<State> for (DriverRuntime, Platform) {
    ///
    /// binding driver to linker
    /// This function will link functions that are exposed by the driver to the linker.
    /// These are `syscall` equivalent, presented to the WebAssembly module.
    ///
    /// This functions includes:
    /// - `intend`: Creating an intent, which is a builder pattern over time, to perform a
    /// - `transfer`: This performs transfer between 2 different active accounts.
    ///   transaction.
    /// - `done`: This function is called to finalize the intent and perform the transaction.
    ///
    fn bind(self, linker: &mut wasmtime::Linker<State>) -> anyhow::Result<()> {
        let driver = self.0.clone();
        let platform = self.1.clone();

        // this will create a asset descriptor
        // this is used to store the account information for this transaction
        let intend = move |mut caller: wasmtime::Caller<'_, State>,
                           path_ptr: i32,
                           path_len: i32|
              -> anyhow::Result<(i32, i32)> {
            let driver = driver.clone();
            let platform = platform.clone();

            let path_name = WasmString::from_caller(&mut caller, (path_ptr, path_len))?;
            tracing::info!(system = "driver", func = "intend", "syscall");

            let (driver_name,driver_version ,account_info) = caller
                .data()
                .resolver
                .mount_points
                .read()
                .map_err(|_| anyhow::anyhow!("lock failed"))?
                .get(path_name.into_str())
                .ok_or_else(|| anyhow::anyhow!("Path not found"))?
                .clone();

            let mut lower_store = wasmtime::Store::new(&driver.engine, PlatformState::default());
            let mut lower_linker = wasmtime::Linker::new(&driver.engine);
            platform.bind(&mut lower_linker)?;
        
            let driver_list = driver
                .drivers
                .read()
                .map_err(|_| anyhow::anyhow!("lock failed"))?;

            let driver_info=DriverInfo{
                name: driver_name.clone(),
                version: driver_version.clone(),
            };

            let driver_module = driver_list
                .get(&driver_info)
                .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;
            
            let lower_instance = lower_linker.instantiate(&mut lower_store, driver_module)?;
            let lower_memory = lower_instance
                .get_memory(&mut lower_store, "memory")
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let input = WasmString::new(&account_info);
            let loaded_input = input.allocate_on_wasm(&lower_memory, &mut lower_store)?;
            let intend_caller = lower_instance
                .get_typed_func::<(i32, i32), (i32, i32)>(&mut lower_store, "intend")?;

            let result = intend_caller.call(&mut lower_store, loaded_input)?;
            let output = WasmString::from_memory(&lower_memory, &lower_store, result)?;

            // store output in the descriptor
            let key = crate::utils::id::new();
            caller.data_mut().descriptors.insert(
                key.clone(),
                Descriptor {
                    driver_name: driver_name.clone(),
                    driver_version: driver_version.clone(),
                    account_info: serde_json::from_str(output.into_str())?,
                },
            );

            let memory = caller
                .get_export("memory")
                .and_then(|m| m.into_memory())
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let loaded_str = WasmString::new(&key).allocate_on_caller(&memory, &mut caller)?;

            Ok(loaded_str)
        };

        let driver = self.0.clone();
        let platform = self.1.clone();

        // this will close the asset descriptor
        // this is used to finalize the transaction
        let done = move |mut caller: wasmtime::Caller<'_, State>, key_ptr: i32, key_len: i32| {
            tracing::info!(system = "driver", func = "done", "syscall");
            let driver = driver.clone();
            let platform = platform.clone();
            let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;
            let value = caller.data().descriptors.get(key.clone().into_str());

            match value {
                None => anyhow::bail!("Descriptor not found"),
                Some(value) => {
                    // get the driver, and execute the done function

                    let driver_name = value.driver_name.clone();
                    let driver_version= value.driver_version.clone();
                    let account_info = value.account_info.clone();

                    let mut lower_store =
                        wasmtime::Store::new(&driver.engine, PlatformState::default());
                    let mut lower_linker = wasmtime::Linker::new(&driver.engine);
                    platform.bind(&mut lower_linker)?;

                    let driver_list = driver
                        .drivers
                        .read()
                        .map_err(|_| anyhow::anyhow!("lock failed"))?;

                    let driver_info=DriverInfo{
                        name:driver_name,
                        version:driver_version,
                    };
                    
                    let driver_module = driver_list
                        .get(&driver_info)
                        .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;

                    let lower_instance =
                        lower_linker.instantiate(&mut lower_store, driver_module)?;

                    let lower_memory = lower_instance
                        .get_memory(&mut lower_store, "memory")
                        .ok_or_else(|| anyhow::anyhow!("No memory"))?;

                    let input = serde_json::to_string(&account_info)?;

                    let loaded_input = WasmString::new(&input)
                        .allocate_on_wasm(&lower_memory, &mut lower_store)?;

                    let done_caller = lower_instance
                        .get_typed_func::<(i32, i32), ()>(&mut lower_store, "done")?;

                    done_caller.call(&mut lower_store, loaded_input)?;

                    caller.data_mut().descriptors.remove(key.into_str());
                }
            }

            Ok(())
        };

        let driver = self.0.clone();
        let platform = self.1.clone();

        // this will transfer the asset
        // this will transfer the asset from one descriptor to another
        // the low level driver will call the transfer function of the driver with the account info
        let transfer = move |mut caller: wasmtime::Caller<'_, State>,
                             ad1_ptr: i32,
                             ad1_len: i32,
                             ad2_ptr: i32,
                             ad2_len: i32,
                             data_ptr: i32,
                             data_len: i32| {
            tracing::info!(system = "driver", func = "transfer", "syscall");
            let driver = driver.clone();
            let platform = platform.clone();

            let ad1 = WasmString::from_caller(&mut caller, (ad1_ptr, ad1_len))?;
            let ad2 = WasmString::from_caller(&mut caller, (ad2_ptr, ad2_len))?;
            let data = WasmString::from_caller(&mut caller, (data_ptr, data_len))?;

            tracing::info!(from_desc = ?ad1, to_desc = ?ad2, data = ?data, "transfer");

            let descriptors = caller
                .data()
                .descriptors
                .get(ad1.clone().into_str())
                .zip(caller.data().descriptors.get(ad2.clone().into_str()));

            let (desc1, desc2) = match descriptors {
                Some((desc1, desc2)) => (desc1, desc2),
                None => anyhow::bail!("Descriptor not found"),
            };

            ensure!(desc1.driver_name == desc2.driver_name && desc1.driver_version == desc2.driver_version, "driver mismatch");
            tracing::info!(from_driver = %desc1.driver_name, to_driver = %desc2.driver_name, "transfer");

            let driver_name = desc1.driver_name.clone();
            let driver_version= desc1.driver_version.clone();

            let mut lower_store = wasmtime::Store::new(&driver.engine, PlatformState::default());
            let mut lower_linker = wasmtime::Linker::new(&driver.engine);
            platform.bind(&mut lower_linker)?;

            let driver_list = driver
                .drivers
                .read()
                .map_err(|_| anyhow::anyhow!("lock failed"))?;

            let driver_info=DriverInfo{
                name: driver_name,
                version: driver_version,
            };
            
            let driver_module = driver_list
                .get(&driver_info)
                .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;

            let lower_instance = lower_linker.instantiate(&mut lower_store, driver_module)?;

            let lower_memory = lower_instance
                .get_memory(&mut lower_store, "memory")
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let input1 = serde_json::to_string(&desc1.account_info)?;
            let input2 = serde_json::to_string(&desc2.account_info)?;
            let data = data.into_str();

            let loaded_input1 =
                WasmString::new(&input1).allocate_on_wasm(&lower_memory, &mut lower_store)?;
            let loaded_input2 =
                WasmString::new(&input2).allocate_on_wasm(&lower_memory, &mut lower_store)?;
            let loaded_input3 =
                WasmString::new(data).allocate_on_wasm(&lower_memory, &mut lower_store)?;

            let loaded_input = (
                loaded_input1.0,
                loaded_input1.1,
                loaded_input2.0,
                loaded_input2.1,
                loaded_input3.0,
                loaded_input3.1,
            );

            let transfer_caller = lower_instance
                .get_typed_func::<(i32, i32, i32, i32, i32, i32), ()>(
                    &mut lower_store,
                    "transfer",
                )?;

            tracing::info!(from = %input1, to = %input2, data = %data, "transfer");
            transfer_caller.call(&mut lower_store, loaded_input)?;

            Ok(())
        };

        let driver = self.0.clone();
        let platform = self.1.clone();

        let view = move |mut caller: wasmtime::Caller<'_, State>, desc_key: i32, desc_len: i32| {
            tracing::info!(system = "driver", func = "view", "syscall");
            let driver = driver.clone();
            let platform = platform.clone();

            let desc = WasmString::from_caller(&mut caller, (desc_key, desc_len))?;

            let descriptor = caller
                .data()
                .descriptors
                .get(desc.clone().into_str())
                .ok_or_else(|| anyhow::anyhow!("Descriptor not found"))?;

            let driver_name = descriptor.driver_name.clone();
            let driver_version=descriptor.driver_version.clone();
            let account_info = descriptor.account_info.clone();

            let mut lower_store = wasmtime::Store::new(&driver.engine, PlatformState::default());
            let mut lower_linker = wasmtime::Linker::new(&driver.engine);
            platform.bind(&mut lower_linker)?;

            let driver_list = driver
                .drivers
                .read()
                .map_err(|_| anyhow::anyhow!("lock failed"))?;

            let driver_info=DriverInfo{
                name:driver_name,
                version:driver_version,
            };
            
            let driver_module = driver_list
                .get(&driver_info)
                .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;

            let lower_instance = lower_linker.instantiate(&mut lower_store, driver_module)?;

            let lower_memory = lower_instance
                .get_memory(&mut lower_store, "memory")
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let input = serde_json::to_string(&account_info)?;

            let loaded_input =
                WasmString::new(&input).allocate_on_wasm(&lower_memory, &mut lower_store)?;

            let view_caller = lower_instance
                .get_typed_func::<(i32, i32), (i32, i32)>(&mut lower_store, "view")?;

            let output = view_caller.call(&mut lower_store, loaded_input)?;

            let output = WasmString::from_memory(&lower_memory, &lower_store, output)?;

            let memory = caller
                .get_export("memory")
                .and_then(|m| m.into_memory())
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let loaded_str = WasmString::new(output.into_str());

            loaded_str.allocate_on_caller(&memory, &mut caller)
        };

        linker.func_wrap("driver", "intend", intend)?;
        linker.func_wrap("driver", "done", done)?;
        linker.func_wrap("driver", "transfer", transfer)?;
        linker.func_wrap("driver", "view", view)?;

        Ok(())
    }
}

pub struct PlatformState {
    pub wasi: WasiCtx,
}

impl PlatformState {
    pub fn new() -> Self {
        Self {
            wasi: WasiCtxBuilder::new().build(),
        }
    }
}

impl Default for PlatformState {
    fn default() -> Self {
        Self::new()
    }
}
