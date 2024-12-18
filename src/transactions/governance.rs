// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::utils::{config::{DeepBookConfig, DEEP_SCALAR, FLOAT_SCALAR}, types::ProposalParams};

/// GovernanceContract struct for managing governance operations in DeepBook.
pub struct GovernanceContract {
    config: DeepBookConfig,
}

impl GovernanceContract {
    /// Creates a new GovernanceContract instance
    /// 
    /// # Arguments
    /// * `config` - Configuration for GovernanceContract
    pub fn new(config: DeepBookConfig) -> Self {
        Self { config }
    }

    // /// Stake a specified amount in the pool
    // /// 
    // /// # Arguments
    // /// * `pool_key` - The key to identify the pool
    // /// * `balance_manager_key` - The key to identify the BalanceManager
    // /// * `stake_amount` - The amount to stake
    // pub fn stake<F>(&self, pool_key: &str, balance_manager_key: &str, stake_amount: f64) -> impl FnOnce(&mut Transaction) -> () {
    //     let pool = self.config.get_pool(pool_key);
    //     let balance_manager = self.config.get_balance_manager(balance_manager_key);
    //     let base_coin = self.config.get_coin(&pool.base_coin);
    //     let quote_coin = self.config.get_coin(&pool.quote_coin);
    //     let stake_input = (stake_amount * DEEP_SCALAR as f64).round() as u64;

    //     move |tx: &mut Transaction| {
    //         let trade_proof = tx.add(self.config.balance_manager.generate_proof(balance_manager_key));
            
    //         tx.move_call(
    //             format!("{}::pool::stake", self.config.deepbook_package_id()),
    //             vec![base_coin.type_name.clone(), quote_coin.type_name.clone()],
    //             vec![
    //                 tx.object(&pool.address),
    //                 tx.object(&balance_manager.address),
    //                 trade_proof,
    //                 tx.pure_u64(stake_input),
    //             ],
    //         );
    //     }
    // }

    // /// Unstake from the pool
    // /// 
    // /// # Arguments
    // /// * `pool_key` - The key to identify the pool
    // /// * `balance_manager_key` - The key to identify the BalanceManager
    // pub fn unstake<F>(&self, pool_key: &str, balance_manager_key: &str) -> impl FnOnce(&mut Transaction) -> () {
    //     let pool = self.config.get_pool(pool_key);
    //     let balance_manager = self.config.get_balance_manager(balance_manager_key);
    //     let base_coin = self.config.get_coin(&pool.base_coin);
    //     let quote_coin = self.config.get_coin(&pool.quote_coin);

    //     move |tx: &mut Transaction| {
    //         let trade_proof = tx.add(self.config.balance_manager.generate_proof(balance_manager_key));
            
    //         tx.move_call(
    //             format!("{}::pool::unstake", self.config.deepbook_package_id()),
    //             vec![base_coin.type_name.clone(), quote_coin.type_name.clone()],
    //             vec![
    //                 tx.object(&pool.address),
    //                 tx.object(&balance_manager.address),
    //                 trade_proof,
    //             ],
    //         );
    //     }
    // }

    // /// Submit a governance proposal
    // /// 
    // /// # Arguments
    // /// * `params` - Parameters for the proposal
    // pub fn submit_proposal<F>(&self, params: ProposalParams) -> impl FnOnce(&mut Transaction) -> () {
    //     let pool = self.config.get_pool(&params.pool_key);
    //     let balance_manager = self.config.get_balance_manager(&params.balance_manager_key);
    //     let base_coin = self.config.get_coin(&pool.base_coin);
    //     let quote_coin = self.config.get_coin(&pool.quote_coin);

    //     let taker_fee = (params.taker_fee * FLOAT_SCALAR as f64).round() as u64;
    //     let maker_fee = (params.maker_fee * FLOAT_SCALAR as f64).round() as u64;
    //     let stake_required = (params.stake_required * DEEP_SCALAR as f64).round() as u64;

    //     move |tx: &mut Transaction| {
    //         let trade_proof = tx.add(self.config.balance_manager.generate_proof(&params.balance_manager_key));
            
    //         tx.move_call(
    //             format!("{}::pool::submit_proposal", self.config.deepbook_package_id()),
    //             vec![base_coin.type_name.clone(), quote_coin.type_name.clone()],
    //             vec![
    //                 tx.object(&pool.address),
    //                 tx.object(&balance_manager.address),
    //                 trade_proof,
    //                 tx.pure_u64(taker_fee),
    //                 tx.pure_u64(maker_fee),
    //                 tx.pure_u64(stake_required),
    //             ],
    //         );
    //     }
    // }

    // /// Vote on a proposal
    // /// 
    // /// # Arguments
    // /// * `pool_key` - The key to identify the pool
    // /// * `balance_manager_key` - The key to identify the BalanceManager
    // /// * `proposal_id` - The ID of the proposal to vote on
    // pub fn vote<F>(&self, pool_key: &str, balance_manager_key: &str, proposal_id: &str) -> impl FnOnce(&mut Transaction) -> () {
    //     let pool = self.config.get_pool(pool_key);
    //     let balance_manager = self.config.get_balance_manager(balance_manager_key);
    //     let base_coin = self.config.get_coin(&pool.base_coin);
    //     let quote_coin = self.config.get_coin(&pool.quote_coin);

    //     move |tx: &mut Transaction| {
    //         let trade_proof = tx.add(self.config.balance_manager.generate_proof(balance_manager_key));
            
    //         tx.move_call(
    //             format!("{}::pool::vote", self.config.deepbook_package_id),
    //             vec![base_coin.type_name.clone(), quote_coin.type_name.clone()],
    //             vec![
    //                 tx.object(&pool.address),
    //                 tx.object(&balance_manager.address),
    //                 trade_proof,
    //                 tx.pure_id(proposal_id),
    //             ],
    //         );
    //     }
    // }
}
