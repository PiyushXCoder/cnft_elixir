use rustler::{resource_impl, Resource, ResourceArc};
use solana_sdk::transaction::Transaction;

use crate::{
    hash::HashWrapper, instruction::InstructionWrapper, pubkey::PubKeyWrapper,
    signature::KeypairWrapper,
};

pub struct TransactionWrapper {
    pub transaction: Transaction,
}

#[resource_impl]
impl Resource for TransactionWrapper {}

impl From<Transaction> for TransactionWrapper {
    fn from(value: Transaction) -> Self {
        Self { transaction: value }
    }
}

#[rustler::nif]
fn new_signed_transaction_with_payer(
    _instructions: Vec<ResourceArc<InstructionWrapper>>,
    _payer: Option<ResourceArc<PubKeyWrapper>>,
    _signing_keypair: Vec<ResourceArc<KeypairWrapper>>,
    _latest_blockhash: ResourceArc<HashWrapper>,
) -> Result<String, String> {
    todo!()
}
