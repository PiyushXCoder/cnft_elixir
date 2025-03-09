use rustler::{resource_impl, Resource};
use solana_sdk::instruction::Instruction;

pub struct InstructionWrapper {
    pub instruction: Instruction,
}

impl From<Instruction> for InstructionWrapper {
    fn from(value: Instruction) -> Self {
        Self { instruction: value }
    }
}

#[resource_impl]
impl Resource for InstructionWrapper {}
