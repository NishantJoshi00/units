use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TransferRequest {
    pub key: String,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TransferResponse {
    signature: String,
}

pub fn transfer_token(key: String, value: String) -> Result<TransferResponse> {
    let function = || async move {
        let request = TransferRequest { key, value };
        let response =
            surf::post("https://finternet-solana-apis-production.up.railway.app/token/transfer")
                .body_json(&request)
                .map_err(|err| anyhow::anyhow!("response error: {:?}", err))?
                .recv_json::<TransferResponse>()
                .await
                .map_err(|err| anyhow::anyhow!("json error: {:?}", err))?;
        Ok::<_, anyhow::Error>(response)
    };
    let output = smol::block_on(function())?;
    Ok(output)
}
