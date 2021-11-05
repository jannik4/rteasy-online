use crate::{util::*, Signals, Span, StepResult};
use rt_easy::rtcore::{
    program::{BusKind, Ident, RegisterKind},
    value::{SignedValue, Value},
};
use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
pub struct Simulator(pub(crate) rt_easy::simulator::Simulator);

#[wasm_bindgen]
impl Simulator {
    pub fn reset(&mut self) {
        self.0.reset(false);
    }

    pub fn cycle_count(&self) -> usize {
        self.0.cycle_count()
    }

    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }

    pub fn signals(&self) -> Signals {
        Signals(self.0.signals())
    }

    pub fn statement_span(&self, statement: usize) -> Option<Span> {
        self.0.statement_span(statement).map(Into::into)
    }

    pub fn add_breakpoint(&mut self, statement: usize) {
        self.0.add_breakpoint(statement);
    }

    pub fn remove_breakpoint(&mut self, statement: usize) {
        self.0.remove_breakpoint(statement);
    }

    pub fn breakpoints(&self) -> Vec<usize> {
        self.0.breakpoints().collect()
    }

    pub fn micro_step(&mut self, stop_on_breakpoint: bool) -> Result<Option<StepResult>> {
        map_err(move || {
            let step_result = self.0.micro_step(stop_on_breakpoint)?;
            Ok(step_result.map(Into::into))
        })
    }

    pub fn step(&mut self, stop_on_breakpoint: bool) -> Result<Option<StepResult>> {
        map_err(move || {
            let step_result = self.0.step(stop_on_breakpoint)?;
            Ok(step_result.map(Into::into))
        })
    }

    pub fn registers(&self, kind: &str) -> Result<Vec<JsValue>> {
        let kind = match kind {
            "Intern" => RegisterKind::Intern,
            "Output" => RegisterKind::Output,
            _ => return Err(JsValue::from_str(&format!("invalid register kind: {:?}", kind))),
        };
        let mut registers =
            self.0.registers(kind).map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        registers.sort();

        Ok(registers.into_iter().map(Into::into).collect())
    }

    pub fn register_value(&self, name: String, base: &str) -> Result<String> {
        map_err(move || Ok(self.0.register_value(&Ident(name))?.as_base(base)?))
    }

    pub fn register_value_next(&self, name: String, base: &str) -> Result<Option<String>> {
        map_err(move || {
            let value = match self.0.register_value_next(&Ident(name))? {
                Some(value) => Some(value.as_base(base)?),
                None => None,
            };
            Ok(value)
        })
    }

    pub fn write_register(&mut self, name: String, value: &str, base: &str) -> Result<()> {
        map_err(move || {
            let value = SignedValue::parse_with_base(value, base)?;
            self.0.write_register(&Ident(name), value)?;
            Ok(())
        })
    }

    pub fn buses(&self, kind: &str) -> Result<Vec<JsValue>> {
        let kind = match kind {
            "Intern" => BusKind::Intern,
            "Input" => BusKind::Input,
            _ => return Err(JsValue::from_str(&format!("invalid bus kind: {:?}", kind))),
        };
        let mut buses = self.0.buses(kind).map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        buses.sort();

        Ok(buses.into_iter().map(Into::into).collect())
    }

    pub fn bus_value(&self, name: String, base: &str) -> Result<String> {
        map_err(move || Ok(self.0.bus_value(&Ident(name))?.as_base(base)?))
    }

    pub fn write_bus(&mut self, name: String, value: &str, base: &str) -> Result<()> {
        map_err(move || {
            let value = SignedValue::parse_with_base(value, base)?;
            self.0.write_bus(&Ident(name), value)?;
            Ok(())
        })
    }

    pub fn register_arrays(&self) -> Vec<JsValue> {
        let mut register_arrays =
            self.0.register_arrays().map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        register_arrays.sort();
        register_arrays.into_iter().map(Into::into).collect()
    }

    pub fn register_array_value_next(
        &self,
        name: String,
        base: &str,
    ) -> Result<Option<Vec<JsValue>>> {
        map_err(move || {
            let (idx, value) = match self.0.register_array_value_next(&Ident(name))? {
                Some((idx, value)) => (idx, value.as_base(base)?),
                None => return Ok(None),
            };
            Ok(Some(vec![JsValue::from_f64(idx as f64), JsValue::from_str(&value)]))
        })
    }

    pub fn register_array_page_count(&self, name: String) -> Result<usize> {
        map_err(move || Ok(self.0.register_array_page_count(&Ident(name))?))
    }

    pub fn register_array_page(
        &self,
        name: String,
        page_nr: usize,
        base: &str,
    ) -> Result<Vec<JsValue>> {
        map_err(move || {
            let page = self.0.register_array_page(&Ident(name), page_nr)?;

            let mut res = Vec::with_capacity(page.len() * 2);
            for (idx, value) in page {
                res.push(JsValue::from_f64(idx as f64));
                res.push(JsValue::from_str(&value.as_base(base)?));
            }

            Ok(res)
        })
    }

    pub fn write_register_array(
        &mut self,
        name: String,
        idx: usize,
        value: &str,
        base: &str,
    ) -> Result<()> {
        map_err(move || {
            let value = SignedValue::parse_with_base(value, base)?;
            self.0.write_register_array(&Ident(name), idx, value)?;
            Ok(())
        })
    }

    pub fn memories(&self) -> Vec<JsValue> {
        let mut memories = self.0.memories().map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        memories.sort();

        memories.into_iter().map(Into::into).collect()
    }

    pub fn memory_value_next(&self, name: String, base: &str) -> Result<Option<Vec<JsValue>>> {
        map_err(move || {
            let (addr, value) = match self.0.memory_value_next(&Ident(name))? {
                Some((addr, value)) => (addr, value.as_base(base)?),
                None => return Ok(None),
            };
            Ok(Some(vec![JsValue::from_str(&addr.as_hex()), JsValue::from_str(&value)]))
        })
    }

    pub fn memory_page_count(&self, name: String) -> Result<String> {
        map_err(move || Ok(self.0.memory_page_count(&Ident(name))?.as_dec()))
    }

    pub fn memory_page_prev(&self, name: String, page_nr: &str) -> Result<Option<String>> {
        map_err(move || {
            let page_nr = match Value::parse_dec(page_nr) {
                Ok(page_nr) => page_nr,
                Err(()) => return Err(JsError::from_str("invalid page nr")),
            };
            Ok(self.0.memory_page_prev(&Ident(name), page_nr)?.map(|value| value.as_dec()))
        })
    }

    pub fn memory_page_next(&self, name: String, page_nr: &str) -> Result<Option<String>> {
        map_err(move || {
            let page_nr = match Value::parse_dec(page_nr) {
                Ok(page_nr) => page_nr,
                Err(()) => return Err(JsError::from_str("invalid page nr")),
            };
            Ok(self.0.memory_page_next(&Ident(name), page_nr)?.map(|value| value.as_dec()))
        })
    }

    pub fn memory_page_nr_of_address(&self, name: String, address: &str) -> Result<Option<String>> {
        map_err(move || {
            let address = match Value::parse_hex(address) {
                Ok(address) => address,
                Err(()) => return Err(JsError::from_str("invalid address")),
            };
            Ok(self.0.memory_page_nr_of_address(&Ident(name), address)?.map(|value| value.as_dec()))
        })
    }

    pub fn memory_page(&self, name: String, page_nr: &str, base: &str) -> Result<Vec<JsValue>> {
        map_err(move || {
            let page_nr = match Value::parse_dec(page_nr) {
                Ok(page_nr) => page_nr,
                Err(()) => return Err(JsError::from_str("invalid page nr")),
            };
            let page = self.0.memory_page(&Ident(name), page_nr)?;

            let mut res = Vec::with_capacity(page.len() * 2);
            for (addr, value) in page {
                res.push(JsValue::from_str(&addr.as_hex()));
                res.push(JsValue::from_str(&value.as_base(base)?));
            }

            Ok(res)
        })
    }

    pub fn write_memory(
        &mut self,
        name: String,
        addr: &str,
        value: &str,
        base: &str,
    ) -> Result<()> {
        map_err(move || {
            let addr = Value::parse_hex(addr).map_err(|()| JsError::from_str("invalid addr"))?;
            let value = SignedValue::parse_with_base(value, base)?;
            self.0.write_memory(&Ident(name), addr, value)?;
            Ok(())
        })
    }

    pub fn save_memory(&self, name: String) -> Result<String> {
        map_err(move || {
            let mut save_bytes = Vec::new();
            self.0.save_memory(&Ident(name), &mut save_bytes)?;
            let save = String::from_utf8(save_bytes)
                .map_err(|e| JsError::from_str(&format!("{:#?}", e)))?;
            Ok(save)
        })
    }

    pub fn load_memory_from_save(&mut self, name: String, save: &str) -> Result<()> {
        map_err(move || {
            self.0.load_memory_from_save(&Ident(name), save.as_bytes())?;
            Ok(())
        })
    }
}
