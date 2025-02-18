#[allow(warnings)]
mod bindings;

use bindings::{component::units::storage, exports::component::units::driver};

struct Component;

impl driver::Guest for Component {
    fn intend(input: String) -> Result<String, driver::DriverError> {
        Ok(input)
    }

    fn done(_input: String) -> Result<(), driver::DriverError> {
        Ok(())
    }

    fn transfer(fro: String, to: String, value: String) -> Result<(), driver::DriverError> {
        Err(driver::DriverError::SystemError("Not Allowed".to_string()))
    }

    fn view(input: String) -> Result<String, driver::DriverError> {
        Ok(input)
    }
    fn bind(input: String, _existing: Option<String>) -> Result<String, driver::DriverError> {
        let age_proof: AgeProof = serde_json::from_str(&input)
            .map_err(|e| driver::DriverError::InvalidInput(e.to_string()))?;

        let age =
            (2025 - age_proof.year) * 365 + age_proof.month as u16 * 30 + age_proof.day as u16;
        let age_in_years = age / 365;

        serde_json::to_string(&AgeCredentials { age: age_in_years })
            .map_err(|e| driver::DriverError::SystemError(e.to_string()))
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AgeCredentials {
    age: u16,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AgeProof {
    name: String,
    day: u8,
    month: u8,
    year: u16,
}

bindings::export!(Component with_types_in bindings);
