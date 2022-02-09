pub type Result<T> = std::result::Result<T, Error>;

// TODO: Replace JsError in rt-easy-wasm when replacing this with thiserror
pub type Error = anyhow::Error;
