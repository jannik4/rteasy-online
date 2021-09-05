mod buses;
mod memories;
mod reg_bus;
mod register_arrays;
mod registers;

use self::{
    buses::BusesState, memories::MemoriesState, register_arrays::RegisterArraysState,
    registers::RegistersState,
};
use crate::Error;
use rtcore::{
    program::{Bus, Ident, Label, Program, Register},
    value::Value,
};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct State {
    registers: RegistersState,
    buses: BusesState,
    memories: MemoriesState,
    register_arrays: RegisterArraysState,
}

impl State {
    pub fn init(program: &Program) -> Self {
        let registers = RegistersState::init(program);
        let buses = BusesState::init(program);
        let memories = MemoriesState::init(program, &registers);
        let register_arrays = RegisterArraysState::init(program);
        Self { registers, buses, memories, register_arrays }
    }

    pub fn clear_buses(&mut self, buses_persist: &HashSet<Ident>) {
        self.buses.clear(buses_persist);
    }

    pub fn write_into_bus(&mut self, bus: &Bus, value: Value) -> Result<(), Error> {
        self.buses.write(bus, value)
    }

    pub fn apply_change_set(&mut self, change_set: ChangeSet) -> Result<Option<Label>, Error> {
        for mem in change_set.write_memory {
            self.memories.write(&mem, &self.registers)?;
        }
        for (reg, val) in change_set.write_register {
            self.registers.write(&reg.ident, reg.range, val)?;
        }
        for (name, idx, val) in change_set.write_register_array {
            self.register_arrays.write(&name, idx, val)?;
        }

        Ok(change_set.goto)
    }

    // --------------------------------
    // Getters
    // --------------------------------

    pub fn registers(&self) -> &RegistersState {
        &self.registers
    }

    pub fn buses(&self) -> &BusesState {
        &self.buses
    }

    pub fn memories(&self) -> &MemoriesState {
        &self.memories
    }

    pub fn register_arrays(&self) -> &RegisterArraysState {
        &self.register_arrays
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "--- Registers ---\n{}\n\n", self.registers)?;
        write!(f, "--- Buses ---\n{}\n\n", self.buses)?;
        write!(f, "--- Register Arrays ---\n{}\n\n", self.register_arrays)?;
        write!(f, "--- Memories ---\n{}", self.memories)?;

        Ok(())
    }
}

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
