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


use alloy_primitives::{utils as AlloyUtils, Address, Bytes, FixedBytes, U256};
use ethers::core::types::U256 as EthersU256;
use ethers::{utils};
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::Context;
use apps::{BonsaiProver, TxSender,AaDeployer,AaContract};
use risc0_zkvm::sha::rust_crypto::{Digest as _, Sha256};
use clap::Parser;
use log::info;
use std::str::FromStr;
use std::{collections::HashMap};
use methods::JWT_VALIDATOR_ELF;
use tokio::sync::oneshot;
use warp::{ Filter,Rejection, Reply};
use std::sync::{Arc, Mutex};
use google_oauth::AsyncClient;



sol! {
    interface IBonsaiPay {
        function claim(address payable to, bytes32 claim_id,  bytes calldata seal);
        function executeCall(address payable _to, bytes32 claim_id,  bytes calldata seal);
    }
    interface IAaDemo {
        function execute(address dest, uint256 value, bytes calldata func,  bytes32 claim_id,bytes calldata seal);
        function setOwner(bytes32 _owner);
    }

    struct Input {
        uint256 identity_provider;
        string jwt;
    }

    struct ClaimsData {
        address msg_sender;
        bytes32 claim_id;
    }
    struct Reciever{
        address to;
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: String,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: String,
}
type UserState = Arc<Mutex<HashMap<String, Option<Address>>>>;
const HEADER_XAUTH: &str = "X-Auth-Token";
const HEADER_DEST: &str ="X-DEST";

async fn handle_jwt_authentication(token: String, user_state: UserState) -> Result<(String,String), warp::Rejection> {
    if token.is_empty() {
        return Err(warp::reject::reject());
    }
    let client_id = "280372739368-qv4brva0eiq0v1oo1jtsqdaph6mv7omo.apps.googleusercontent.com";

    info!("Token received: {}", token);
    let client = AsyncClient::new(client_id);
    let payload = client.validate_id_token(token.clone()).await.expect("Could not validate token"); // In production, remember to handle this error.


    let args = Args::parse();
    let (tx, rx) = oneshot::channel();
    let email = payload.email.unwrap();
    let email_clone = email.clone();
    

    // Spawn a new thread for the Bonsai Prover computation
    std::thread::spawn(move || {
        deploy_AA(args,email_clone , user_state, tx);

    });

    match rx.await {
        Ok(_result) => Ok((email,token)),
        Err(_) => Err(warp::reject::reject()),
    }
}
async fn handle_execute(token: String, email: String, user_state: UserState, dest: String) ->Result<impl warp::Reply, warp::Rejection> {
    if token.is_empty() {
        return Err(warp::reject::reject());
    }
   
    info!("Token received: {}", token);


    let args = Args::parse();
    let (tx, rx) = oneshot::channel();
    

    // Spawn a new thread for the Bonsai Prover computation
    std::thread::spawn(move || {
        prove_and_execute_transaction(args, token, email,user_state,dest, tx);
    });

    match rx.await {
        Ok(_result) => Ok(warp::reply()),
        Err(_) => Err(warp::reject::reject()),
    }
}


  


  


fn deploy_AA(
    args: Args,
    email: String,
    user_state: UserState,
    tx: oneshot::Sender<(&str)>,
) {
   
    let mut state = user_state.lock().unwrap();
   

    // Send the calldata to Ethereum.
    if(!state.contains_key(&email)){    
    let deployer = AaDeployer::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &"0x2A662A912A1e11c7Cc9cD2a509dF085335Cd2619",

    ).expect("Failed to create Deployer");
   
    let email_clone = email.clone();

  

   
        let runtime = tokio::runtime::Runtime::new().expect("failed to start new tokio runtime");
        match
        runtime
            .block_on(deployer.deploy()) {
                Ok((receipt)) => {
                    info!(" Deploy Transaction receipt: {:?}", receipt);
                    let addy = receipt.unwrap().contract_address.unwrap();
                    state.insert(email,Some(Address::from_slice( addy.as_bytes() )));
                    info!("Contract address: {:?}", addy);
                }
                Err(e) => {
                    info!("Failed to send tx: {:?}", e);
                }
            };
        let claim_id: FixedBytes<32> =   FixedBytes::from_slice(Sha256::digest(email_clone.as_bytes()).as_slice());

        let calldata = IAaDemo::IAaDemoCalls::setOwner(IAaDemo::setOwnerCall {
              _owner: claim_id
         })
        .abi_encode();   
         let contract_addy = state.get(&email_clone).unwrap().unwrap();


    
        let init_value: EthersU256 = EthersU256::from(utils::parse_units(1000,"szabo").unwrap());

        match 
        runtime
            .block_on(deployer.send(None, &contract_addy.to_string(),Some(init_value))) {
                Ok((receipt)) => {
                    log::info!(" Init Funding Transaction receipt: {:?}", receipt);
                    
                }
                Err(e) => {
                 log::info!("Failed to send tx: {:?}", e);
                 }
              };
        match
         runtime
            .block_on(deployer.send(Some(calldata), &contract_addy.to_string(),None)) {
                Ok((receipt)) => {
                log::info!(" Set Owner Transaction receipt: {:?}", receipt);
                         
                }
                Err(e) => {
                    log::info!("Failed to send tx: {:?}", e);
                }
            };        
    

    };

    tx.send("Deployed successfully")
        .expect("failed to send over channel");
}

