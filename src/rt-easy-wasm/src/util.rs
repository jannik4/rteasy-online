use rt_easy::rtcore::value::{SignedValue, ValueSlice};
use rt_easy::simulator::Error as SimulatorError;
use wasm_bindgen::prelude::*;

pub fn map_err<T>(f: impl FnOnce() -> std::result::Result<T, JsError>) -> Result<T, JsValue> {
    f().map_err(|e| e.0)
}

// TODO: Use wasm_bindgen::Error when SimulatorError implements StdError
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
        Self(format!("{}", e).into())
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

pub trait ValueExt: Sized {
    fn parse_with_base(value: &str, base: &str) -> Result<Self, JsError>;
}

// impl ValueExt for Value {
//     fn parse_with_base(value: &str, base: &str) -> Result<Self, JsError> {
//         let parse_result = match base {
//             "BIN" => Value::parse_bin(value),
//             "DEC" => Value::parse_dec(value),
//             "HEX" => Value::parse_hex(value),
//             _ => return Err(JsError::from_str(&format!("invalid base: {}", base))),
//         };
//
//         match parse_result {
//             Ok(value) => Ok(value),
//             Err(()) => Err(JsError::from_str(&format!("invalid value: {}", value))),
//         }
//     }
// }

impl ValueExt for SignedValue {
    fn parse_with_base(value: &str, base: &str) -> Result<Self, JsError> {
        let parse_result = match base {
            "BIN" => SignedValue::parse_bin(value),
            "DEC" => SignedValue::parse_dec(value),
            "HEX" => SignedValue::parse_hex(value),
            _ => return Err(JsError::from_str(&format!("invalid base: {}", base))),
        };

        match parse_result {
            Ok(value) => Ok(value),
            Err(()) => Err(JsError::from_str(&format!("invalid value: {}", value))),
        }
    }
}
