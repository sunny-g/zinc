use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default)]
pub struct And;

impl InstructionInfo for And {
    fn to_assembly(&self) -> String {
        "and".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::And
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::And as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        2
    }

    fn outputs_count(&self) -> usize {
        1
    }
}
