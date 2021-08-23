use crate::CompilerError;
use rtcore::ast;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Symbols<'s> {
    symbols: HashMap<ast::Ident<'s>, Symbol<'s>>,
    labels: HashSet<ast::Label<'s>>,
}

impl<'s> Symbols<'s> {
    pub fn build(ast: &ast::Ast<'s>, error_sink: &mut impl FnMut(CompilerError)) -> Self {
        let mut symbols = Self::default();

        // Check declarations
        for declaration in &ast.declarations {
            match declaration {
                ast::Declaration::Register(declare_register) => {
                    for reg in &declare_register.registers {
                        if symbols.symbols.insert(reg.ident, Symbol::Register(reg.range)).is_some()
                        {
                            error_sink(CompilerError::DuplicateSymbol(reg.ident.0.to_string()));
                        }

                        // TODO: Instead of this check use u16 in BitRange ???
                        let size = reg.range.map(|r| r.size()).unwrap_or(1);
                        if size > u16::MAX as usize {
                            error_sink(CompilerError::BitRangeToWide(size));
                        }
                    }
                }
                ast::Declaration::Bus(declare_bus) => {
                    for bus in &declare_bus.buses {
                        if symbols.symbols.insert(bus.ident, Symbol::Bus(bus.range)).is_some() {
                            error_sink(CompilerError::DuplicateSymbol(bus.ident.0.to_string()));
                        }

                        // TODO: Instead of this check use u16 in BitRange ???
                        let size = bus.range.map(|r| r.size()).unwrap_or(1);
                        if size > u16::MAX as usize {
                            error_sink(CompilerError::BitRangeToWide(size));
                        }
                    }
                }
                ast::Declaration::Memory(declare_memory) => {
                    for memory in &declare_memory.memories {
                        if symbols
                            .symbols
                            .insert(memory.ident, Symbol::Memory(memory.range))
                            .is_some()
                        {
                            error_sink(CompilerError::DuplicateSymbol(memory.ident.0.to_string()));
                        }

                        for mem_reg in
                            &[&memory.range.address_register, &memory.range.data_register]
                        {
                            match symbols.symbol(**mem_reg) {
                                Some(Symbol::Register(_)) => (),
                                Some(symbol) => error_sink(CompilerError::WrongSymbolType {
                                    expected: &[SymbolType::Register],
                                    found: symbol.type_(),
                                }),
                                None => error_sink(CompilerError::SymbolNotFound(
                                    &[SymbolType::Register],
                                    mem_reg.0.to_string(),
                                )),
                            }
                        }
                    }
                }
                ast::Declaration::RegisterArray(declare_register_array) => {
                    for reg_array in &declare_register_array.register_arrays {
                        if !reg_array.len.is_power_of_two() {
                            error_sink(CompilerError::RegArrayLenNotPowerOfTwo(
                                reg_array.ident.0.to_string(),
                            ));
                        }

                        if symbols
                            .symbols
                            .insert(
                                reg_array.ident,
                                Symbol::RegisterArray {
                                    range: reg_array.range,
                                    len: reg_array.len,
                                },
                            )
                            .is_some()
                        {
                            error_sink(CompilerError::DuplicateSymbol(
                                reg_array.ident.0.to_string(),
                            ));
                        }

                        // TODO: Instead of this check use u16 in BitRange ???
                        let size = reg_array.range.map(|r| r.size()).unwrap_or(1);
                        if size > u16::MAX as usize {
                            error_sink(CompilerError::BitRangeToWide(size));
                        }
                    }
                }
            }
        }

        // Check labels
        for statement in &ast.statements {
            if let Some(label) = statement.label {
                if !symbols.labels.insert(label) {
                    error_sink(CompilerError::DuplicateLabel(label.0.to_string()));
                }
            }
        }
        if let Some(label) = ast.trailing_label {
            if !symbols.labels.insert(label) {
                error_sink(CompilerError::DuplicateLabel(label.0.to_string()));
            }
        }

        symbols
    }

    pub fn symbol(&self, ident: ast::Ident<'s>) -> Option<Symbol<'s>> {
        self.symbols.get(&ident).copied()
    }

    pub fn contains_label(&self, label: ast::Label<'s>) -> bool {
        self.labels.contains(&label)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol<'s> {
    Register(Option<ast::BitRange>),
    Bus(Option<ast::BitRange>),
    Memory(ast::MemoryRange<'s>),
    RegisterArray { range: Option<ast::BitRange>, len: usize },
}

impl Symbol<'_> {
    pub fn type_(&self) -> SymbolType {
        match self {
            Self::Register(_) => SymbolType::Register,
            Self::Bus(_) => SymbolType::Bus,
            Self::Memory(_) => SymbolType::Memory,
            Self::RegisterArray { .. } => SymbolType::RegisterArray,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Register,
    Bus,
    Memory,
    RegisterArray,
}
