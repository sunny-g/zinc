use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets;
use algebra::Field;
use r1cs_core::ConstraintSystem;
use zinc_bytecode::instructions::Ne;

impl<F, CS> VMInstruction<F, CS> for Ne
where
    F: Field,
    CS: ConstraintSystem<F>,
{
    fn execute(&self, vm: &mut VirtualMachine<F, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let ne = gadgets::ne(cs.ns(|| "ne"), &left, &right)?;

        vm.push(Cell::Value(ne))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_ne() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(1.into()))
            .add(PushConst::new_field(2.into()))
            .add(Ne)
            .add(PushConst::new_field(2.into()))
            .add(PushConst::new_field(2.into()))
            .add(Ne)
            .add(PushConst::new_field(2.into()))
            .add(PushConst::new_field(1.into()))
            .add(Ne)
            .test(&[1, 0, 1])
    }
}
