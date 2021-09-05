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
        Err(e) => return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(&e, &code))),
    };

    match rt_easy::compiler::check(ast, &Default::default()) {
        Ok(()) => (),
        Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
    };

    Ok(())
}

#[wasm_bindgen]
pub fn build(code: String) -> Result<Simulator, JsValue> {
    let ast = match rt_easy::parser::parse(&code) {
        Ok(ast) => ast,
        Err(e) => return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(&e, &code))),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    let program = match rt_easy::compiler::compile(&backend, ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => return Err(JsValue::from_str(&format!("{:#?}", e))),
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

    pub fn micro_step(&mut self) -> Option<Span> {
        let span = self.0.micro_step().unwrap();
        match span {
            Some(span) => Some(Span { start: span.start, end: span.end }),
            None => None,
        }
    }

    pub fn step(&mut self) -> Option<Span> {
        let span = self.0.step().unwrap();
        match span {
            Some(span) => Some(Span { start: span.start, end: span.end }),
            None => None,
        }
    }

    pub fn registers(&self) -> Vec<JsValue> {
        let mut registers =
            self.0.state().registers().names().map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        registers.sort();

        registers.into_iter().map(Into::into).collect()
    }

    pub fn register_value(&self, name: &str, base: &str) -> Result<String, JsValue> {
        let value = match self
            .0
            .state()
            .registers()
            .read_full(&rt_easy::rtcore::program::Ident(name.to_string()))
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

    pub fn buses(&self) -> Vec<JsValue> {
        let mut buses =
            self.0.state().buses().names().map(|ident| ident.0.to_owned()).collect::<Vec<_>>();
        buses.sort();

        buses.into_iter().map(Into::into).collect()
    }

    pub fn bus_value(&self, name: &str, base: &str) -> Result<String, JsValue> {
        let value = match self
            .0
            .state()
            .buses()
            .read_full(&rt_easy::rtcore::program::Ident(name.to_string()))
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

    pub fn write_into_bus(&mut self, name: &str, value: &str, base: &str) -> Result<(), JsValue> {
        let value = match base {
            "BIN" => rt_easy::rtcore::value::Value::parse_bin(value, false),
            "DEC" => rt_easy::rtcore::value::Value::parse_dec(value),
            "HEX" => rt_easy::rtcore::value::Value::parse_hex(value, false),
            _ => return Err(JsValue::from_str("invalid base")),
        };
        let value = value.map_err(|()| JsValue::from_str("invalid value"))?;

        self.0
            .write_into_bus(
                &rt_easy::rtcore::program::Bus {
                    ident: rt_easy::rtcore::program::Ident(name.to_string()),
                    range: None,
                },
                value,
            )
            .map_err(|e| JsValue::from_str(&format!("{:#?}", e)))?;

        Ok(())
    }
}

#[wasm_bindgen]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
