#[allow(warnings)]
mod bindings;

use bindings::{
    component::units::driver::{self},
    Guest,
};

struct Component;

impl Guest for Component {
    fn main(input: String) -> Result<String, bindings::UserError> {
        let input: OnlyPath = serde_json::from_str(&input)
            .map_err(|e| bindings::UserError::InvalidInput(e.to_string()))?;
        let p1 = driver::intend(&input.path)
            .map_err(|e| bindings::UserError::InvalidInput(e.to_string()))?;

        let p1_data = driver::view(&p1)
            .map_err(|e| bindings::UserError::SystemError(format!("Failed to view data: {}", e)))?;

        // let p1_d = serde_json::from_str::<MoreData>(&p1_data).map_err(|e| {
        //     bindings::UserError::SystemError(format!("Failed to parse data: {}", e))
        // })?;

        // let data = ViewData {
        //     data: p1_d,
        //     path: input.path.clone(),
        // };

        let data = serde_json::to_string(&p1_data).map_err(|e| {
            bindings::UserError::SystemError(format!("Failed to serialize data: {}", e))
        })?;

        driver::done(&p1).map_err(|e| {
            bindings::UserError::SystemError(format!("Failed to mark done: {}", e))
        })?;

        Ok(data)
    }
}

#[derive(serde::Serialize)]
struct ViewData {
    path: String,
    data: MoreData,
}

#[derive(serde::Deserialize)]
struct OnlyPath {
    path: String,
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
