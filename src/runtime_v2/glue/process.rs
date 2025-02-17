use crate::runtime_v2::driver::DriverInfo;
use crate::runtime_v2::types;
use crate::runtime_v2::types::component::module::component::units;
use crate::runtime_v2::types::component::module::component::units::driver::DriverError;

impl units::driver::Host for types::ProcessState {
    async fn intend(&mut self, input: String) -> Result<String, DriverError> {
        tracing::info!(
            loc = "start",
            runtime = "process",
            call = "intend",
            input = input.as_str()
        );
        let path = if let Some(suffix) = input.strip_prefix("~/") {
            format!("/accounts/{}/{}", self.ctx.user_id, suffix)
        } else {
            input.clone()
        };

        let path_info = self.get_path_info(path).await?;

        let driver_info = DriverInfo {
            name: path_info.driver_name.clone(),
            version: path_info.driver_version.clone(),
        };
        let account_info = path_info.account_info.clone();

        let driver = self
            .get_driver(&driver_info, self.driver_runtime.engine.clone())
            .await?;
        let (mut linker, mut state) = self.get_lower_runtime(driver_info)?;
        wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
            types::component::module::component::units::driver::DriverError::SystemError(
                err.to_string(),
            )
        })?;
        let instance =
            types::component::driver::DriverWorld::instantiate_async(&mut state, &driver, &linker)
                .await
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        let result = instance
            .component_units_driver()
            .call_intend(state, &account_info)
            .await
            .map_err(|_| DriverError::SystemError("Failed while calling intend".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling intend".to_string()))?;

        let key = crate::utils::id::new();

        self.descriptors.insert(
            key.clone(),
            types::Descriptor {
                driver_name: path_info.driver_name,
                driver_version: path_info.driver_version,
                account_info: serde_json::from_str(&result).map_err(|_| {
                    DriverError::SystemError("Failed while parsing account info".to_string())
                })?,
            },
        );

        tracing::info!(
            loc = "end",
            runtime = "process",
            call = "intend",
            key = key.as_str()
        );

        Ok(key)
    }

    async fn done(&mut self, input: String) -> Result<(), DriverError> {
        tracing::info!(
            loc = "start",
            runtime = "process",
            call = "done",
            input = input.as_str()
        );
        let descriptor = self
            .descriptors
            .get(&input)
            .ok_or(DriverError::InvalidInput(
                "Failed while finding descriptor".to_string(),
            ))?;

        let driver_info = DriverInfo {
            name: descriptor.driver_name.clone(),
            version: descriptor.driver_version.clone(),
        };

        let account_info = serde_json::to_string(&descriptor.account_info).map_err(|_| {
            DriverError::SystemError("Failed while serializing account info".to_string())
        })?;

        let driver = self
            .get_driver(&driver_info, self.driver_runtime.engine.clone())
            .await?;
        let (mut linker, mut state) = self.get_lower_runtime(driver_info)?;
        wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
            types::component::module::component::units::driver::DriverError::SystemError(
                err.to_string(),
            )
        })?;
        let instance =
            types::component::driver::DriverWorld::instantiate_async(&mut state, &driver, &linker)
                .await
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        instance
            .component_units_driver()
            .call_done(state, &account_info)
            .await
            .map_err(|_| DriverError::SystemError("Failed while calling done".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling done".to_string()))?;

        self.descriptors.remove(&input);

        tracing::info!(
            loc = "end",
            runtime = "process",
            call = "done",
            input = input.as_str()
        );

        Ok(())
    }

    async fn transfer(
        &mut self,
        fro: String,
        to: String,
        value: String,
    ) -> Result<(), DriverError> {
        tracing::info!(
            loc = "start",
            runtime = "process",
            call = "transfer",
            from = fro.as_str(),
            to = to.as_str(),
            value = value.as_str()
        );

        let d_1 = self.get_descriptor(fro)?;
        let d_2 = self.get_descriptor(to)?;

        let acc_1 = serde_json::to_string(&d_1.account_info).map_err(|_| {
            DriverError::SystemError("Failed while serializing account info".to_string())
        })?;
        let acc_2 = serde_json::to_string(&d_2.account_info).map_err(|_| {
            DriverError::SystemError("Failed while serializing account info".to_string())
        })?;

        let driver_info = DriverInfo {
            name: d_1.driver_name.clone(),
            version: d_2.driver_version.clone(),
        };

        assert_eq!(d_1.driver_name, d_2.driver_name);
        assert_eq!(d_1.driver_version, d_2.driver_version);

        let driver = self
            .get_driver(&driver_info, self.driver_runtime.engine.clone())
            .await?;
        let (mut linker, mut state) = self.get_lower_runtime(driver_info)?;
        wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
            types::component::module::component::units::driver::DriverError::SystemError(
                err.to_string(),
            )
        })?;
        let instance =
            types::component::driver::DriverWorld::instantiate_async(&mut state, &driver, &linker)
                .await
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        instance
            .component_units_driver()
            .call_transfer(state, &acc_1, &acc_2, &value)
            .await
            .map_err(|_| DriverError::SystemError("Failed while calling transfer".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling transfer".to_string()))?;

        tracing::info!(loc = "end", runtime = "process", call = "transfer");

        Ok(())
    }

    async fn view(&mut self, input: String) -> Result<String, DriverError> {
        tracing::info!(
            loc = "start",
            runtime = "process",
            call = "view",
            input = input.as_str()
        );

        let descriptor = self.get_descriptor(input)?;
        let account_info = serde_json::to_string(&descriptor.account_info).map_err(|_| {
            DriverError::SystemError("Failed while serializing account info".to_string())
        })?;

        let driver_info = DriverInfo {
            name: descriptor.driver_name.clone(),
            version: descriptor.driver_version.clone(),
        };

        let driver = self
            .get_driver(&driver_info, self.driver_runtime.engine.clone())
            .await?;

        let (mut linker, mut state) = self.get_lower_runtime(driver_info)?;
        wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
            types::component::module::component::units::driver::DriverError::SystemError(
                err.to_string(),
            )
        })?;
        let instance =
            types::component::driver::DriverWorld::instantiate_async(&mut state, &driver, &linker)
                .await
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        let result = instance
            .component_units_driver()
            .call_view(state, &account_info)
            .await
            .map_err(|_| DriverError::SystemError("Failed while calling view".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling view".to_string()))?;

        tracing::info!(
            loc = "end",
            runtime = "process",
            call = "view",
            result = result.as_str()
        );

        Ok(result)
    }

    async fn bind(
        &mut self,
        _input: String,
        _existing: Option<String>,
    ) -> Result<String, DriverError> {
        Err(DriverError::SystemError(
            "Programmability Disabled".to_string(),
        ))
    }
}
