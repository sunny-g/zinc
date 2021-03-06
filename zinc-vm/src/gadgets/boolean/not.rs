use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{Expression, Scalar, ScalarType, ScalarTypeExpectation};
use crate::{auto_const, Result};
use algebra::Field;
use r1cs_core::ConstraintSystem;

pub fn not<F, CS>(cs: CS, scalar: &Scalar<F>) -> Result<Scalar<F>>
where
    F: Field,
    CS: ConstraintSystem<F>,
{
    fn inner<F, CS>(mut cs: CS, scalar: &Scalar<F>) -> Result<Scalar<F>>
    where
        F: Field,
        CS: ConstraintSystem<F>,
    {
        scalar.get_type().assert_type(ScalarType::Boolean)?;

        let one_expr = Expression::u64::<CS>(1);
        let not_expr = one_expr - scalar.to_expression::<CS>();

        let not_num = not_expr.into_number(cs.ns(|| "not_num"))?;

        Ok(Scalar::new_unchecked_variable(
            not_num.get_value(),
            not_num.get_variable(),
            ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, scalar)
}
