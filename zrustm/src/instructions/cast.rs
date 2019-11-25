extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Cast;

impl<E, O> VMInstruction<E, O> for Cast
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cast() -> Result<(), RuntimeError> {
        Ok(())
    }
}
