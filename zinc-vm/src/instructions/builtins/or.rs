use crate::core::{Cell, InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use algebra::Field;
use r1cs_core::ConstraintSystem;
use zinc_bytecode::instructions::Or;

impl<F, CS> VMInstruction<F, CS> for Or
where
    F: Field,
    CS: ConstraintSystem<F>,
{
    fn execute(&self, vm: &mut VirtualMachine<F, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let or = vm.operations().or(left, right)?;

        vm.push(Cell::Value(or))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::ScalarType;
    use zinc_bytecode::*;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(Or)
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(Or)
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(Or)
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(Or)
            .test(&[1, 1, 1, 0])
    }
}
