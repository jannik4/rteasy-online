use crate::vhdl::*;
use compiler::mir;

pub fn generate_expression<'s>(expression: &mir::Expression<'s>) -> Expression<'s> {
    match expression {
        mir::Expression::Atom(atom) => Expression::Atom(generate_atom(atom)),
        mir::Expression::BinaryTerm(binary_term) => Expression::BinaryTerm(Box::new(BinaryTerm {
            lhs: generate_expression(&binary_term.lhs),
            rhs: generate_expression(&binary_term.rhs),
            operator: binary_term.operator.node,
            ctx_size: binary_term.ctx_size,
        })),
        mir::Expression::UnaryTerm(unary_term) => Expression::UnaryTerm(Box::new(UnaryTerm {
            expression: generate_expression(&unary_term.expression),
            operator: unary_term.operator.node,
            ctx_size: unary_term.ctx_size,
        })),
    }
}

pub fn generate_atom<'s>(atom: &mir::Atom<'s>) -> Atom<'s> {
    match atom {
        mir::Atom::Concat(_concat) => todo!(),
        mir::Atom::Register(reg) => Atom::Register(generate_register(reg)),
        mir::Atom::Bus(bus) => Atom::Bus(generate_bus(bus)),
        mir::Atom::RegisterArray(reg_array) => {
            Atom::RegisterArray(generate_register_array(reg_array))
        }
        mir::Atom::Number(number) => Atom::Number(Number { value: number.node.value.clone() }),
    }
}

pub fn generate_register<'s>(reg: &mir::Register<'s>) -> Register<'s> {
    Register { ident: reg.ident.node, range: reg.range.map(|s| s.node), kind: reg.kind }
}

pub fn generate_bus<'s>(bus: &mir::Bus<'s>) -> Bus<'s> {
    Bus { ident: bus.ident.node, range: bus.range.map(|s| s.node), kind: bus.kind }
}

pub fn generate_register_array<'s>(reg_array: &mir::RegisterArray<'s>) -> RegisterArray<'s> {
    RegisterArray {
        ident: reg_array.ident.node.into(),
        index: Box::new(generate_expression(&reg_array.index)),
        index_ctx_size: reg_array.index_ctx_size,
    }
}
