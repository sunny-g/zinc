extern crate franklin_crypto;

use crate::gadgets::stdlib::arrays::{ArrayPad, Reverse, Truncate};
use crate::gadgets::stdlib::bits::*;
use crate::gadgets::stdlib::crypto::{Pedersen, Sha256, VerifyEddsaSignature};

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::instructions::CallBuiltin;

impl<E, CS> VMInstruction<E, CS> for CallBuiltin
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        // TODO: Use call_native for all built-in functions
        if let BuiltinIdentifier::CryptoSchnorrVerify = self.identifier {
            return vm.call_native(VerifyEddsaSignature);
        }

        let mut input = Vec::new();
        for _ in 0..self.inputs_count {
            let value = vm.pop()?.value()?;
            input.push(value);
        }

        input.reverse();

        let output = match self.identifier {
            BuiltinIdentifier::CryptoSha256 => vm.operations().execute(Sha256, &input),
            BuiltinIdentifier::CryptoPedersen => vm.operations().execute(Pedersen, &input),
            BuiltinIdentifier::ToBits => vm.operations().execute(ToBits, &input),
            BuiltinIdentifier::UnsignedFromBits => {
                vm.operations().execute(UnsignedFromBits, &input)
            }
            BuiltinIdentifier::SignedFromBits => vm.operations().execute(SignedFromBits, &input),
            BuiltinIdentifier::FieldFromBits => vm.operations().execute(FieldFromBits, &input),
            BuiltinIdentifier::ArrayPad => vm.operations().execute(ArrayPad, &input),
            BuiltinIdentifier::ArrayTruncate => vm.operations().execute(Truncate, &input),
            BuiltinIdentifier::ArrayReverse => vm.operations().execute(Reverse, &input),
            f => unimplemented!("Builtin function {} is not implemented.", f),
        }?;

        for value in output.into_iter() {
            vm.push(Cell::Value(value))?
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
