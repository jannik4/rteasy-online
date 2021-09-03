use super::reg_bus::RegBusState;
use crate::Error;
use rtcore::{
    program::{BitRange, Declaration, Ident, Program},
    value::Value,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct RegistersState {
    registers: HashMap<Ident, RegBusState>,
}

impl RegistersState {
    pub fn init(program: &Program) -> Self {
        let mut registers = HashMap::new();

        for declaration in program.declarations() {
            if let Declaration::Register(declare_register) = declaration {
                for reg in &declare_register.registers {
                    registers.insert(reg.ident.clone(), RegBusState::init(reg.range));
                }
            }
        }

        Self { registers }
    }

    pub fn names(&self) -> impl Iterator<Item = &Ident> {
        self.registers.keys()
    }

    pub fn read_full(&self, name: &Ident) -> Result<Value, Error> {
        match self.registers.get(name) {
            Some(state) => Ok(state.read_full()),
            None => Err(Error::Other),
        }
    }

    pub fn read(&self, name: &Ident, range: Option<BitRange>) -> Result<Value, Error> {
        match self.registers.get(name) {
            Some(state) => state.read(range),
            None => Err(Error::Other),
        }
    }

    pub fn write(
        &mut self,
        name: &Ident,
        range: Option<BitRange>,
        value: Value,
    ) -> Result<(), Error> {
        match self.registers.get_mut(name) {
            Some(state) => state.write(range, value),
            None => Err(Error::Other),
        }
    }

    pub fn range_of(&self, name: &Ident) -> Result<BitRange, Error> {
        match self.registers.get(name) {
            Some(state) => Ok(state.range()),
            None => Err(Error::Other),
        }
    }
}

impl fmt::Display for RegistersState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut registers = self.registers.keys().collect::<Vec<_>>();
        registers.sort();

        for (idx, reg) in registers.into_iter().enumerate() {
            write!(
                f,
                "{}{} = {}",
                if idx != 0 { "\n" } else { "" },
                reg.0,
                self.read_full(reg).unwrap().as_dec()
            )?;
        }

        Ok(())
    }
}
