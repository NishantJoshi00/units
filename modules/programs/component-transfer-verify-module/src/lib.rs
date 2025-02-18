#[allow(warnings)]
mod bindings;
mod check;

use bindings::{component::units::driver, Guest};

struct Component;

impl Guest for Component {
    fn main(input: String) -> Result<String, bindings::UserError> {
        let input = serde_json::from_str::<Input>(&input)
            .map_err(|e| bindings::UserError::InvalidInput(e.to_string()))?;

        let p1 = driver::intend(&input.path1)
            .map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;

        let p2 = driver::intend(&input.path2)
            .map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;

        if !check::check_credential(&input.cred_path, 18)? {
            return Err(bindings::UserError::InvalidInput(
                "Age is less than 18".to_string(),
            ));
        }

        let data = Data {
            amount: input.amount,
        };

        let data = serde_json::to_string(&data)
            .map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;

        driver::transfer(&p1, &p2, &data)
            .map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;

        driver::done(&p1).map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;
        driver::done(&p2).map_err(|e| bindings::UserError::UnknownError(e.to_string()))?;
        let output = format!(" Transfer successful of {} rupees", input.amount);

        Ok(output.to_string())
    }
}

#[derive(serde::Deserialize)]
struct Input {
    path1: String,
    path2: String,
    cred_path: String,
    amount: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    amount: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MoreData {
    name: String,
    amount: u64,
}

bindings::export!(Component with_types_in bindings);
