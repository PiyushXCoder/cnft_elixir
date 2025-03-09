use rustler::{resource_impl, Resource};
use solana_sdk::signature::Keypair;

pub struct KeypairWrapper {
    pub keypair: Keypair,
}

#[resource_impl]
impl Resource for KeypairWrapper {}

impl From<Keypair> for KeypairWrapper {
    fn from(value: Keypair) -> Self {
        Self { keypair: value }
    }
}
