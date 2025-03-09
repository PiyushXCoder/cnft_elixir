use rustler::{resource_impl, Resource};
use solana_program::hash::Hash;

pub struct HashWrapper {
    pub hash: Hash,
}

impl From<Hash> for HashWrapper {
    fn from(value: Hash) -> Self {
        Self { hash: value }
    }
}

#[resource_impl]
impl Resource for HashWrapper {}
