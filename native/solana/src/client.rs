use std::{ops::Deref, sync::RwLock};

use super::transaction::TransactionWrapper;
use rustler::{resource_impl, Resource, ResourceArc};
use solana_client::rpc_client::RpcClient;

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

impl Deref for RpcClientWrapper {
    type Target = RwLock<RpcClient>;
    fn deref(&self) -> &Self::Target {
        &self.client
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
) -> Result<String, String> {
    let client = _client.read().map_err(|e| e.to_string())?;
    let signature = client
        .send_and_confirm_transaction(&_transaction.transaction)
        .map_err(|e| e.to_string())?;
    Ok(signature.to_string())
}

#[rustler::nif]
fn get_minimum_balance_for_rent_exemption(
    _client: ResourceArc<RpcClientWrapper>,
    _size: usize,
) -> Result<u64, String> {
    let client = _client.read().map_err(|e| e.to_string())?;
    client
        .get_minimum_balance_for_rent_exemption(_size)
        .map_err(|e| e.to_string())
}
