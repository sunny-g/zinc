use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets;
use crate::gadgets::{ScalarType, ScalarTypeExpectation};
use algebra::Field;
use r1cs_core::ConstraintSystem;
use zinc_bytecode::instructions::Add;

impl<F, CS> VMInstruction<F, CS> for Add
where
    F: Field,
    CS: ConstraintSystem<F>,
{
    fn execute(&self, vm: &mut VirtualMachine<F, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let sum_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_sum = gadgets::add(cs.ns(|| "sum"), &left, &right)?;

        let sum = gadgets::types::conditional_type_check(
            cs.ns(|| "type check"),
            &condition,
            &unchecked_sum,
            sum_type,
        )?;

        vm.push(Cell::Value(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_add() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(1.into()))
            .add(PushConst::new_field(2.into()))
            .add(Add)
            .test(&[3])
    }
}
