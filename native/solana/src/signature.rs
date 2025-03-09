use rustler::{resource_impl, Resource, ResourceArc};
use solana_sdk::{signature::Keypair, signer::EncodableKey};

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

#[rustler::nif]
fn new_keypair() -> ResourceArc<KeypairWrapper> {
    ResourceArc::new(KeypairWrapper::from(Keypair::new()))
}

#[rustler::nif]
fn keypair_read_from_file(file: String) -> Result<ResourceArc<KeypairWrapper>, String> {
    Ok(ResourceArc::new(KeypairWrapper::from(
        Keypair::read_from_file(file).map_err(|e| e.to_string())?,
    )))
}
