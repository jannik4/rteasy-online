use crate::{CompilerError, CompilerErrorKind};
use std::collections::{HashMap, HashSet};
use std::fmt;

const MAX_BIT_RANGE_SIZE: usize = u16::MAX as usize;
const MAX_BIT_RANGE_SIZE_ADDRESS_REGISTER: usize = 64;

#[derive(Debug, Default)]
pub struct Symbols<'s> {
    symbols: HashMap<rtast::Ident<'s>, Symbol<'s>>,
    labels: HashSet<rtast::Label<'s>>,
}

impl<'s> Symbols<'s> {
    pub fn build(ast: &rtast::Ast<'s>, error_sink: &mut impl FnMut(CompilerError)) -> Self {
        let mut symbols = Self::default();

        // Check declarations
        for declaration in &ast.declarations {
            match declaration {
                rtast::Declaration::Register(declare_register) => {
                    for reg in &declare_register.registers {
                        if symbols
                            .symbols
                            .insert(
                                reg.ident.node,
                                Symbol::Register(reg.range.map(|s| s.node), declare_register.kind),
                            )
                            .is_some()
                        {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::DuplicateSymbol(reg.ident.node.0.to_string()),
                                reg.ident.span,
                            ));
                        }

                        if let Some(range) = reg.range {
                            let size = range.node.size();
                            if size > MAX_BIT_RANGE_SIZE {
                                error_sink(CompilerError::new(
                                    CompilerErrorKind::BitRangeTooWide {
                                        max_size: MAX_BIT_RANGE_SIZE,
                                        size,
                                    },
                                    range.span,
                                ));
                            }
                        }
                    }
                }
                rtast::Declaration::Bus(declare_bus) => {
                    for bus in &declare_bus.buses {
                        if symbols
                            .symbols
                            .insert(
                                bus.ident.node,
                                Symbol::Bus(bus.range.map(|s| s.node), declare_bus.kind),
                            )
                            .is_some()
                        {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::DuplicateSymbol(bus.ident.node.0.to_string()),
                                bus.ident.span,
                            ));
                        }

                        if let Some(range) = bus.range {
                            let size = range.node.size();
                            if size > MAX_BIT_RANGE_SIZE {
                                error_sink(CompilerError::new(
                                    CompilerErrorKind::BitRangeTooWide {
                                        max_size: MAX_BIT_RANGE_SIZE,
                                        size,
                                    },
                                    range.span,
                                ));
                            }
                        }
                    }
                }
                rtast::Declaration::Memory(declare_memory) => {
                    for memory in &declare_memory.memories {
                        if symbols
                            .symbols
                            .insert(memory.ident.node, Symbol::Memory(memory.range))
                            .is_some()
                        {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::DuplicateSymbol(memory.ident.node.0.to_string()),
                                memory.ident.span,
                            ));
                        }

                        for (mem_reg, is_ar) in [
                            (&memory.range.address_register, true),
                            (&memory.range.data_register, false),
                        ] {
                            match symbols.symbol(mem_reg.node) {
                                Some(Symbol::Register(range, _)) => {
                                    let size = range.map(|r| r.size()).unwrap_or(1);
                                    if is_ar && size > MAX_BIT_RANGE_SIZE_ADDRESS_REGISTER {
                                        error_sink(CompilerError::new(
                                            CompilerErrorKind::BitRangeTooWide {
                                                max_size: MAX_BIT_RANGE_SIZE_ADDRESS_REGISTER,
                                                size,
                                            },
                                            memory.range.address_register.span,
                                        ));
                                    }
                                }
                                Some(symbol) => error_sink(CompilerError::new(
                                    CompilerErrorKind::WrongSymbolType {
                                        expected: &[SymbolType::Register],
                                        found: symbol.type_(),
                                    },
                                    mem_reg.span,
                                )),
                                None => error_sink(CompilerError::new(
                                    CompilerErrorKind::SymbolNotFound(
                                        &[SymbolType::Register],
                                        mem_reg.node.0.to_string(),
                                    ),
                                    mem_reg.span,
                                )),
                            }
                        }
                    }
                }
                rtast::Declaration::RegisterArray(declare_register_array) => {
                    for reg_array in &declare_register_array.register_arrays {
                        if !reg_array.len.is_power_of_two() {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::RegArrayLenNotPowerOfTwo(
                                    reg_array.ident.node.0.to_string(),
                                ),
                                reg_array.span,
                            ));
                        }

                        if symbols
                            .symbols
                            .insert(
                                reg_array.ident.node,
                                Symbol::RegisterArray {
                                    range: reg_array.range.map(|s| s.node),
                                    len: reg_array.len,
                                },
                            )
                            .is_some()
                        {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::DuplicateSymbol(
                                    reg_array.ident.node.0.to_string(),
                                ),
                                reg_array.ident.span,
                            ));
                        }

                        if let Some(range) = reg_array.range {
                            let size = range.node.size();
                            if size > MAX_BIT_RANGE_SIZE {
                                error_sink(CompilerError::new(
                                    CompilerErrorKind::BitRangeTooWide {
                                        max_size: MAX_BIT_RANGE_SIZE,
                                        size,
                                    },
                                    range.span,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Check labels
        for statement in &ast.statements {
            if let Some(label) = statement.label {
                if !symbols.labels.insert(label.node) {
                    error_sink(CompilerError::new(
                        CompilerErrorKind::DuplicateLabel(label.node.0.to_string()),
                        label.span,
                    ));
                }
            }
        }
        if let Some(label) = ast.trailing_label {
            if !symbols.labels.insert(label.node) {
                error_sink(CompilerError::new(
                    CompilerErrorKind::DuplicateLabel(label.node.0.to_string()),
                    label.span,
                ));
            }
        }

        symbols
    }

    pub fn symbol(&self, ident: rtast::Ident<'s>) -> Option<Symbol<'s>> {
        self.symbols.get(&ident).copied()
    }

    pub fn contains_label(&self, label: rtast::Label<'s>) -> bool {
        self.labels.contains(&label)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol<'s> {
    Register(Option<rtast::BitRange>, rtast::RegisterKind),
    Bus(Option<rtast::BitRange>, rtast::BusKind),
    Memory(rtast::MemoryRange<'s>),
    RegisterArray { range: Option<rtast::BitRange>, len: usize },
}

impl Symbol<'_> {
    pub fn type_(&self) -> SymbolType {
        match self {
            Self::Register(_, _) => SymbolType::Register,
            Self::Bus(_, _) => SymbolType::Bus,
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

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register => write!(f, "register"),
            Self::Bus => write!(f, "bus"),
            Self::Memory => write!(f, "memory"),
            Self::RegisterArray => write!(f, "register array"),
        }
    }
}
