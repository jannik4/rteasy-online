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
#[allow(non_snake_case)]
pub fn sayHello(name: String) -> Result<String, JsValue> {
    Ok(format!("Hello {}", name))
}

#[wasm_bindgen]
pub fn parse(code: String) -> String {
    match rt_easy::parser::parse(&code) {
        Ok(ast) => format!("{:#?}", ast),
        Err(e) => format!("{}", e),
    }
}

#[wasm_bindgen]
pub fn run(code: String) -> String {
    let ast = match rt_easy::parser::parse(&code) {
        Ok(ast) => ast,
        Err(e) => return rt_easy::parser::pretty_print_error(&e, &code),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    let program = match rt_easy::compiler::compile(&backend, ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => return format!("{:#?}", e),
    };

    let mut simulator = rt_easy::simulator::Simulator::init(program);
    for _ in 0..1000 {
        if simulator.is_finished() {
            break;
        }
        simulator.step().unwrap();
    }

    format!("{}", simulator.state())
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
pub struct Simulator(rt_easy::simulator::Simulator);

#[wasm_bindgen]
impl Simulator {
    /// Returns `true` if the simulator is finished.
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }

    pub fn step(&mut self) -> Option<Span> {
        let span = self.0.micro_step().unwrap();
        match span {
            Some(span) => Some(Span { start: span.start, end: span.end }),
            None => None,
        }
    }

    pub fn read_reg_a(&self) -> String {
        self.0
            .state()
            .read_register(&rt_easy::rtcore::program::Ident("A".to_string()), None)
            .unwrap()
            .as_dec()
    }

    pub fn state(&self) -> String {
        format!("{}", self.0.state())
    }
}

#[wasm_bindgen]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
