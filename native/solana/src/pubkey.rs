use super::signature::KeypairWrapper;
use rustler::{resource_impl, Resource, ResourceArc};
use solana_sdk::pubkey::Pubkey;

pub struct PubKeyWrapper {
    pub keypair: Pubkey,
}

#[resource_impl]
impl Resource for PubKeyWrapper {}

impl From<Pubkey> for PubKeyWrapper {
    fn from(value: Pubkey) -> Self {
        Self { keypair: value }
    }
}

#[rustler::nif]
fn keypair_to_public_key(_keypair: ResourceArc<KeypairWrapper>) -> ResourceArc<PubKeyWrapper> {
    todo!()
}
