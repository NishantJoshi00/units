use std::{error::Error, sync::Arc};

use anyhow::anyhow;
use rand::random;
use starknet::accounts::{Account, ConnectedAccount};
use starknet::core::types::contract::SierraClassDebugInfo;
use starknet::core::types::{
    BlockTag, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV3, DataAvailabilityMode,
    ResourceBounds, ResourceBoundsMapping,
};
use starknet::{
    accounts::{AccountError, ExecutionEncoding, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{
        contract::SierraClass, BlockId, BroadcastedDeclareTransactionV3, Felt, FlattenedSierraClass,
    },
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, ProviderError, Url,
    },
    signers::{LocalWallet, SigningKey},
};

use super::types::ProvableConfig;
#[derive(Debug, Clone)]
pub struct ProvableRuntime {
    pub provider: Arc<JsonRpcClient<HttpTransport>>,
    pub operator_account: Arc<SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>>,
}

impl ProvableRuntime {
    pub fn new(config: ProvableConfig) -> Self {
        let provider = Arc::new(JsonRpcClient::new(HttpTransport::new(
            Url::parse(&config.url).unwrap(),
        )));
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            Felt::from_hex(&config.operator_private_key).unwrap(),
        ));
        let address = Felt::from_hex(&config.operator_address).unwrap();
        let mut operator_account: SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, _> =
            SingleOwnerAccount::new(
                provider.clone(),
                signer,
                address,
                Felt::from_hex_unchecked(&config.chain_id),
                ExecutionEncoding::New,
            );
        operator_account.set_block_id(BlockId::Tag(starknet::core::types::BlockTag::Pending));
        Self {
            provider,
            operator_account: Arc::new(operator_account),
        }
    }

    pub fn get_provider(&self) -> &JsonRpcClient<HttpTransport> {
        self.provider.as_ref()
    }

    pub fn get_operator_account(
        &self,
    ) -> &SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet> {
        self.operator_account.as_ref()
    }

    pub async fn deploy_account(
        &self,
        public_key: String,
        program_hash: String,
    ) -> Result<String, Box<dyn Error>> {
        tracing::info!("Deploying program with public key {}", public_key);
        let deployment = self.deploy_program(program_hash, vec![public_key]).await?;
        tracing::info!("Program deployment successful, address: {}", deployment);
        Ok(deployment)
    }

    pub async fn declare_program(
        &self,
        class: Vec<u8>,
        compiled_class_hash: String,
    ) -> anyhow::Result<String> {
        let program: SierraClass = serde_json::from_slice(class.as_slice()).map_err(|e| {
            println!("Error: {:?}", e);
            tracing::error!(error = %e, "Failed to deserialize driver binary");
            Box::new(e)
        })?;

        // check if program is already declared
        let class_hash = program.class_hash()?;
        let program_exists = self
            .provider
            .get_class(BlockId::Tag(BlockTag::Pending), class_hash.clone())
            .await;

        if program_exists.is_ok() {
            return Ok(class_hash.to_hex_string());
        }

        let flattened_program = program.flatten().unwrap();

        let result = self
            .operator_account
            .declare_v3(
                Arc::new(flattened_program),
                Felt::from_hex_unchecked(&compiled_class_hash),
            )
            .send()
            .await
            .unwrap();

        tracing::info!(
            "Contract declaration successful, txn hash: {}",
            result.transaction_hash
        );

        Ok(result.class_hash.to_hex_string())
    }

    pub async fn declare_and_deploy_program(
        &self,
        class: Vec<u8>,
        compiled_class_hash: String,
        constructor_calldata: Vec<String>,
    ) -> anyhow::Result<String> {
        let class_hash = self.declare_program(class, compiled_class_hash).await?;
        tracing::info!("Declare was succesful, class hash {:?}", class_hash);
        let deploy_address = self
            .deploy_program(class_hash, constructor_calldata)
            .await?;
        Ok(deploy_address)
    }

    pub async fn deploy_program(
        &self,
        class_hash: String,
        constructor_calldata: Vec<String>,
    ) -> anyhow::Result<String> {
        tracing::info!("Creating contract factory for program deployment");
        let contract_factory = ContractFactory::new(
            // hash of the account program
            Felt::from_hex_unchecked(&class_hash),
            self.get_operator_account(),
        );

        let salt: u128 = random();
        let deployment = contract_factory.deploy_v3(
            constructor_calldata
                .into_iter()
                .map(|s| Felt::from_hex_unchecked(s.as_str()))
                .collect(),
            // TODO: should this be random?
            salt.into(),
            true,
        );
        let address = deployment.deployed_address();
        let transaction_hash = deployment
            .gas_price(0)
            .send()
            .await
            .map_err(|e| {
                println!("Error: {:?}", e);
                tracing::error!(error = %e, "Failed to deploy program");
                e
            })?
            .transaction_hash;
        tracing::info!(
            "Contract deployment successful, txn hash: {}",
            transaction_hash.to_hex_string()
        );
        Ok(address.to_hex_string())
    }

    pub async fn execute_program(
        &self,
        user_address: String,
        input: String,
        signature: String,
    ) -> anyhow::Result<String> {
        let user_address = Felt::from_hex_unchecked(user_address.as_str());

        let txn_input = BroadcastedInvokeTransactionV3 {
            sender_address: user_address.clone(),
            calldata: input
                .split(",")
                .into_iter()
                .map(|s| Felt::from_hex_unchecked(s))
                .collect(),
            signature: signature
                .split(",")
                .into_iter()
                .map(|s| Felt::from_hex_unchecked(s))
                .collect(),
            nonce: self
                .provider
                .get_nonce(BlockId::Tag(BlockTag::Pending), user_address)
                .await?,
            resource_bounds: ResourceBoundsMapping {
                l1_gas: ResourceBounds {
                    max_amount: 0,
                    max_price_per_unit: 0,
                },
                l2_gas: ResourceBounds {
                    max_amount: 0,
                    max_price_per_unit: 0,
                },
            },
            tip: 0,
            paymaster_data: vec![],
            account_deployment_data: vec![],
            nonce_data_availability_mode: DataAvailabilityMode::L1,
            fee_data_availability_mode: DataAvailabilityMode::L1,
            is_query: false,
        };

        println!("this is txn input {:?}", txn_input);

        let invoke_result = match self
            .provider
            .add_invoke_transaction(BroadcastedInvokeTransaction::V3(txn_input))
            .await
        {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(anyhow!("Failed to execute program"));
            }
        };

        Ok(invoke_result.transaction_hash.to_hex_string())
    }
}