fn prove_and_execute_transaction(
    args: Args,
    token: String,
    email:String,
    user_state: UserState,
    dest: String,
    tx: oneshot::Sender<(Vec<u8>, Vec<u8>)>,
) {
    let input = Input {
        identity_provider: U256::ZERO, // Google as the identity provider
        jwt: token,
    };
    

    let (journal, seal) =
        BonsaiProver::prove(JWT_VALIDATOR_ELF, &input.abi_encode())
            .expect("failed to prove on bonsai");

    let seal_clone = seal.clone();
    let mut state = user_state.lock().unwrap();
   
    let contract_addy = state.get(&email).unwrap().unwrap();
    // Send the calldata to Ethereum.
    let aa_contract = AaContract::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &contract_addy.to_string(),
    )
    .expect("failed to create tx sender");

    let claims = ClaimsData::abi_decode(&journal, true)
        .context("decoding journal data")
        .expect("failed to decode");

    info!("Claim ID: {:?}", claims.claim_id);
    info!("Msg Sender: {:?}", claims.msg_sender);
    let test_value: U256 = AlloyUtils::parse_units("10","Twei").unwrap().into();
    

    let calldata = IAaDemo::IAaDemoCalls::execute(IAaDemo::executeCall {
        dest:  claims.msg_sender,
        value: test_value,
        func: Bytes::new(),
        claim_id: claims.claim_id,
        seal: Bytes::from(seal_clone),
    })
    .abi_encode();

    // Send the calldata to Ethereum.
    let runtime = tokio::runtime::Runtime::new().expect("failed to start new tokio runtime");
    runtime
        .block_on(aa_contract.send(calldata))
        .expect("failed to send tx");

    tx.send((journal,  seal))
        .expect("failed to send over channel");
}
fn jwt_authentication_filter(  user_state: UserState) -> impl Filter<Extract = (String,String), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(HEADER_XAUTH))
        .and(with_user_state(user_state.clone()))
        .and_then(|token: String, user_state: UserState| {

        let mut state = user_state.lock().unwrap();
        let user_state_clone = Arc::clone(&user_state);
        async move {
        handle_jwt_authentication(token, user_state_clone).await.map(|(email,token)| (email,token))
        }}).untuple_one()
}


fn auth_filter(user_state: UserState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"])
        .allow_headers(vec!["content-type", "x-auth-token","x-to"])
        .max_age(3600);

    warp::path("deploy" )
        .and(warp::get())
        .and(warp::path::end())
        .and(jwt_authentication_filter(user_state.clone()))
        .and(with_user_state(user_state))
        .map(|email: String,token:String ,user_state:UserState| {
            let mut state = user_state.lock().unwrap();

             warp::reply::json(&state.get(&email).unwrap().unwrap().to_string())

            })
        .with(cors)

   
    
}
fn execute_filter(user_state: UserState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"])
        .allow_headers(vec!["content-type", "x-auth-token","x-dest"])
        .max_age(3600);


    warp::path("execute" )
        .and(warp::get())
        .and(warp::header::<String>(HEADER_DEST))
        .and(warp::path::end())
        .and(jwt_authentication_filter(user_state.clone()))
        .and(with_user_state(user_state))
        .and_then( |dest, email,token ,  user_state: UserState| async move {
            handle_execute(token, email, user_state, dest).await
        })
        .with(cors)    
    
}

fn with_user_state(
    user_state: UserState,
) -> impl Filter<Extract = (UserState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || user_state.clone())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let user_state: UserState = Arc::new(Mutex::new(HashMap::new()));


    let api = auth_filter(user_state.clone());

    let execute_route = execute_filter(user_state.clone());

    // Combine routes
    let routes = api.or(execute_route);
    
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
