// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::SuiClient;

use crate::transactions::balance_manager::BalanceManagerContract;
use crate::transactions::deepbook::DeepBookContract;
use crate::transactions::deepbook_admin::DeepBookAdminContract;
use crate::transactions::flashloan::FlashLoanContract;
use crate::transactions::governance::GovernanceContract;
use crate::utils::config::{
    BalanceManagerMap, CoinMap, DeepBookConfig, Environment, PoolMap, DEEP_SCALAR, FLOAT_SCALAR,
};
use crate::DataReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteQuantityOut {
    pub base_quantity: f64,
    pub base_out: f64,
    pub quote_out: f64,
    pub deep_required: f64,
}

/// DeepBookClient struct for managing DeepBook operations.
pub struct DeepBookClient {
    client: SuiClient,
    config: DeepBookConfig,
    address: SuiAddress,
    pub balance_manager: BalanceManagerContract,
    pub deep_book: DeepBookContract,
    pub deep_book_admin: DeepBookAdminContract,
    pub flash_loans: FlashLoanContract,
    pub governance: GovernanceContract,
}

impl DeepBookClient {
    /// Creates a new DeepBookClient instance
    ///
    /// @param client - The SuiClient instance
    /// @param address - The address of the DeepBook contract
    /// @param env - The environment of the DeepBook contract
    /// @param balance_managers - The balance managers associated with the DeepBook contract
    /// @param coins - The coins associated with the DeepBook contract
    /// @param pools - The pools associated with the DeepBook contract
    /// @param admin_cap - The admin cap associated with the DeepBook contract
    pub fn new(
        client: SuiClient,
        address: SuiAddress,
        env: Environment,
        balance_managers: Option<BalanceManagerMap>,
        coins: Option<CoinMap>,
        pools: Option<PoolMap>,
        admin_cap: Option<String>,
    ) -> Self {
        let config = DeepBookConfig::new(env, address, admin_cap, balance_managers, coins, pools);
        let balance_manager = BalanceManagerContract::new(client.clone(), config.clone());
        Self {
            client: client.clone(),
            address,
            config: config.clone(),
            balance_manager: balance_manager.clone(),
            deep_book: DeepBookContract::new(
                client.clone(),
                config.clone(),
                balance_manager.clone(),
            ),
            deep_book_admin: DeepBookAdminContract::new(client.clone(), config.clone()),
            flash_loans: FlashLoanContract::new(client.clone(), config.clone()),
            governance: GovernanceContract::new(
                client.clone(),
                config.clone(),
                balance_manager.clone(),
            ),
        }
    }

    /// Check the balance of a balance manager for a specific coin
    pub async fn check_manager_balance(
        &self,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<(String, f64)> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let coin = self.config.get_coin(coin_key)?;

        self.balance_manager
            .check_manager_balance(&mut ptb, manager_key, coin_key)
            .await?;
        let res: (Vec<u8>, sui_json_rpc_types::SuiTypeTag) = self
            .client
            .dev_inspect_transaction(self.address, ptb)
            .await?;

        let balance = bcs::from_bytes::<u64>(&res.0)?;
        let adjusted_balance = balance as f64 / coin.scalar as f64;

        Ok((coin.type_name.clone(), (adjusted_balance * 1e9).round() / 1e9))
    }

    // /// Check if a pool is whitelisted
    // pub async fn whitelisted(&self, pool_key: &str) -> Result<bool> {
    //     let mut ptb = ProgrammableTransactionBuilder::new();
    //     self.deep_book.whitelisted(&mut ptb, pool_key).await?;

    //     let res = self
    //         .client
    //         .dev_inspect_transaction(self.address, ptb)
    //         .await?;

    //     let whitelisted = match res.0.try_into() {
    //         Ok(whitelisted) => whitelisted,
    //         Err(_) => return Err(anyhow::anyhow!("Failed to get whitelist status")),
    //     };
    //     Ok(whitelisted != 0)
    // }

    // /// Get the quote quantity out for a given base quantity
    // pub async fn get_quote_quantity_out(
    //     &self,
    //     pool_key: &str,
    //     base_quantity: f64,
    // ) -> Result<QuoteQuantityOut> {
    //     let pool = self.config.get_pool(pool_key)?;
    //     let base_scalar = self.config.get_coin(&pool.base_coin)?.scalar;
    //     let quote_scalar = self.config.get_coin(&pool.quote_coin)?.scalar;

    //     let mut ptb = ProgrammableTransactionBuilder::new();
    //     self.deep_book
    //         .get_quote_quantity_out(&mut ptb, pool_key, base_quantity)
    //         .await?;
    //     let res = self
    //         .client
    //         .dev_inspect_transaction(self.address, ptb)
    //         .await?;

    //     let results = res
    //         .results
    //         .and_then(|r| r.first())
    //         .and_then(|r| r.return_values)
    //         .ok_or_else(|| anyhow::anyhow!("Failed to get quote quantity"))?;

    //     let base_out = u64::from_le_bytes(results[0].clone().try_into()?);
    //     let quote_out = u64::from_le_bytes(results[1].clone().try_into()?);
    //     let deep_required = u64::from_le_bytes(results[2].clone().try_into()?);

    //     Ok(QuoteQuantityOut {
    //         base_quantity,
    //         base_out: (base_out as f64 / base_scalar as f64 * 1e9).round() / 1e9,
    //         quote_out: (quote_out as f64 / quote_scalar as f64 * 1e9).round() / 1e9,
    //         deep_required: (deep_required as f64 / DEEP_SCALAR as f64 * 1e9).round() / 1e9,
    //     })
    // }

    // /// Get open orders for a balance manager in a pool
    // pub async fn account_open_orders(
    //     &self,
    //     pool_key: &str,
    //     manager_key: &str,
    // ) -> Result<Vec<u128>> {
    //     let mut tx = Transaction::new();
    //     tx.add(self.deep_book.account_open_orders(pool_key, manager_key)?);

    //     let res = self
    //         .client
    //         .dev_inspect_transaction_block(self.address, tx)
    //         .await?;

    //     let order_ids = res
    //         .results
    //         .and_then(|r| r.first())
    //         .and_then(|r| r.return_values.first())
    //         .and_then(|r| r.first())
    //         .ok_or_else(|| anyhow::anyhow!("Failed to get order IDs"))?;

    //     // Parse the VecSet of u128 values
    //     // Note: This implementation assumes a specific binary format and may need adjustment
    //     let mut result = Vec::new();
    //     let mut offset = 0;
    //     while offset + 16 <= order_ids.len() {
    //         let mut bytes = [0u8; 16];
    //         bytes.copy_from_slice(&order_ids[offset..offset + 16]);
    //         result.push(u128::from_le_bytes(bytes));
    //         offset += 16;
    //     }

    //     Ok(result)
    // }
}
