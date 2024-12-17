// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use sui_json_rpc_types::{SuiObjectData, SuiObjectDataOptions};
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::ObjectArg;
use sui_sdk::types::{Identifier, TypeTag, SUI_FRAMEWORK_PACKAGE_ID};
use sui_sdk::SuiClient;

use crate::utils::config::DeepBookConfig;

/// BalanceManagerContract struct for managing BalanceManager operations.
pub struct BalanceManagerContract {
    config: DeepBookConfig,
    client: SuiClient,
}

impl BalanceManagerContract {
    /// Creates a new instance of BalanceManagerContract
    pub fn new(config: DeepBookConfig, client: SuiClient) -> Self {
        Self { config, client }
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

        let manager_tag = TypeTag::from_str(
            format!(
                "{}::balance_manager::BalanceManager",
                self.config.deepbook_package_id()
            )
            .as_str(),
        )?;

        ptb.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            Identifier::new("transfer")?,
            Identifier::new("public_share_object")?,
            vec![manager_tag],
            vec![manager],
        );
        Ok(())
    }

    /// Deposit funds into the BalanceManager
    pub async fn deposit_into_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_deposit: u64,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?.clone();
        let deposit_input = amount_to_deposit * coin.scalar;
        // TODO: input coin

        let manager_key = ptb.obj(self.share_object_mutable(manager_id).await?)?;
        let deposit = ptb.pure(deposit_input)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("deposit")?,
            vec![TypeTag::from_str(coin.type_str.as_str())?],
            vec![manager_key, deposit],
        );

        Ok(())
    }

    /// Withdraw funds from the BalanceManager
    pub async fn withdraw_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_withdraw: u64,
        recipient: SuiAddress,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;
        let withdraw_input = amount_to_withdraw * coin.scalar;
        // TODO: input coin

        let manager_key = ptb.obj(self.share_object_mutable(manager_id).await?)?;
        let withdraw = ptb.pure(withdraw_input)?;
        let coin_object = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("withdraw")?,
            vec![TypeTag::from_str(coin.type_str.as_str())?],
            vec![manager_key, withdraw],
        );

        ptb.transfer_arg(recipient, coin_object);
        Ok(())
    }

    /// Withdraw all funds from the BalanceManager
    pub async fn withdraw_all_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        recipient: SuiAddress,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;

        let manager_key = ptb.obj(self.share_object_mutable(manager_id).await?)?;
        let withdrawal_coin = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("withdraw_all")?,
            vec![TypeTag::from_str(coin.type_str.as_str())?],
            vec![manager_key],
        );

        ptb.transfer_arg(recipient, withdrawal_coin);
        Ok(())
    }

    /// Check the balance of the BalanceManager
    pub async fn check_manager_balance(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;

        let manager_key = ptb.obj(self.share_object(manager_id).await?)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("balance")?,
            vec![TypeTag::from_str(coin.type_str.as_str())?],
            vec![manager_key],
        );
        Ok(())
    }

    /// Generate a trade proof for the BalanceManager
    pub async fn generate_proof(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let balance_manager = self.config.get_balance_manager(manager_key)?;
        let manager_address = balance_manager.address.as_str();
        let trade_cap = balance_manager.trade_cap.clone();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;

        if let Some(trade_cap) = trade_cap {
            let trade_cap_id = ObjectID::from_hex_literal(trade_cap.as_str())?;
            self.generate_proof_as_trader(ptb, &manager_id, &trade_cap_id).await?;
        } else {
            self.generate_proof_as_owner(ptb, &manager_id).await?;
        }
        Ok(())
    }

    /// Generate a trade proof as the owner
        pub async fn generate_proof_as_owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &ObjectID,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_key = ptb.obj(self.share_object(*manager_id).await?)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("generate_proof_as_owner")?,
            vec![],
            vec![manager_key],
        );
        Ok(())
    }

    /// Generate a trade proof as a trader
    pub async fn generate_proof_as_trader(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &ObjectID,
        trade_cap_id: &ObjectID,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_key = ptb.obj(self.share_object(*manager_id).await?)?;
        let trade_cap_key = ptb.obj(self.share_object(*trade_cap_id).await?)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("generate_proof_as_trader")?,
            vec![],
            vec![manager_key, trade_cap_key],
        );
        Ok(())
    }

    /// Get the owner of the BalanceManager
    pub async fn owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let manager_key = ptb.obj(self.share_object(manager_id).await?)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("owner")?,
            vec![],
            vec![manager_key],
        );
        Ok(())
    }

    /// Get the ID of the BalanceManager
    pub async fn id(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let manager_key = ptb.obj(self.share_object(manager_id).await?)?;
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("id")?,
            vec![],
            vec![manager_key],
        );
        Ok(())
    }

    async fn share_object(&self, manager_id: ObjectID) -> anyhow::Result<ObjectArg> {
        let object = self.get_object(manager_id).await?;
        Ok(ObjectArg::SharedObject {
            id: manager_id,
            initial_shared_version: object.version,
            mutable: false,
        })
    }

    async fn share_object_mutable(&self, manager_id: ObjectID) -> anyhow::Result<ObjectArg> {
        let object = self.get_object(manager_id).await?;
        Ok(ObjectArg::SharedObject {
            id: manager_id,
            initial_shared_version: object.version,
            mutable: true,
        })
    }

    pub async fn get_object(&self, object_id: ObjectID) -> anyhow::Result<SuiObjectData> {
        self
            .client
            .read_api()
            .get_object_with_options(object_id, SuiObjectDataOptions::full_content())
            .await?
            .data
            .ok_or(anyhow::anyhow!("Object {} not found", object_id))
    }
}
