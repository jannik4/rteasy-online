use super::{
    memories::MemoriesState, register_arrays::RegisterArraysState, registers::RegistersState,
};
use crate::Error;
use rtcore::{
    program::{Ident, Label, Register},
    value::Value,
};

#[derive(Debug)]
pub struct ChangeSet {
    goto: Option<Label>,
    write_register: Vec<(Register, Value)>,
    write_register_array: Vec<(Ident, Value, Value)>,
    write_memory: Vec<Ident>,
}

impl ChangeSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn goto(&mut self, label: Label) -> Result<(), Error> {
        match self.goto {
            Some(_) => Err(Error::Other),
            None => {
                self.goto = Some(label);
                Ok(())
            }
        }
    }

    pub fn write_into_register(&mut self, register: Register, value: Value) -> Result<(), Error> {
        self.write_register.push((register, value));
        Ok(())
    }

    pub fn write_into_register_array(
        &mut self,
        name: Ident,
        idx: Value,
        value: Value,
    ) -> Result<(), Error> {
        self.write_register_array.push((name, idx, value));
        Ok(())
    }

    pub fn write_memory(&mut self, name: Ident) -> Result<(), Error> {
        self.write_memory.push(name);
        Ok(())
    }

    pub(super) fn apply(
        self,
        registers: &mut RegistersState,
        memories: &mut MemoriesState,
        register_arrays: &mut RegisterArraysState,
    ) -> Result<Option<Label>, Error> {
        for mem in self.write_memory {
            memories.write(&mem, registers)?;
        }
        for (reg, val) in self.write_register {
            registers.write(&reg.ident, reg.range, val)?;
        }
        for (name, idx, val) in self.write_register_array {
            register_arrays.write(&name, idx, val)?;
        }

        Ok(self.goto)
    }
}

impl Default for ChangeSet {
    fn default() -> Self {
        Self {
            goto: None,
            write_register: Vec::new(),
            write_register_array: Vec::new(),
            write_memory: Vec::new(),
        }
    }
}
