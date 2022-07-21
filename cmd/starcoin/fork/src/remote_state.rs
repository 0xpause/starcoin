use anyhow::{anyhow, Result};
use jsonrpc_client_transports::RpcChannel;
use starcoin_crypto::HashValue;

use starcoin_rpc_api::state::StateApiClient;
use starcoin_rpc_api::chain::ChainApiClient;
use starcoin_rpc_api::types::BlockView;
use futures::executor::block_on;

#[derive(Clone)]
pub struct RemoteStateView {
    state_client: StateApiClient,
    chain_client: ChainApiClient,
    state_root: HashValue,
}

impl RemoteStateView {
    pub fn from_url(rpc_url: &str, block_number: Option<u64>) -> Result<Self> {
        let (state_client, chain_client, state_root) =
            block_on(async { Self::from_url_inner(rpc_url, block_number).await })?;

        Ok(Self {
            state_client,
            chain_client,
            state_root,
        })
    }

    pub fn state_client(&self) -> &StateApiClient {
        &self.state_client
    }

    pub fn chain_client(&self) -> &ChainApiClient {
        &self.chain_client
    }

    async fn from_url_inner(
        rpc_url: &str, 
        block_number: Option<u64>
    ) -> Result<(StateApiClient, ChainApiClient, HashValue)> {
        let rpc_channel: RpcChannel = jsonrpc_client_transports::transports::http::connect(rpc_url)
            .await
            .map_err(|e| anyhow!(format!("{}", e)))?;
        let chain_client: starcoin_rpc_api::chain::ChainApiClient = rpc_channel.clone().into();
        let state_root = match block_number {
            None => {
                let chain_info = chain_client
                    .info()
                    .await
                    .map_err(|e| anyhow!(format!("{}", e)))?;
                chain_info.head.state_root
            }
            Some(n) => {
                let b: Option<BlockView> = chain_client
                    .get_block_by_number(n, None)
                    .await
                    .map_err(|e| anyhow!(format!("{}", e)))?;
                let b = b.ok_or_else(|| anyhow::anyhow!("cannot found block of height {}", n))?;
                b.header.state_root
            }
        };
        let state_client: starcoin_rpc_api::state::StateApiClient = rpc_channel.clone().into();
        Ok((state_client, chain_client, state_root))
    }

}
