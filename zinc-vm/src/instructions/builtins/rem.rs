use crate::auto_const;
use crate::core::{Cell, InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use crate::gadgets::{self, ScalarType, ScalarTypeExpectation};
use algebra::Field;
use r1cs_core::ConstraintSystem;
use zinc_bytecode::instructions::Rem;

impl<F, CS> VMInstruction<F, CS> for Rem
where
    F: Field,
    CS: ConstraintSystem<F>,
{
    fn execute(&self, vm: &mut VirtualMachine<F, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let (_div, unchecked_rem) =
            gadgets::div_rem_conditional(cs.ns(|| "div_rem"), &condition, &left, &right)?;

        let rem = gadgets::conditional_type_check(
            cs.ns(|| "type check"),
            &condition,
            &unchecked_rem,
            ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;
    use zinc_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new(9.into(), IntegerType::I8.into()))
            .add(PushConst::new(4.into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new(9.into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new(4.into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Rem)
            .test(&[3, 3, 1, 1])
    }
}
