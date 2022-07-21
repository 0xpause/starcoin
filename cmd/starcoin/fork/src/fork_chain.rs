// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::remote_state::RemoteStateView;
use anyhow::anyhow;
use futures::{FutureExt, TryFutureExt};
use starcoin_config::NodeConfig;
use starcoin_crypto::HashValue;
use starcoin_rpc_api::chain::{ChainApi, GetBlockOption, GetEventOption, GetTransactionOption};
use starcoin_rpc_api::types::pubsub::EventFilter;
use starcoin_rpc_api::types::{
    BlockHeaderView, BlockInfoView,  BlockView, ChainId, ChainInfoView,
    StrView, TransactionEventResponse, TransactionInfoView,
    TransactionInfoWithProofView, TransactionView,
};
use starcoin_rpc_api::FutureResult;
use starcoin_rpc_server::module::map_err;
use starcoin_storage::Storage;
use starcoin_types::access_path::AccessPath;
use starcoin_types::block::BlockNumber;
use std::sync::Arc;
use jsonrpc_core;

pub struct ForkChain
{
    config: Arc<NodeConfig>,
    storage: Arc<Storage>,
    client: Arc<RemoteStateView>,
    fork_block_number: u64,
    current_block_number: u64,
}

impl ForkChain
{
    pub fn new(
        config: Arc<NodeConfig>,
        storage: Arc<Storage>,
        client: Arc<RemoteStateView>,
    ) -> Self {
        Self {
            config,
            storage,
            client,
        }
    }
}

impl ChainApi for ForkChain
{
    fn id(&self) -> jsonrpc_core::Result<ChainId> {
        Ok(self.config.net().id().into())
    }

    fn info(&self) -> FutureResult<ChainInfoView> {
        let fut = if self.current_block_number > self.fork_block_number {
            // Get chain info from Storage
            async move { ChainInfoView::new() }
        } else {
            // Get chain info from remote
            let client = self.client.clone();
            async move {
                client.chain_client()
                    .info()
                    .await
                    .map_err(|e| anyhow!(format!("{}", e)))
            }
        };
        Box::pin(fut.boxed().map_err(map_err))
    }

    fn get_block_by_hash(
        &self,
        hash: HashValue,
        option: Option<GetBlockOption>,
    ) -> FutureResult<Option<BlockView>> {
        unimplemented!();
    }

    fn get_block_by_number(
        &self,
        number: u64,
        option: Option<GetBlockOption>,
    ) -> FutureResult<Option<BlockView>> {
        unimplemented!();
    }

    fn get_blocks_by_number(
        &self,
        number: Option<BlockNumber>,
        count: u64,
    ) -> FutureResult<Vec<BlockView>> {
        unimplemented!();
    }

    fn get_block_info_by_number(&self, number: u64) -> FutureResult<Option<BlockInfoView>> {
        unimplemented!();
    }

    fn get_transaction(
        &self,
        transaction_hash: HashValue,
        option: Option<GetTransactionOption>,
    ) -> FutureResult<Option<TransactionView>> {
        unimplemented!();
    }

    fn get_transaction_info(
        &self,
        transaction_hash: HashValue,
    ) -> FutureResult<Option<TransactionInfoView>> {
        unimplemented!();
    }

    fn get_block_txn_infos(&self, block_hash: HashValue) -> FutureResult<Vec<TransactionInfoView>> {
        unimplemented!();
    }

    fn get_txn_info_by_block_and_index(
        &self,
        block_hash: HashValue,
        idx: u64,
    ) -> FutureResult<Option<TransactionInfoView>> {
        unimplemented!();
    }

    fn get_events_by_txn_hash(
        &self,
        txn_hash: HashValue,
        option: Option<GetEventOption>,
    ) -> FutureResult<Vec<TransactionEventResponse>> {
        unimplemented!();
    }

    fn get_events(
        &self,
        mut filter: EventFilter,
        option: Option<GetEventOption>,
    ) -> FutureResult<Vec<TransactionEventResponse>> {
        unimplemented!();
    }

    fn get_headers(&self, block_hashes: Vec<HashValue>) -> FutureResult<Vec<BlockHeaderView>> {
        unimplemented!();
    }

    fn get_transaction_infos(
        &self,
        start_global_index: u64,
        reverse: bool,
        max_size: u64,
    ) -> FutureResult<Vec<TransactionInfoView>> {
        unimplemented!();
    }

    fn get_transaction_proof(
        &self,
        block_hash: HashValue,
        transaction_global_index: u64,
        event_index: Option<u64>,
        access_path: Option<StrView<AccessPath>>,
    ) -> FutureResult<Option<TransactionInfoWithProofView>> {
        unimplemented!();
    }

    fn get_transaction_proof_raw(
        &self,
        block_hash: HashValue,
        transaction_global_index: u64,
        event_index: Option<u64>,
        access_path: Option<StrView<AccessPath>>,
    ) -> FutureResult<Option<StrView<Vec<u8>>>> {
        unimplemented!();
    }
}

