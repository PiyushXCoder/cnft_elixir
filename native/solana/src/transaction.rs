use rustler::{resource_impl, Resource, ResourceArc};
use solana_sdk::{instruction::Instruction, transaction::Transaction};

use crate::{
    hash::HashWrapper, instruction::InstructionWrapper, pubkey::PubkeyWrapper,
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
    _payer: Option<ResourceArc<PubkeyWrapper>>,
    _signing_keypair: Vec<ResourceArc<KeypairWrapper>>,
    _latest_blockhash: ResourceArc<HashWrapper>,
) -> ResourceArc<TransactionWrapper> {
    let instructions: &[Instruction] = &_instructions
        .into_iter()
        .map(|instruction| instruction.instruction.clone())
        .collect::<Vec<Instruction>>();

    let mut signing_keypairs = Vec::new();
    for keypair in _signing_keypair.iter().map(|a| &a.keypair) {
        signing_keypairs.push(keypair);
    }

    let payer = _payer.map(|payer| payer.pubkey);
    let latest_blockhash = _latest_blockhash.hash;
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        payer.as_ref(),
        &signing_keypairs[..],
        latest_blockhash,
    );
    ResourceArc::new(TransactionWrapper::from(transaction))
}
