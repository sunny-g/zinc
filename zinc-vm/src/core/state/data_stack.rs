use std::collections::BTreeMap;

use crate::core::Cell;
use crate::errors::MalformedBytecode;
use crate::gadgets;
use crate::gadgets::{Gadgets, Scalar};
use crate::RuntimeError;
use algebra::Field;
use r1cs_core::ConstraintSystem;
use std::fmt;

#[derive(Debug)]
struct CellDelta<F: Field> {
    old: Option<Cell<F>>,
    new: Cell<F>,
}

type DataStackDelta<F> = BTreeMap<usize, CellDelta<F>>;

#[derive(Debug)]
enum DataStackBranch<F: Field> {
    IfThen(DataStackDelta<F>),
    IfThenElse(DataStackDelta<F>, DataStackDelta<F>),
}

impl<F: Field> DataStackBranch<F> {
    fn new() -> Self {
        DataStackBranch::IfThen(DataStackDelta::new())
    }

    fn active_delta(&mut self) -> &mut DataStackDelta<F> {
        match self {
            DataStackBranch::IfThen(t) => t,
            DataStackBranch::IfThenElse(_, e) => e,
        }
    }

    fn switch(self) -> Option<Self> {
        match self {
            DataStackBranch::IfThen(t) => {
                Some(DataStackBranch::IfThenElse(t, DataStackDelta::new()))
            }
            DataStackBranch::IfThenElse(_, _) => None,
        }
    }
}

#[derive(Debug)]
pub struct DataStack<F: Field> {
    memory: Vec<Option<Cell<F>>>,
    branches: Vec<DataStackBranch<F>>,
}

