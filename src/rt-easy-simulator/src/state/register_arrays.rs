use crate::Error;
use rtcore::{
    program::{Declaration, Ident, Program},
    value::Value,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct RegisterArraysState {
    register_arrays: HashMap<Ident, RegisterArrayState>,
}

impl RegisterArraysState {
    pub fn init(program: &Program) -> Self {
        let mut register_arrays = HashMap::new();

        for declaration in program.declarations() {
            if let Declaration::RegisterArray(declare_register_array) = declaration {
                for reg_array in &declare_register_array.register_arrays {
                    let index_size = log_2(reg_array.len);
                    let data_size = reg_array.range.map(|range| range.size()).unwrap_or(1);

                    register_arrays.insert(
                        reg_array.ident.clone(),
                        RegisterArrayState::init(index_size, data_size),
                    );
                }
            }
        }

        Self { register_arrays }
    }

    pub fn read(&self, name: &Ident, idx: Value) -> Result<Value, Error> {
        match self.register_arrays.get(name) {
            Some(state) => state.read(idx),
            None => Err(Error::Other),
        }
    }

    pub fn write(&mut self, name: &Ident, idx: Value, value: Value) -> Result<(), Error> {
        match self.register_arrays.get_mut(name) {
            Some(state) => state.write(idx, value),
            None => Err(Error::Other),
        }
    }

    /*pub fn range_of(&self, name: &Ident) -> Result<BitRange, Error> {
        match self.register_arrays.get(name) {
            Some(state) => Ok(state.range()),
            None => Err(Error::Other),
        }
    }*/
}

impl fmt::Display for RegisterArraysState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut register_arrays = self.register_arrays.keys().collect::<Vec<_>>();
        register_arrays.sort();

        for (idx, reg_array) in register_arrays.into_iter().enumerate() {
            write!(
                f,
                "{}{} = {}",
                if idx != 0 { "\n" } else { "" },
                reg_array.0,
                self.register_arrays.get(reg_array).unwrap()
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct RegisterArrayState {
    data: HashMap<Value, Value>,
    index_size: usize,
    data_size: usize,
}

impl RegisterArrayState {
    fn init(index_size: usize, data_size: usize) -> Self {
        Self { data: HashMap::new(), index_size, data_size }
    }

    fn read(&self, idx: Value) -> Result<Value, Error> {
        debug_assert_eq!(idx.size(), self.index_size);

        let value = self.data.get(&idx).cloned().unwrap_or_else(|| Value::zero(self.data_size));
        Ok(value)
    }

    fn write(&mut self, idx: Value, value: Value) -> Result<(), Error> {
        debug_assert_eq!(idx.size(), self.index_size);
        debug_assert_eq!(value.size(), self.data_size);

        self.data.insert(idx, value);
        Ok(())
    }
}

impl fmt::Display for RegisterArrayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indexes = self.data.keys().collect::<Vec<_>>();
        indexes.sort();

        write!(f, "[\n")?;
        for idx in indexes {
            write!(f, "  {} = {}\n", idx.as_dec(), self.data.get(idx).unwrap().as_dec())?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

fn log_2(x: usize) -> usize {
    const fn num_bits<T>() -> usize {
        std::mem::size_of::<T>() * 8
    }

    if x == 0 {
        0
    } else {
        num_bits::<usize>() - x.leading_zeros() as usize - 1
    }
}
