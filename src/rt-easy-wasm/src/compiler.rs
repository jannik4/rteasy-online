use crate::Simulator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check(code: String) -> Result<(), JsValue> {
    let ast = match rt_easy::parser::parse(&code) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(
                &e, &code, None, true,
            )))
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
            return Err(JsValue::from_str(&rt_easy::parser::pretty_print_error(
                &e, &code, None, true,
            )))
        }
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    let program = match rt_easy::compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => return Err(JsValue::from_str(&e.pretty_print(&code, None, true))),
    };

    Ok(Simulator(rt_easy::simulator::Simulator::init(program)))
}
