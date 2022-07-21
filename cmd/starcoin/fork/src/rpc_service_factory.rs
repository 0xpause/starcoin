// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::fork_chain::ForkChain;
use crate::remote_state::RemoteStateView;
use anyhow::Result;
use starcoin_config::NodeConfig;
use starcoin_rpc_server::service::RpcService;
use starcoin_service_registry::{ServiceContext, ServiceFactory};
use starcoin_storage::Storage;
use std::sync::Arc;

pub struct ForkNodeRpcServiceFactory;

// implement rpc service factory at node for avoid cycle dependency.
impl ServiceFactory<RpcService> for ForkNodeRpcServiceFactory {
    fn create(ctx: &mut ServiceContext<RpcService>) -> Result<RpcService> {
        let config = ctx.get_shared::<Arc<NodeConfig>>()?;
        let storage = ctx.get_shared::<Arc<Storage>>()?;
        let remote_client = ctx.get_shared::<Arc<RemoteStateView>>()?;

        let chain_api = ForkChain::new(config.clone(), storage, remote_client);

        Ok(RpcService::new_with_fork_node_api(
            config,          
            Some(chain_api),
        ))
    }
}
