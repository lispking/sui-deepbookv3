use shared_crypto::intent::Intent;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_deepbookv3::{
    transactions::balance_manager::BalanceManagerContract,
    utils::config::{DeepBookConfig, Environment},
};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        base_types::SuiAddress,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Transaction, TransactionData},
    },
    SuiClientBuilder,
};
use utils::retrieve_wallet;

mod utils;

#[tokio::test]
async fn test_create_and_share_balance_manager() {
    let sui_client = SuiClientBuilder::default().build_testnet().await.unwrap();
    println!("Sui testnet version: {}", sui_client.api_version());

    let mut wallet = retrieve_wallet().unwrap();
    let sender = wallet.active_address().unwrap();
    println!("Sender: {}", sender);

    let coins = sui_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await.unwrap();
    let coin = coins.data.into_iter().next().unwrap();

    let config = DeepBookConfig::new(
        Environment::Testnet,
        SuiAddress::random_for_testing_only(),
        None,
        None,
        None,
        None,
    );
    println!("config: {:#?}", config);

    let balance_manager = BalanceManagerContract::new(config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = balance_manager.create_and_share_balance_manager(&mut ptb);

    let builder = ptb.finish();
    println!("{:?}", builder);

    let gas_budget = 10_000_000;
    let gas_price = sui_client
        .read_api()
        .get_reference_gas_price()
        .await
        .unwrap();
    // create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    // 4) sign transaction
    let keystore =
        FileBasedKeystore::new(&sui_config_dir().unwrap().join(SUI_KEYSTORE_FILENAME)).unwrap();
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction()).unwrap();

    // 5) execute the transaction
    print!("Executing the transaction...");
    let transaction_response = sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .unwrap();
    println!("{}", transaction_response);
}
