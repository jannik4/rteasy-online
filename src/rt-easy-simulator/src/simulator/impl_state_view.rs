use super::Simulator;
use crate::Error;
use rtcore::{
    program::{BusKind, Ident, RegisterKind},
    value::Value,
};

impl Simulator {
    // ------------------------------------------------------------
    // Registers
    // ------------------------------------------------------------

    pub fn registers(&self, kind: RegisterKind) -> impl Iterator<Item = &Ident> {
        self.state.register_names(kind)
    }
    pub fn register_value(&self, name: &Ident) -> Result<Value, Error> {
        self.state.register(name)?.read(None)
    }
    pub fn register_value_next(&self, name: &Ident) -> Result<Option<Value>, Error> {
        Ok(self.state.register(name)?.value_next())
    }
    pub fn write_register(&mut self, name: &Ident, value: Value) -> Result<(), Error> {
        self.state.register_mut(name)?.write(None, value)?;
        self.state.register_mut(name)?.clock();
        Ok(())
    }

    // ------------------------------------------------------------
    // Buses
    // ------------------------------------------------------------

    pub fn buses(&self, kind: BusKind) -> impl Iterator<Item = &Ident> {
        self.state.bus_names(kind)
    }
    pub fn bus_value(&self, name: &Ident) -> Result<Value, Error> {
        self.state.bus(name)?.read(None)
    }
    pub fn write_bus(&mut self, name: &Ident, value: Value) -> Result<(), Error> {
        self.state.bus_mut(name)?.write(None, value)?;

        // Persist bus value if between statements
        if self.cursor.is_at_statement_start() {
            self.buses_persist.insert(name.clone());
        }

        Ok(())
    }

    // ------------------------------------------------------------
    // Register arrays
    // ------------------------------------------------------------

    pub fn register_arrays(&self) -> impl Iterator<Item = &Ident> {
        self.state.register_array_names()
    }
    pub fn register_array_value_next(&self, name: &Ident) -> Result<Option<(usize, Value)>, Error> {
        Ok(self.state.register_array(name)?.value_next())
    }
    pub fn register_array_page_count(&self, name: &Ident) -> Result<usize, Error> {
        Ok(self.state.register_array(name)?.page_count())
    }
    pub fn register_array_page(
        &self,
        name: &Ident,
        page_nr: usize,
    ) -> Result<Vec<(usize, Value)>, Error> {
        Ok(self.state.register_array(name)?.page(page_nr))
    }
    pub fn write_register_array(
        &mut self,
        name: &Ident,
        idx: usize,
        value: Value,
    ) -> Result<(), Error> {
        let reg_array_state = self.state.register_array_mut(name)?;
        let idx = Value::parse_bin(&format!("{:b}", idx)).unwrap();
        reg_array_state.write(idx, value)?;
        reg_array_state.clock();
        Ok(())
    }

    // ------------------------------------------------------------
    // Memories
    // ------------------------------------------------------------

    pub fn memories(&self) -> impl Iterator<Item = &Ident> {
        self.state.memory_names()
    }
    pub fn memory_value_next(&self, name: &Ident) -> Result<Option<(Value, Value)>, Error> {
        Ok(self.state.memory(name)?.value_next())
    }
    pub fn memory_page_count(&self, name: &Ident) -> Result<Value, Error> {
        Ok(self.state.memory(name)?.page_count())
    }
    pub fn memory_page_prev(&self, name: &Ident, page_nr: Value) -> Result<Option<Value>, Error> {
        Ok(self.state.memory(name)?.page_prev(page_nr))
    }
    pub fn memory_page_next(&self, name: &Ident, page_nr: Value) -> Result<Option<Value>, Error> {
        Ok(self.state.memory(name)?.page_next(page_nr))
    }
    pub fn memory_page_nr_of_address(
        &self,
        name: &Ident,
        address: Value,
    ) -> Result<Option<Value>, Error> {
        Ok(self.state.memory(name)?.page_nr_of_address(address))
    }
    pub fn memory_page(&self, name: &Ident, page_nr: Value) -> Result<Vec<(Value, Value)>, Error> {
        Ok(self.state.memory(name)?.page(page_nr))
    }
    pub fn write_memory(&mut self, name: &Ident, addr: Value, value: Value) -> Result<(), Error> {
        Ok(self.state.memory_mut(name)?.write_at(addr, value)?)
    }
    pub fn save_memory<W>(&self, name: &Ident, writer: W) -> Result<(), Error>
    where
        W: std::io::Write,
    {
        self.state.memory(name)?.save(writer)
    }
    pub fn load_memory_from_save<R>(&mut self, name: &Ident, reader: R) -> Result<(), Error>
    where
        R: std::io::Read,
    {
        self.state.memory_mut(name)?.load_from_save(reader)
    }
}