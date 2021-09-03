use super::{
    buses::BusesState, memories::MemoriesState, register_arrays::RegisterArraysState,
    registers::RegistersState,
};
use crate::{ChangeSet, Error};
use rtcore::{
    program::{BitRange, Bus, Ident, Label, Program},
    value::Value,
};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct State {
    cycle_count: usize,
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
        Self {
            cycle_count: 0,
            registers,
            buses,
            memories,
            register_arrays, /*curr_span: None*/
        }
    }

    pub fn cycle_count(&self) -> usize {
        self.cycle_count
    }

    pub fn finish_cycle(&mut self) {
        self.cycle_count += 1;
    }

    pub fn apply_change_set(&mut self, change_set: ChangeSet) -> Result<Option<Label>, Error> {
        change_set.apply(&mut self.registers, &mut self.memories, &mut self.register_arrays)
    }

    pub fn range_of_register(&self, name: &Ident) -> Result<BitRange, Error> {
        self.registers.range_of(name)
    }

    pub fn range_of_bus(&self, name: &Ident) -> Result<BitRange, Error> {
        self.buses.range_of(name)
    }

    pub fn read_register(&self, name: &Ident, range: Option<BitRange>) -> Result<Value, Error> {
        self.registers.read(name, range)
    }

    pub fn read_register_array(&self, name: &Ident, idx: Value) -> Result<Value, Error> {
        self.register_arrays.read(name, idx)
    }

    pub fn clear_buses(&mut self, buses_persist: &HashSet<Ident>) {
        self.buses.clear(buses_persist);
    }

    pub fn read_bus(&self, name: &Ident, range: Option<BitRange>) -> Result<Value, Error> {
        self.buses.read(name, range)
    }

    pub fn write_into_bus(&mut self, bus: &Bus, value: Value) -> Result<(), Error> {
        self.buses.write(bus, value)
    }

    pub fn read_memory(&self, name: &Ident, change_set: &mut ChangeSet) -> Result<(), Error> {
        self.memories.read(name, &self.registers, change_set)
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
