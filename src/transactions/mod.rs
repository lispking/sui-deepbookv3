// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use sui_json_rpc_types::SuiObjectData;
use sui_json_rpc_types::SuiObjectDataOptions;
use sui_sdk::{types::base_types::ObjectID, SuiClient};

pub mod balance_manager;
pub mod governance;

#[async_trait]
pub trait DataReader {
    async fn get_object(&self, object_id: ObjectID) -> anyhow::Result<SuiObjectData>;
}

#[async_trait]
impl DataReader for SuiClient {
    async fn get_object(&self, object_id: ObjectID) -> anyhow::Result<SuiObjectData> {
        self.read_api()
            .get_object_with_options(object_id, SuiObjectDataOptions::full_content())
            .await?
            .data
            .ok_or(anyhow::anyhow!("Object {} not found", object_id))
    }
}
