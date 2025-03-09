use super::signature::KeypairWrapper;
use rustler::{resource_impl, Resource, ResourceArc};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

pub struct PubkeyWrapper {
    pub pubkey: Pubkey,
}

#[resource_impl]
impl Resource for PubkeyWrapper {}

impl From<Pubkey> for PubkeyWrapper {
    fn from(value: Pubkey) -> Self {
        Self { pubkey: value }
    }
}

#[rustler::nif]
fn keypair_to_public_key(_keypair: ResourceArc<KeypairWrapper>) -> ResourceArc<PubkeyWrapper> {
    ResourceArc::new(PubkeyWrapper::from(_keypair.keypair.pubkey()))
}