impl<F: Field> DataStack<F> {
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            branches: Vec::new(),
        }
    }

    pub fn get(&mut self, address: usize) -> Result<Cell<F>, RuntimeError> {
        if let Some(cell) = self.memory.get(address) {
            cell.clone()
                .ok_or_else(|| MalformedBytecode::UninitializedStorageAccess.into())
        } else {
            Err(MalformedBytecode::UninitializedStorageAccess.into())
        }
    }

    pub fn set(&mut self, address: usize, value: Cell<F>) -> Result<(), RuntimeError> {
        if self.memory.len() <= address {
            let mut extra = vec![None; address + 1 - self.memory.len()];
            self.memory.append(&mut extra);
        }

        if let Some(branch) = self.branches.last_mut() {
            let delta = branch.active_delta();
            let old = match delta.get(&address) {
                Some(old_cd) => old_cd.old.clone(),
                None => self.memory[address].clone(),
            };
            delta.insert(
                address,
                CellDelta {
                    old,
                    new: value.clone(),
                },
            );
        }

        self.memory[address] = Some(value);

        Ok(())
    }

    /// Create a new memory state branch
    pub fn fork(&mut self) {
        self.branches.push(DataStackBranch::new());
    }

    /// Create an alternative branch (same parent as current one).
    pub fn switch_branch(&mut self) -> Result<(), RuntimeError> {
        let mut branch = self
            .branches
            .pop()
            .ok_or(MalformedBytecode::UnexpectedElse)?;
        self.revert(&branch.active_delta());
        let new_branch = branch.switch().ok_or(MalformedBytecode::UnexpectedElse)?;
        self.branches.push(new_branch);
        Ok(())
    }

    /// Merge top-level branch or branches into parent branch.
    pub fn merge<CS: ConstraintSystem<F>>(
        &mut self,
        condition: Scalar<F>,
        ops: &mut Gadgets<F, CS>,
    ) -> Result<(), RuntimeError> {
        let mut branch = self
            .branches
            .pop()
            .ok_or(MalformedBytecode::UnexpectedEndIf)?;
        self.revert(branch.active_delta());

        match branch {
            DataStackBranch::IfThen(delta) => self.merge_single(condition, &delta, ops)?,
            DataStackBranch::IfThenElse(t, f) => self.merge_pair(condition, &t, &f, ops)?,
        }

        Ok(())
    }

    fn revert(&mut self, delta: &DataStackDelta<F>) {
        for (address, d) in delta.iter() {
            self.memory[*address] = d.old.clone();
        }
    }

    /// Conditionally apply delta
    fn merge_single<CS: ConstraintSystem<F>>(
        &mut self,
        condition: Scalar<F>,
        delta: &DataStackDelta<F>,
        ops: &mut Gadgets<F, CS>,
    ) -> Result<(), RuntimeError> {
        for (&addr, diff) in delta.iter() {
            match (&self.memory[addr], &diff.new) {
                (None, _) => {}
                (Some(Cell::Value(old)), Cell::Value(new)) => {
                    let cs = ops
                        .constraint_system()
                        .ns(|| format!("merge address {}", addr));
                    let value = gadgets::conditional_select(cs, &condition, new, old)?;
                    self.set(addr, Cell::Value(value))?;
                }
            }
        }

        Ok(())
    }

    /// Conditionally apply one of two deltas.
    fn merge_pair<CS>(
        &mut self,
        condition: Scalar<F>,
        delta_then: &DataStackDelta<F>,
        delta_else: &DataStackDelta<F>,
        ops: &mut Gadgets<F, CS>,
    ) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<F>,
    {
        for (addr, diff) in delta_then.iter() {
            let alt = if let Some(diff) = delta_else.get(addr) {
                Some(diff.new.clone())
            } else {
                self.memory[*addr].clone()
            };

            match (&alt, &diff.new) {
                (None, _) => {}
                (Some(Cell::Value(old)), Cell::Value(new)) => {
                    let cs = ops
                        .constraint_system()
                        .ns(|| format!("merge address {}", addr));
                    let value = gadgets::conditional_select(cs, &condition, new, old)?;
                    self.set(*addr, Cell::Value(value))?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::{BigInt, ToBigInt};
    use pairing::bn256::Bn256;

    use crate::gadgets::{Gadgets, ScalarType};

    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;

    fn assert_cell_eq<F: Field>(cell: Cell<F>, value: BigInt) {
        let Cell::Value(v) = cell;
        assert_eq!(v.to_bigint().unwrap(), value);
    }

    #[test]
    fn test_get_set() {
        let mut ds = DataStack::new();
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let op = Gadgets::new(&mut cs);
        let value = op.constant_bigint(&42.into(), ScalarType::Field).unwrap();
        ds.set(4, Cell::Value(value)).unwrap();

        assert_cell_eq(ds.get(4).unwrap(), 42.into());
    }

    #[test]
    fn test_fork_merge_true() {
        let mut ds = DataStack::new();
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let mut ops = Gadgets::new(&mut cs);
        let value = ops.constant_bigint(&42.into(), ScalarType::Field).unwrap();
        ds.set(4, Cell::Value(value)).unwrap();

        ds.fork();

        assert_cell_eq(ds.get(4).unwrap(), 42.into());

        let value2 = ops.constant_bigint(&13.into(), ScalarType::Field).unwrap();
        ds.set(4, Cell::Value(value2)).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 13.into());

        let condition = Scalar::new_constant_bool(true);
        ds.merge(condition, &mut ops).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 13.into());
    }

    #[test]
    fn test_fork_merge_false() {
        let mut ds = DataStack::new();
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let mut ops = Gadgets::new(&mut cs);
        let value = ops.constant_bigint(&42.into(), ScalarType::Field).unwrap();
        ds.set(4, Cell::Value(value)).unwrap();

        ds.fork();

        assert_cell_eq(ds.get(4).unwrap(), 42.into());

        let value2 = ops.constant_bigint(&13.into(), ScalarType::Field).unwrap();
        ds.set(4, Cell::Value(value2)).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 13.into());

        let condition = Scalar::new_constant_bool(false);
        ds.merge(condition, &mut ops).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 42.into());
    }
}

impl<F: Field> fmt::Display for DataStack<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Data Stack:")?;

        for (address, opt_cell) in self.memory.iter().enumerate() {
            match opt_cell {
                None => writeln!(f, "\t{:4}: <empty>", address)?,
                Some(Cell::Value(value)) => writeln!(f, "\t{:4}: {}", address, value)?,
            }
        }

        Ok(())
    }
}
