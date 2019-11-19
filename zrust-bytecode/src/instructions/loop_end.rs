use crate::{Instruction, InstructionCode, DecodingError};
use crate::instructions::utils::decode_simple_instruction;

#[derive(Debug,PartialEq)]
pub struct LoopEnd;

impl Instruction for LoopEnd {
    fn to_assembly(&self) -> String {
        "loop_end".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::LoopEnd
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::LoopEnd as u8]
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }
}

impl LoopEnd {
    pub fn decode(bytes: &[u8]) -> Result<(LoopEnd, usize), DecodingError> {
        decode_simple_instruction(bytes, InstructionCode::LoopEnd, LoopEnd)
    }
}
