use crate::render_as_vhdl::RenderAsVhdl;
use crate::vhdl::*;
use indexmap::IndexSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item<'s> {
    Register(Ident<'s>, RegisterKind),
    Bus(Ident<'s>, BusKind),
    RegisterArray(Ident<'s>),
}

impl Display for RenderAsVhdl<&Item<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Item::Register(name, _kind) => {
                write!(f, "register_{}", name.0)
            }
            Item::Bus(name, kind) => {
                let prefix = match kind {
                    BusKind::Intern => "bus",
                    BusKind::Input => "input",
                };
                write!(f, "{}_{}", prefix, name.0)
            }
            Item::RegisterArray(name) => write!(f, "register_array_{}", name.0),
        }
    }
}

pub fn build<'s>(expressions: impl IntoIterator<Item = &'s Expression<'s>>) -> IndexSet<Item<'s>> {
    let mut items = IndexSet::new();
    for expr in expressions {
        insert_expression(expr, &mut items);
    }
    items
}

fn insert_expression<'s>(expr: &Expression<'s>, items: &mut IndexSet<Item<'s>>) {
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

fn insert_register<'s>(reg: &Register<'s>, items: &mut IndexSet<Item<'s>>) {
    items.insert(Item::Register(reg.ident, reg.kind));
}

fn insert_bus<'s>(bus: &Bus<'s>, items: &mut IndexSet<Item<'s>>) {
    items.insert(Item::Bus(bus.ident, bus.kind));
}

fn insert_register_array<'s>(reg_array: &RegisterArray<'s>, items: &mut IndexSet<Item<'s>>) {
    items.insert(Item::RegisterArray(reg_array.ident));
}
