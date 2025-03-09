use std::sync::RwLock;

use super::transaction::TransactionWrapper;
use rustler::{resource_impl, Resource, ResourceArc};
use solana_client::rpc_client::RpcClient;

#[allow(dead_code)]
pub struct RpcClientWrapper {
    pub client: RwLock<RpcClient>,
}

impl RpcClientWrapper {
    pub fn new(url: String) -> Self {
        Self {
            client: RwLock::new(RpcClient::new(url)),
        }
    }
}

#[resource_impl]
impl Resource for RpcClientWrapper {}

impl From<RpcClient> for RpcClientWrapper {
    fn from(value: RpcClient) -> Self {
        Self {
            client: RwLock::new(value),
        }
    }
}

#[rustler::nif]
fn new_rpc_client(url: String) -> ResourceArc<RpcClientWrapper> {
    ResourceArc::new(RpcClientWrapper::new(url))
}

#[rustler::nif]
fn send_and_confirm_transaction(
    _client: ResourceArc<RpcClientWrapper>,
    _transaction: ResourceArc<TransactionWrapper>,
) -> Result<u64, String> {
    todo!()
}

#[rustler::nif]
fn get_minimum_balance_for_rent_exemption(_size: usize) -> Result<u64, String> {
    todo!()
}

rustler::init!("Elixir.SolanaClient");
