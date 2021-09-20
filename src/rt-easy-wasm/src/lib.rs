#![deny(rust_2018_idioms)]

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn setPanicHook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn check(code: String) -> Result<(), JsValue> {
    let ast = match rt_easy::parser::parse(&code) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(&e, &code, true)))
        }
    };

    match rt_easy::compiler::check(ast, &Default::default()) {
        Ok(()) => (),
        Err(e) => return Err(JsValue::from_str(&e.pretty_print(&code, None, true))),
    };

    Ok(())
}

#[wasm_bindgen]
pub fn build(code: String) -> Result<Simulator, JsValue> {
    let ast = match rt_easy::parser::parse(&code) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(&e, &code, true)))
        }
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    let program = match rt_easy::compiler::compile(&backend, ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => return Err(JsValue::from_str(&e.pretty_print(&code, None, true))),
    };

    Ok(Simulator(rt_easy::simulator::Simulator::init(program)))
}

#[wasm_bindgen]
pub struct Simulator(rt_easy::simulator::Simulator);

#[wasm_bindgen]
impl Simulator {
    pub fn reset(&mut self) {
        self.0.reset();
    }

    /// Returns `true` if the simulator is finished.
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }

    pub fn cycle_count(&self) -> usize {
        self.0.cycle_count()
    }

    pub fn micro_step(&mut self) -> Result<Option<StepResult>, JsValue> {
        match self.0.micro_step() {
            Ok(step_result) => Ok(step_result.map(Into::into)),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn step(&mut self) -> Result<Option<StepResult>, JsValue> {
        match self.0.step() {
            Ok(step_result) => Ok(step_result.map(Into::into)),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn registers(&self, kind: &str) -> Result<Vec<JsValue>, JsValue> {
        let kind = match kind {
            "Intern" => rt_easy::rtcore::program::RegisterKind::Intern,
            "Output" => rt_easy::rtcore::program::RegisterKind::Output,
            _ => return Err(JsValue::from_str(&format!("invalid register kind: {:?}", kind))),
        };
        let mut registers =
            self.0.registers(kind).map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        registers.sort();

        Ok(registers.into_iter().map(Into::into).collect())
    }

    pub fn register_value(&self, name: &str, base: &str) -> Result<String, JsValue> {
        let value = match self.0.register_value(&rt_easy::rtcore::program::Ident(name.to_string()))
        {
            Ok(value) => value,
            Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
        };

        let value = match base {
            "BIN" => value.as_bin(),
            "DEC" => value.as_dec(),
            "HEX" => value.as_hex(),
            _ => return Err(JsValue::from_str("invalid base")),
        };

        Ok(value)
    }

    pub fn register_value_next(&self, name: &str, base: &str) -> Result<Option<String>, JsValue> {
        let value =
            match self.0.register_value_next(&rt_easy::rtcore::program::Ident(name.to_string())) {
                Ok(value) => value,
                Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
            };

        let value = match value {
            Some(value) => match base {
                "BIN" => Some(value.as_bin()),
                "DEC" => Some(value.as_dec()),
                "HEX" => Some(value.as_hex()),
                _ => return Err(JsValue::from_str("invalid base")),
            },
            None => None,
        };

        Ok(value)
    }

    pub fn write_into_register(
        &mut self,
        name: &str,
        value: &str,
        base: &str,
    ) -> Result<(), JsValue> {
        let value = match base {
            "BIN" => rt_easy::rtcore::value::Value::parse_bin(value),
            "DEC" => rt_easy::rtcore::value::Value::parse_dec(value),
            "HEX" => rt_easy::rtcore::value::Value::parse_hex(value),
            _ => return Err(JsValue::from_str("invalid base")),
        };
        let value = value.map_err(|()| JsValue::from_str("invalid value"))?;

        self.0
            .write_register(&rt_easy::rtcore::program::Ident(name.to_string()), value)
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;

        Ok(())
    }

    pub fn buses(&self, kind: &str) -> Result<Vec<JsValue>, JsValue> {
        let kind = match kind {
            "Intern" => rt_easy::rtcore::program::BusKind::Intern,
            "Input" => rt_easy::rtcore::program::BusKind::Input,
            _ => return Err(JsValue::from_str(&format!("invalid bus kind: {:?}", kind))),
        };
        let mut buses = self.0.buses(kind).map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        buses.sort();

        Ok(buses.into_iter().map(Into::into).collect())
    }

    pub fn bus_value(&self, name: &str, base: &str) -> Result<String, JsValue> {
        let value = match self.0.bus_value(&rt_easy::rtcore::program::Ident(name.to_string())) {
            Ok(value) => value,
            Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
        };

        let value = match base {
            "BIN" => value.as_bin(),
            "DEC" => value.as_dec(),
            "HEX" => value.as_hex(),
            _ => return Err(JsValue::from_str("invalid base")),
        };

        Ok(value)
    }

    pub fn write_into_bus(&mut self, name: &str, value: &str, base: &str) -> Result<(), JsValue> {
        let value = match base {
            "BIN" => rt_easy::rtcore::value::Value::parse_bin(value),
            "DEC" => rt_easy::rtcore::value::Value::parse_dec(value),
            "HEX" => rt_easy::rtcore::value::Value::parse_hex(value),
            _ => return Err(JsValue::from_str("invalid base")),
        };
        let value = value.map_err(|()| JsValue::from_str("invalid value"))?;

        self.0
            .write_bus(&rt_easy::rtcore::program::Ident(name.to_string()), value)
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;

        Ok(())
    }

    pub fn memories(&self) -> Vec<JsValue> {
        let mut memories = self.0.memories().map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        memories.sort();

        memories.into_iter().map(Into::into).collect()
    }

    pub fn memory_page_count(&self, name: &str) -> Result<String, JsValue> {
        match self.0.memory_page_count(&rt_easy::rtcore::program::Ident(name.to_string())) {
            Ok(value) => Ok(value.as_dec()),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn memory_page_prev(&self, name: &str, page_nr: &str) -> Result<Option<String>, JsValue> {
        let page_nr = match rt_easy::rtcore::value::Value::parse_dec(page_nr) {
            Ok(page_nr) => page_nr,
            Err(()) => return Err(JsValue::from_str("invalid page nr")),
        };

        match self.0.memory_page_prev(&rt_easy::rtcore::program::Ident(name.to_string()), page_nr) {
            Ok(value) => Ok(value.map(|value| value.as_dec())),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn memory_page_next(&self, name: &str, page_nr: &str) -> Result<Option<String>, JsValue> {
        let page_nr = match rt_easy::rtcore::value::Value::parse_dec(page_nr) {
            Ok(page_nr) => page_nr,
            Err(()) => return Err(JsValue::from_str("invalid page nr")),
        };

        match self.0.memory_page_next(&rt_easy::rtcore::program::Ident(name.to_string()), page_nr) {
            Ok(value) => Ok(value.map(|value| value.as_dec())),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn memory_page(
        &self,
        name: &str,
        page_nr: &str,
        base: &str,
    ) -> Result<Vec<JsValue>, JsValue> {
        let page_nr = match rt_easy::rtcore::value::Value::parse_dec(page_nr) {
            Ok(page_nr) => page_nr,
            Err(()) => return Err(JsValue::from_str("invalid page nr")),
        };

        let page =
            match self.0.memory_page(&rt_easy::rtcore::program::Ident(name.to_string()), page_nr) {
                Ok(page) => page,
                Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
            };

        let mut res = Vec::with_capacity(page.len() * 2);
        for (addr, value) in page {
            res.push(JsValue::from_str(&addr.as_hex()));
            res.push(JsValue::from_str(&match base {
                "BIN" => value.as_bin(),
                "DEC" => value.as_dec(),
                "HEX" => value.as_hex(),
                _ => return Err(JsValue::from_str("invalid base")),
            }));
        }

        Ok(res)
    }

    pub fn write_into_memory(
        &mut self,
        name: &str,
        addr: &str,
        value: &str,
        base: &str,
    ) -> Result<(), JsValue> {
        let addr = rt_easy::rtcore::value::Value::parse_hex(addr)
            .map_err(|()| JsValue::from_str("invalid addr"))?;
        let value = match base {
            "BIN" => rt_easy::rtcore::value::Value::parse_bin(value),
            "DEC" => rt_easy::rtcore::value::Value::parse_dec(value),
            "HEX" => rt_easy::rtcore::value::Value::parse_hex(value),
            _ => return Err(JsValue::from_str("invalid base")),
        };
        let value = value.map_err(|()| JsValue::from_str("invalid value"))?;

        self.0
            .write_memory(&rt_easy::rtcore::program::Ident(name.to_string()), addr, value)
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;

        Ok(())
    }

    pub fn memory_save(&self, name: &str) -> Result<String, JsValue> {
        let mut save_bytes = Vec::new();
        self.0
            .memory_save(&rt_easy::rtcore::program::Ident(name.to_string()), &mut save_bytes)
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;
        let save =
            String::from_utf8(save_bytes).map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;
        Ok(save)
    }

    pub fn memory_load_from_save(&mut self, name: &str, save: &str) -> Result<(), JsValue> {
        self.0
            .memory_load_from_save(
                &rt_easy::rtcore::program::Ident(name.to_string()),
                save.as_bytes(),
            )
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;
        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct StepResult {
    pub is_at_statement_start: bool,
    pub span: Span,
    pub condition: Option<StepResultCondition>,
}

impl From<rt_easy::simulator::StepResult> for StepResult {
    fn from(step_result: rt_easy::simulator::StepResult) -> Self {
        Self {
            is_at_statement_start: step_result.is_at_statement_start,
            span: step_result.span.into(),
            condition: step_result
                .condition
                .map(|(value, span)| StepResultCondition { value, span: span.into() }),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct StepResultCondition {
    pub value: bool,
    pub span: Span,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<rt_easy::rtcore::program::Span> for Span {
    fn from(span: rt_easy::rtcore::program::Span) -> Self {
        Self { start: span.start, end: span.end }
    }
}
