use crate::render_as_vhdl::RenderAsVhdl;
use crate::*;
use indexmap::IndexSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item<'a> {
    Register(&'a Ident, RegisterKind),
    Bus(&'a Ident, BusKind),
    RegisterArray(&'a Ident),
}

impl Display for RenderAsVhdl<&Item<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Item::Register(name, _kind) => {
                write!(f, "register_{}", name)
            }
            Item::Bus(name, kind) => {
                let prefix = match kind {
                    BusKind::Intern => "bus",
                    BusKind::Input => "input",
                };
                write!(f, "{}_{}", prefix, name)
            }
            Item::RegisterArray(name) => write!(f, "register_array_{}", name),
        }
    }
}

pub fn build<'a>(expressions: impl IntoIterator<Item = &'a Expression>) -> IndexSet<Item<'a>> {
    let mut items = IndexSet::new();
    for expr in expressions {
        insert_expression(expr, &mut items);
    }
    items
}

fn insert_expression<'a>(expr: &'a Expression, items: &mut IndexSet<Item<'a>>) {
    match &expr.kind {
        ExpressionKind::Atom(Atom::Concat(concat)) => {
            for part in &concat.parts {
                match part {
                    ConcatPartExpr::Register(reg) => insert_register(reg, items),
                    ConcatPartExpr::Bus(bus) => insert_bus(bus, items),
                    ConcatPartExpr::RegisterArray(reg_array) => {
                        insert_register_array(reg_array, items)
                    }
                    ConcatPartExpr::Number(_) => (),
                }
            }
        }
        ExpressionKind::Atom(Atom::Register(reg)) => insert_register(reg, items),
        ExpressionKind::Atom(Atom::Bus(bus)) => insert_bus(bus, items),
        ExpressionKind::Atom(Atom::RegisterArray(reg_array)) => {
            insert_register_array(reg_array, items)
        }
        ExpressionKind::Atom(Atom::Number(_)) => (),
        ExpressionKind::BinaryTerm(term) => {
            insert_expression(&term.lhs, items);
            insert_expression(&term.rhs, items);
        }
        ExpressionKind::UnaryTerm(term) => insert_expression(&term.expression, items),
    }
}

fn insert_register<'a>(reg: &'a Register, items: &mut IndexSet<Item<'a>>) {
    items.insert(Item::Register(&reg.ident, reg.kind));
}

fn insert_bus<'a>(bus: &'a Bus, items: &mut IndexSet<Item<'a>>) {
    items.insert(Item::Bus(&bus.ident, bus.kind));
}

fn insert_register_array<'a>(reg_array: &'a RegisterArray, items: &mut IndexSet<Item<'a>>) {
    items.insert(Item::RegisterArray(&reg_array.ident));
}
