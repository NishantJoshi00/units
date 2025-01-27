use crate::runtime_v2::driver::DriverInfo;
use crate::runtime_v2::types;
use crate::runtime_v2::types::component::module::component::units;
use crate::runtime_v2::types::component::module::component::units::driver::DriverError;

impl units::driver::Host for types::ProcessState {
    fn intend(&mut self, input: String) -> Result<String, DriverError> {
        let path_info = self.get_path_info(input)?;

        let driver_info = DriverInfo {
            name: path_info.driver_name.clone(),
            version: path_info.driver_version.clone(),
        };

        let account_info = path_info.account_info.clone();

        let driver = self.get_driver(&driver_info)?;
        let (linker, mut state) = self.get_lower_runtime(driver_info)?;

        let instance =
            types::component::driver::DriverWorld::instantiate(&mut state, &driver, &linker)
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        let result = instance
            .component_units_driver()
            .call_intend(state, &account_info)
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

        Ok(key)
    }

    fn done(&mut self, input: String) -> Result<(), DriverError> {
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

        let driver = self.get_driver(&driver_info)?;
        let (linker, mut state) = self.get_lower_runtime(driver_info)?;

        let instance =
            types::component::driver::DriverWorld::instantiate(&mut state, &driver, &linker)
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        instance
            .component_units_driver()
            .call_done(state, &account_info)
            .map_err(|_| DriverError::SystemError("Failed while calling done".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling done".to_string()))?;

        self.descriptors.remove(&input);

        Ok(())
    }

    fn transfer(&mut self, fro: String, to: String, value: String) -> Result<(), DriverError> {
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

        let driver = self.get_driver(&driver_info)?;
        let (linker, mut state) = self.get_lower_runtime(driver_info)?;

        let instance =
            types::component::driver::DriverWorld::instantiate(&mut state, &driver, &linker)
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        instance
            .component_units_driver()
            .call_transfer(state, &acc_1, &acc_2, &value)
            .map_err(|_| DriverError::SystemError("Failed while calling transfer".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling transfer".to_string()))?;

        Ok(())
    }

    fn view(&mut self, input: String) -> Result<String, DriverError> {
        let descriptor = self.get_descriptor(input)?;
        let account_info = serde_json::to_string(&descriptor.account_info).map_err(|_| {
            DriverError::SystemError("Failed while serializing account info".to_string())
        })?;

        let driver_info = DriverInfo {
            name: descriptor.driver_name.clone(),
            version: descriptor.driver_version.clone(),
        };

        let driver = self.get_driver(&driver_info)?;

        let (linker, mut state) = self.get_lower_runtime(driver_info)?;

        let instance =
            types::component::driver::DriverWorld::instantiate(&mut state, &driver, &linker)
                .map_err(|_| {
                    DriverError::SystemError("Failed while instantiating driver".to_string())
                })?;

        let result = instance
            .component_units_driver()
            .call_view(state, &account_info)
            .map_err(|_| DriverError::SystemError("Failed while calling view".to_string()))?
            .map_err(|_| DriverError::SystemError("Failed while calling view".to_string()))?;

        Ok(result)
    }
}
