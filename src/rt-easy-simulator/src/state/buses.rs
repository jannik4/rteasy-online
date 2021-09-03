use super::reg_bus::RegBusState;
use crate::Error;
use rtcore::{
    program::{BitRange, Bus, Declaration, Ident, Program},
    value::Value,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct BusesState {
    buses: HashMap<Ident, RegBusState>,
}

impl BusesState {
    pub fn init(program: &Program) -> Self {
        let mut buses = HashMap::new();

        for declaration in program.declarations() {
            if let Declaration::Bus(declare_bus) = declaration {
                for bus in &declare_bus.buses {
                    buses.insert(bus.ident.clone(), RegBusState::init(bus.range));
                }
            }
        }

        Self { buses }
    }

    pub fn clear(&mut self, buses_persist: &HashSet<Ident>) {
        for (ident, bus) in &mut self.buses {
            if !buses_persist.contains(ident) {
                bus.write(None, Value::zero(bus.range().size())).unwrap();
            }
        }
    }

    pub fn read_full(&self, name: &Ident) -> Result<Value, Error> {
        match self.buses.get(name) {
            Some(state) => Ok(state.read_full()),
            None => Err(Error::Other),
        }
    }

    pub fn read(&self, name: &Ident, range: Option<BitRange>) -> Result<Value, Error> {
        match self.buses.get(name) {
            Some(state) => state.read(range),
            None => Err(Error::Other),
        }
    }

    pub fn write(&mut self, bus: &Bus, value: Value) -> Result<(), Error> {
        match self.buses.get_mut(&bus.ident) {
            Some(state) => state.write(bus.range, value),
            None => Err(Error::Other),
        }
    }

    pub fn range_of(&self, name: &Ident) -> Result<BitRange, Error> {
        match self.buses.get(name) {
            Some(state) => Ok(state.range()),
            None => Err(Error::Other),
        }
    }
}

impl fmt::Display for BusesState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buses = self.buses.keys().collect::<Vec<_>>();
        buses.sort();

        for (idx, bus) in buses.into_iter().enumerate() {
            write!(
                f,
                "{}{} = {}",
                if idx != 0 { "\n" } else { "" },
                bus.0,
                self.read_full(bus).unwrap().as_dec()
            )?;
        }

        Ok(())
    }
}
