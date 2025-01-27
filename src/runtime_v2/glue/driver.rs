use crate::runtime_v2::types::component::driver::component::units::storage::{Host, StorageError};
use crate::runtime_v2::types::DriverState;

impl Host for DriverState {
    fn get(&mut self, key: String) -> Result<String, StorageError> {
        let output =
            self.platform.storage.get(&key).map_err(|e| {
                StorageError::SystemError(format!("Failed while getting key: {:?}", e))
            })?;
        match output {
            Some(value) => Ok(value),
            None => Err(StorageError::NotFound(format!("Key not found: {}", key))),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StorageError> {
        self.platform
            .storage
            .set(&key, &value)
            .map_err(|e| StorageError::SystemError(format!("Failed while setting key: {:?}", e)))
    }
}
