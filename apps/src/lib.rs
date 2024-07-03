// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// The following library provides utility functions to help with sending
// off-chain proof requests to the Bonsai proving service and publish the
// received proofs directly to a deployed app contract on Ethereum.
//
// Please note that both `risc0_zkvm` and `bonsai_sdk` crates are still
// under active development. As such, this library might change to adapt to
// the upstream changes.

use std::time::Duration;
use std::sync::Arc;
use alloy_sol_types::sol;
use alloy_primitives::FixedBytes;
use alloy_provider::{ProviderBuilder, ProviderLayer,Provider,ReqwestProvider};
use alloy_network::EthereumWallet;
use alloy_signer_local::PrivateKeySigner;
use url::Url;
use anyhow::{Context, Result};
use bonsai_sdk::alpha as bonsai_sdk;
use ethers::prelude::*;
use ethers::contract::abigen;
use risc0_ethereum_contracts::groth16::abi_encode;
use risc0_zkvm::{compute_image_id, Receipt};


sol!{
    #[allow(missing_docs)]
    #[sol(rpc)]
    AADemo,
     "./src/AADemo.json"
    
    }

/// Wrapper of a `SignerMiddleware` client to send transactions to the given
/// contract's `Address`.
pub struct TxSender {
    chain_id: u64,
    provider: FillProvider<T>, T>, T>, T<T>, T<T>, T<T>, T>,
    contract: Address,
}

impl TxSender {
    /// Creates a new `TxSender`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let url = Url::parse(rpc_url)?;
        let signer: PrivateKeySigner = private_key.parse().expect("failed to parse private key");
        let wallet = EthereumWallet::from(signer);
        let provider =ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(url);
        // let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        // let contract = contract.parse::<Address>()?;
        // log::info!("tx Address: {:?}",client.address());

        Ok(TxSender {
            chain_id,
            client,
            contract,
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata).gas(U256::from(2_000_000)).value(U256::from(1_000));

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }

    
}



pub struct AaDeployer {
    chain_id: u64,
    provider: Provider<ReqwestProvider<Holesky>, Http<Client>,T>,
    groth_contract: Address,
}

impl AaDeployer {
    /// Creates a new `AaDeployer`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, groth_contract: &str) -> Result<Self> {
        let url = Url::parse(rpc_url)?;
        let signer: PrivateKeySigner = private_key.parse().expect("failed to parse private key");
        let wallet = EthereumWallet::from(signer);
        let provider =ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(url);;
        let groth_contract:Address = groth_contract.parse::<Address>()?;
        log::info!("tx Address: {:?}",provider.default_signer_address());

        Ok(AaDeployer {
            chain_id,
            provider: Box::new(provider),
            groth_contract,
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn deploy(&self) -> Result<Option<(TransactionReceipt)>>  {
        let aa_demo_contract = AaDemo::deploy(self.client.clone(),self.groth_contract).unwrap();
     

        let (contract_instance, receipt)  =  aa_demo_contract.send_with_receipt().await?;

        log::info!("Transaction receipt: {:?}", &receipt);
        //I have two choices, I can either call send here or I can call from a level above. 
        // If I call it from a level above I need to send out
        Ok(Some(receipt))
    }

    pub async fn send(&self, calldata: Option<Vec<u8>>,contract:&str, value: Option<U256>) -> Result<Option<TransactionReceipt>> {
        let contract = contract.parse::<Address>()?;

        let mut tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(contract)
            .from(self.client.address())
            .gas(U256::from(1_000_000));

            // Set value if provided
        if let Some(data) = calldata {
            log::info!("have calldata"); 

            tx =  tx.data(data);
        }
    
            // Set value if provided
        if let Some(val) = value {
           log::info!("have value"); 
           tx = tx.value(val);
        }

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }

    
}



pub struct AaContract {
    chain_id: u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract: Address,
}

impl AaContract {
    /// Creates a new `AaDeployer`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());

        let contract = contract.parse::<Address>()?;

        Ok(AaContract {
            chain_id,
            client,
            contract
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata)
            .gas(U256::from(3_000_000));

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }

    
}
/// An implementation of a Prover that runs on Bonsai.
pub struct BonsaiProver {}
impl BonsaiProver {
    /// Generates a snark proof as a triplet (`Vec<u8>`, 
    /// `Vec<u8>) for the given elf and input.
    pub fn prove(elf: &[u8], input: &[u8]) -> Result<(Vec<u8>,Vec<u8>)> {
        let client = bonsai_sdk::Client::from_env(risc0_zkvm::VERSION)?;
        // Compute the image_id, then upload the ELF with the image_id as its key.
        let image_id = compute_image_id(elf)?;
        let image_id_hex = image_id.to_string();
        client.upload_img(&image_id_hex, elf.to_vec())?;
        log::info!("Image ID: 0x{}", image_id_hex);

        // Prepare input data and upload it.
        let input_id = client.upload_input(input.to_vec())?;

        // Start a session running the prover.
        let session = client.create_session(image_id_hex, input_id, vec![])?;
        log::info!("Created session: {}", session.uuid);
        let _receipt = loop {
            let res = session.status(&client)?;
            if res.status == "RUNNING" {
                log::info!(
                    "Current status: {} - state: {} - continue polling...",
                    res.status,
                    res.state.unwrap_or_default()
                );
                std::thread::sleep(Duration::from_secs(15));
                continue;
            }
            if res.status == "SUCCEEDED" {
                // Download the receipt, containing the output.
                let receipt_url = res
                    .receipt_url
                    .context("API error, missing receipt on completed session")?;

                let receipt_buf = client.download(&receipt_url)?;
                let receipt: Receipt = bincode::deserialize(&receipt_buf)?;

                break receipt;
            }

            panic!(
                "Workflow exited: {} - | err: {}",
                res.status,
                res.error_msg.unwrap_or_default()
            );
        };

        // Fetch the snark.
        let snark_session = client.create_snark(session.uuid)?;
        log::info!("Created snark session: {}", snark_session.uuid);
        let snark_receipt = loop {
            let res = snark_session.status(&client)?;
            match res.status.as_str() {
                "RUNNING" => {
                    log::info!("Current status: {} - continue polling...", res.status,);
                    std::thread::sleep(Duration::from_secs(15));
                    continue;
                }
                "SUCCEEDED" => {
                    break res.output.context("No snark generated :(")?;
                }
                _ => {
                    panic!(
                        "Workflow exited: {} err: {}",
                        res.status,
                        res.error_msg.unwrap_or_default()
                    );
                }
            }
        };

        let snark = snark_receipt.snark;
        log::debug!("Snark proof!: {snark:?}");

        let seal = abi_encode(snark.to_vec()).context("Read seal")?;
        let post_state_digest: FixedBytes<32> = snark_receipt
            .post_state_digest
            .as_slice()
            .try_into()
            .context("Read post_state_digest")?;
        let journal = snark_receipt.journal;

        Ok((journal, seal))
    }
}
