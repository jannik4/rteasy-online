use rt_easy::rtcore::value::{Value, ValueSlice};
use rt_easy::simulator::Error as SimulatorError;
use wasm_bindgen::prelude::*;

pub fn map_err<T>(f: impl FnOnce() -> std::result::Result<T, JsError>) -> Result<T, JsValue> {
    f().map_err(|e| e.0)
}

pub struct JsError(pub JsValue);

impl JsError {
    pub fn from_str(s: &str) -> Self {
        Self(JsValue::from_str(s))
    }
}

impl From<JsValue> for JsError {
    fn from(e: JsValue) -> Self {
        Self(e)
    }
}

impl From<SimulatorError> for JsError {
    fn from(e: SimulatorError) -> Self {
        Self(format!("{:#?}", e).into())
    }
}

pub trait ValueSliceExt {
    fn as_base(&self, base: &str) -> Result<String, JsError>;
}

impl ValueSliceExt for ValueSlice {
    fn as_base(&self, base: &str) -> Result<String, JsError> {
        match base {
            "BIN" => Ok(self.as_bin(true)),
            "DEC" => Ok(self.as_dec()),
            "HEX" => Ok(self.as_hex()),
            _ => Err(JsError::from_str(&format!("invalid base: {}", base))),
        }
    }
}

pub trait ValueExt {
    fn parse_with_base(value: &str, base: &str) -> Result<Value, JsError>;
}

impl ValueExt for Value {
    fn parse_with_base(value: &str, base: &str) -> Result<Value, JsError> {
        let parse_result = match base {
            "BIN" => Value::parse_bin(value),
            "DEC" => Value::parse_dec(value),
            "HEX" => Value::parse_hex(value),
            _ => return Err(JsError::from_str(&format!("invalid base: {}", base))),
        };

        match parse_result {
            Ok(value) => Ok(value),
            Err(()) => Err(JsError::from_str(&format!("invalid value: {}", value))),
        }
    }
}
