use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct Xor;

impl Instruction for Xor {
    fn to_assembly(&self) -> String {
        "xor".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Xor
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Xor as u8]
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Xor {
    pub fn decode(bytes: &[u8]) -> Result<(Xor, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::Xor, Xor)
    }
}
