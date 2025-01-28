

#[allow(warnings)]
mod bindings;

use bindings::{component::units::storage, exports::component::units::driver};

struct Component;

impl driver::Guest for Component {
    fn intend(input: String) -> Result<String, driver::DriverError> {
        let account: AccountInfo = serde_json::from_str(&input).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let stored = storage::get(&account.name).unwrap_or(input.to_string());
        storage::set(&account.name, &stored).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        Ok(stored)
    }

    fn done(_input: String) -> Result<(), driver::DriverError> {
        Ok(())
    }

    fn transfer(
        fro: String,
        to: String,
        value: String,
    ) -> Result<(), driver::DriverError> {
        let mut from_acc = serde_json::from_str::<AccountInfo>(&fro).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let mut to_acc = serde_json::from_str::<AccountInfo>(&to).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?;
        let diff = serde_json::from_str::<Data>(&value).map_err(|e| {
            driver::DriverError::InvalidInput(e.to_string())
        })?.amount;

        from_acc.amount -= diff;
        to_acc.amount += diff;

        storage::set(&from_acc.name, &serde_json::to_string(&from_acc).unwrap()).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        storage::set(&to_acc.name, &serde_json::to_string(&to_acc).unwrap()).map_err(|e| {
            driver::DriverError::SystemError(e.to_string())
        })?;
        Ok(())
    }

    fn view(input: String) -> Result<String, driver::DriverError> {
        Ok(input)
    }
    fn bind(
        input: String,
        _existing: Option<String>,
    ) -> Result<String, driver::DriverError> {
        Ok(input)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AccountInfo {
    name: String,
    amount: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data {
    amount: u64,
}


bindings::export!(Component with_types_in bindings);
