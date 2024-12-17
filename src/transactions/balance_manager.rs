// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::{parse_sui_struct_tag, Identifier, TypeTag, SUI_FRAMEWORK_PACKAGE_ID};

use crate::utils::config::DeepBookConfig;

/// BalanceManagerContract struct for managing BalanceManager operations.
pub struct BalanceManagerContract {
    config: DeepBookConfig,
}

impl BalanceManagerContract {
    /// Creates a new instance of BalanceManagerContract
    pub fn new(config: DeepBookConfig) -> Self {
        Self { config }
    }

    /// Create and share a new BalanceManager
    pub fn create_and_share_balance_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("new")?,
            vec![],
            vec![],
        );

        let manager_tag = TypeTag::from_str(format!(
            "{}::balance_manager::BalanceManager",
            self.config.deepbook_package_id()
        ).as_str())?;

        ptb.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            Identifier::new("transfer")?,
            Identifier::new("public_share_object")?,
            vec![manager_tag],
            vec![manager],
        );
        Ok(())
    }

    // /// Deposit funds into the BalanceManager
    // pub async fn deposit_into_manager(
    //     &self,
    //     manager_key: &str,
    //     coin_key: &str,
    //     amount_to_deposit: u64,
    // ) -> anyhow::Result<()> {
    //     let tx = self.client.transaction_builder();
    //     let signer = self.config.address().clone();
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address.clone();
    //     let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
    //     let coin = self.config.get_coin(coin_key)?.clone();
    //     let deposit_input = amount_to_deposit * coin.scalar;

    //     tx.move_call(
    //         signer,
    //         package_id,
    //         "balance_manager",
    //         "deposit",
    //         vec![
    //             SuiTypeTag::new(coin.type_str)
    //         ],
    //         vec![
    //             SuiJsonValue::new(json!(manager_id.to_string()))?,
    //             SuiJsonValue::new(json!(deposit_input))?,
    //         ],
    //         None,
    //         10_000_000,
    //         None,
    //     ).await?;

    //     Ok(())
    // }

    // /// Withdraw funds from the BalanceManager
    // pub fn withdraw_from_manager(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    //     coin_key: &str,
    //     amount_to_withdraw: u64,
    //     recipient: SuiAddress,
    // ) -> Result<()> {
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address;
    //     let coin = self.config.get_coin(coin_key)?;
    //     let withdraw_input = (amount_to_withdraw * coin.scalar).round() as u64;

    //     let coin_object = tx.move_call(
    //         format!("{}::balance_manager::withdraw", self.config.deepbook_package_id()),
    //         vec![
    //             tx.object(manager_id.to_string()),
    //             tx.pure_u64(withdraw_input),
    //         ],
    //         vec![coin.type_str],
    //     )?;

    //     tx.transfer_objects(vec![coin_object], recipient)?;
    //     Ok(())
    // }

    // /// Withdraw all funds from the BalanceManager
    // pub fn withdraw_all_from_manager(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    //     coin_key: &str,
    //     recipient: SuiAddress,
    // ) -> Result<()> {
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address;
    //     let coin = self.config.get_coin(coin_key)?;

    //     let withdrawal_coin = tx.move_call(
    //         format!("{}::balance_manager::withdraw_all", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id.to_string())],
    //         vec![coin.type_str],
    //     )?;

    //     tx.transfer_objects(vec![withdrawal_coin], recipient)?;
    //     Ok(())
    // }

    // /// Check the balance of the BalanceManager
    // pub fn check_manager_balance(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    //     coin_key: &str,
    // ) -> Result<()> {
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address;
    //     let coin = self.config.get_coin(coin_key)?;

    //     tx.move_call(
    //         format!("{}::balance_manager::balance", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id.to_string())],
    //         vec![coin.type_str],
    //     )?;

    //     Ok(())
    // }

    // /// Generate a trade proof for the BalanceManager
    // pub fn generate_proof(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    // ) -> Result<()> {
    //     let balance_manager = self.config.get_balance_manager(manager_key)?;

    //     if let Some(trade_cap) = balance_manager.trade_cap {
    //         self.generate_proof_as_trader(tx, &balance_manager.address, &trade_cap)?;
    //     } else {
    //         self.generate_proof_as_owner(tx, &balance_manager.address)?;
    //     }

    //     Ok(())
    // }

    // /// Generate a trade proof as the owner
    // pub fn generate_proof_as_owner(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_id: &str,
    // ) -> Result<()> {
    //     tx.move_call(
    //         format!("{}::balance_manager::generate_proof_as_owner", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id)],
    //         vec![],
    //     )?;

    //     Ok(())
    // }

    // /// Generate a trade proof as a trader
    // pub fn generate_proof_as_trader(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_id: &str,
    //     trade_cap_id: &str,
    // ) -> Result<()> {
    //     tx.move_call(
    //         format!("{}::balance_manager::generate_proof_as_trader", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id), tx.object(trade_cap_id)],
    //         vec![],
    //     )?;

    //     Ok(())
    // }

    // /// Get the owner of the BalanceManager
    // pub fn owner(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    // ) -> Result<()> {
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address;

    //     tx.move_call(
    //         format!("{}::balance_manager::owner", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id.to_string())],
    //         vec![],
    //     )?;

    //     Ok(())
    // }

    // /// Get the ID of the BalanceManager
    // pub fn id(
    //     &self,
    //     tx: &mut Transaction,
    //     manager_key: &str,
    // ) -> Result<()> {
    //     let manager_id = self.config.get_balance_manager(manager_key)?.address;

    //     tx.move_call(
    //         format!("{}::balance_manager::id", self.config.deepbook_package_id()),
    //         vec![tx.object(manager_id.to_string())],
    //         vec![],
    //     )?;

    //     Ok(())
    // }
}
