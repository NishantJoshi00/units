use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TransferRequest {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct TransferResponse {
    signature: String,
}

pub fn transfer_token(key: String, value: String) -> Result<TransferResponse> {
    let client = Client::new();

    let request = TransferRequest { key, value };

    let response = client
        .post("https://finternet-solana-apis-production.up.railway.app/token/transfer")
        .json(&request)
        .send()?
        .json::<TransferResponse>()?;

    Ok(response)
}
