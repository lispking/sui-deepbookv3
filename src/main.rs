// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk::{types::base_types::SuiAddress, SuiClientBuilder};

use sui_deepbookv3::utils::config::{DeepBookConfig, Environment};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());

    let config = DeepBookConfig::new(
        Environment::Testnet,
        SuiAddress::random_for_testing_only(),
        None,
        None,
        None,
        None,
    );
    println!("config: {:#?}", config);

    let coin = config.get_coin("DEEP");
    println!("coin: {:#?}", coin);

    let pool = config.get_pool("DBUSDT_DBUSDC");
    println!("pool: {:#?}", pool);

    Ok(())
}
