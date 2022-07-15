use std::str::FromStr;
use anyhow::Error;
use log::info;
use walletconnect::transport::WalletConnect;
use walletconnect::{qr, Client, Metadata, H160};
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

use crate::address::ADDRESS;
pub struct Wallet {
    contract: Contract<WalletConnect<Http>>,
    account: H160,
}

impl Wallet {
    pub async fn new() ->  Result<Self, Error> {
        let client = Client::new(
            "redis-on-blockchain",
            Metadata {
                description: "redis on blockchain".into(),
                url: "https://rob.buhe.dev".parse()?,
                icons: vec!["https://avatars0.githubusercontent.com/u/4210206".parse()?],
                name: "redis on blockchain".into(),
            },
        )?;


        client.ensure_session(qr::print_with_url).await?;

        let wc = WalletConnect::new(client, "3f433221d3db475db058b3875a617fdd").unwrap();
        let web3 = Web3::new(wc);
        
        let accounts = web3.eth().accounts().await?;
        info!("Connected accounts:");
        for account in &accounts {
            info!(" - {:?}", account);
        }


            // Get current balance
        let balance = web3.eth().balance(accounts[0], None).await?;

        info!("Balance: {}", balance);

        let addr = Address::from_str(ADDRESS)?;

        info!("Address:\n  https://ropsten.etherscan.io/address/{}", accounts[0]);
        Ok(Self {
            contract: Contract::from_json(web3.eth(), addr, include_bytes!("../abi/contracts/Redis.sol/Redis.json"))?,
            account: accounts[0],
        })
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), Error> {
         let tx = self.contract
            .call("set", (key.to_string(), value.to_string()), self.account, Options::default())
            .await
            .unwrap();

        info!("{} set {} tx is {}", key, value, tx);
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<String, Error> {
         let value: String = self.contract
            .query("get", (key.to_string(), ), self.account, Options::default(), None)
            .await
            .unwrap();

        info!("{} get: {}", key, value);
        Ok(value)
    }
}