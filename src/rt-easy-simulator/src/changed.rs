use rtcore::{program::Ident, value::Value};

#[derive(Debug)]
pub enum Changed {
    Register { name: Ident },
    RegisterArray { name: Ident, index: Value },
    Memory { name: Ident, address: Value },
}
