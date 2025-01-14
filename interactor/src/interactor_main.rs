#![allow(non_snake_case)]

mod proxy;
mod config;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const STATE_FILE: &str = "state.toml";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "getRegistrationFee" => interact.registration_fee().await,
        "getRegistrationFeeVip" => interact.registration_fee_vip().await,
        "getParticipants" => interact.participants().await,
        "getVipParticipants" => interact.vip_participants().await,
        "update_normal_registration_fee" => interact.update_normal_registration_fee().await,
        "update_vip_registration_fee" => interact.update_vip_registration_fee().await,
        "register" => interact.register().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let config = Config::new();
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator()).await;
        interactor.set_current_dir_from_workspace("my-neversea-2025");

        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/my-neversea-2025.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    async fn deploy(&mut self) {
        let registration_fee = BigUint::<StaticApi>::from(0u128);
        let vip_registration_fee = BigUint::<StaticApi>::from(0u128);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::MyNeversea2025Proxy)
            .init(registration_fee, vip_registration_fee)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn registration_fee(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MyNeversea2025Proxy)
            .registration_fee()
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn registration_fee_vip(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MyNeversea2025Proxy)
            .registration_fee_vip()
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn participants(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MyNeversea2025Proxy)
            .participants()
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn vip_participants(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MyNeversea2025Proxy)
            .vip_participants()
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn update_normal_registration_fee(&mut self) {
        let new_registration_fee = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MyNeversea2025Proxy)
            .update_normal_registration_fee(new_registration_fee)
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn update_vip_registration_fee(&mut self) {
        let new_registration_fee = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MyNeversea2025Proxy)
            .update_vip_registration_fee(new_registration_fee)
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn register(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MyNeversea2025Proxy)
            .register()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            
            .run()
            .await;

        println!("Result: {response:?}");
    }

}
