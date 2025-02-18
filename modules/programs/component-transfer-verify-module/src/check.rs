use super::bindings::component::units::driver;
use super::bindings::UserError;

pub fn check_credential(cred_path: &str, over: u16) -> Result<bool, UserError> {
    let cred = driver::intend(cred_path).map_err(|e| UserError::UnknownError(e.to_string()))?;
    let data = driver::view(&cred).map_err(|e| UserError::UnknownError(e.to_string()))?;
    let proof = serde_json::from_str::<AgeProof>(&data)
        .map_err(|e| UserError::InvalidInput(e.to_string()))?;
    driver::done(&cred).map_err(|e| UserError::UnknownError(e.to_string()))?;
    Ok(proof.age >= over)
}

#[derive(serde::Deserialize)]
struct AgeProof {
    age: u16,
}
