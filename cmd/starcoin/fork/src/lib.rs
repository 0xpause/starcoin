// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, bail, format_err, Result};
use futures::executor::block_on;
use futures_timer::Delay;
use starcoin_config::{BaseConfig, NodeConfig, StarcoinOpt};
use starcoin_logger::prelude::*;
use starcoin_node_api::errors::NodeStartError;
use starcoin_rpc_server::service::RpcService;
use starcoin_service_registry::{RegistryAsyncService, RegistryService,  ServiceRef};
// use starcoin_storage::Storage;
// use starcoin_sync::sync::SyncService;
// use starcoin_txpool::TxPoolService;
use std::sync::Arc;
use tokio::runtime::Runtime;
use fork_node::ForkNodeService;
use starcoin_rpc_client::RpcClient;
use jsonrpc_client_transports::RpcChannel;

pub mod fork_node;
pub mod rpc_service_factory;
pub mod fork_chain;
pub mod remote_state;

pub struct ForkNodeHandle {
  runtime: Runtime,
  join_handle: timeout_join_handler::TimeoutJoinHandle<Result<()>>,
  node_service: ServiceRef<ForkNodeService>,
  registry: ServiceRef<RegistryService>,
}

impl ForkNodeHandle {
  pub fn new(
      join_handle: timeout_join_handler::TimeoutJoinHandle<Result<()>>,
      node_service: ServiceRef<ForkNodeService>,
      registry: ServiceRef<RegistryService>,
  ) -> Self {
      Self {
          runtime: Runtime::new().unwrap(),
          join_handle,
          node_service,
          registry,
      }
  }

  pub fn rpc_service(&self) -> Result<ServiceRef<RpcService>> {
      block_on(async { self.registry.service_ref::<RpcService>().await })
  }
}

pub fn run_node_fork_remote(
  opt: &StarcoinOpt,
) -> Result<(Option<ForkNodeHandle>, Arc<NodeConfig>), NodeStartError> {
  let rpc = opt.fork.clone().unwrap();
  let fork_number = opt.fork_number;
  //check genesis config is ready
  let mut base_config =
      BaseConfig::load_with_opt(opt).map_err(NodeStartError::LoadConfigError)?;
  // if !base_config.net().is_ready() {
  //     let future_block_resolve =
  //         RpcFutureBlockParameterResolver::new(base_config.net().id().clone());
  //     base_config
  //         .resolve(&future_block_resolve)
  //         .map_err(NodeStartError::LoadConfigError)?;
  // }
  let config = Arc::new(
      base_config
          .into_node_config(opt)
          .map_err(NodeStartError::LoadConfigError)?,
  );
  let ipc_file = config.rpc.get_ipc_file();
  let node_handle = if !ipc_file.exists() {
      let node_handle = run_fork_node(rpc, fork_number, config.clone())?;
      Some(node_handle)
  } else {
      //TODO check ipc file is available.
      info!("Node has started at {:?}", ipc_file);
      None
  };
  Ok((node_handle, config))
}

/// Run node in a new Thread, and return a NodeHandle.
pub fn run_fork_node(rpc: String, fork_number: Option<u64>, config: Arc<NodeConfig>) -> Result<ForkNodeHandle, NodeStartError> {
  // crash_handler::setup_panic_handler();
  let logger_handle = starcoin_logger::init();
  ForkNodeService::launch(rpc, fork_number, config, logger_handle)
}