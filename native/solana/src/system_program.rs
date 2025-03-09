use rustler::ResourceArc;
use solana_sdk::system_instruction;

use crate::{instruction::InstructionWrapper, pubkey::PubkeyWrapper};

#[rustler::nif]
fn create_account_instruction(
    from_pubkey: ResourceArc<PubkeyWrapper>,
    to_pubkey: ResourceArc<PubkeyWrapper>,
    lamports: u64,
    space: u64,
    owner: ResourceArc<PubkeyWrapper>,
) -> ResourceArc<InstructionWrapper> {
    let instruction = system_instruction::create_account(
        &from_pubkey.pubkey,
        &to_pubkey.pubkey,
        lamports,
        space,
        &owner.pubkey,
    );
    ResourceArc::new(InstructionWrapper::from(instruction))
}
