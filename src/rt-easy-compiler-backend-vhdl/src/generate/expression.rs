use super::concat::generate_concat_expr;
use crate::vhdl::*;
use compiler::mir;

pub fn generate_expression<'s>(
    expression: &mir::Expression<'s>,
    ctx_size: usize,
) -> Expression<'s> {
    let (kind, extend_to) = match expression {
        mir::Expression::Atom(atom) => {
            (ExpressionKind::Atom(generate_atom(atom)), Extend::Zero(ctx_size))
        }
        mir::Expression::BinaryTerm(binary_term) => {
            let ctx_size_inner = binary_term.ctx_size.calc(ctx_size);
            let kind = ExpressionKind::BinaryTerm(Box::new(BinaryTerm {
                lhs: generate_expression(&binary_term.lhs, ctx_size_inner),
                rhs: generate_expression(&binary_term.rhs, ctx_size_inner),
                operator: binary_term.operator.node,
            }));
            (kind, Extend::Zero(ctx_size))
        }
        mir::Expression::UnaryTerm(unary_term) => {
            let ctx_size_inner = unary_term.ctx_size.calc(ctx_size);
            let kind = ExpressionKind::UnaryTerm(Box::new(UnaryTerm {
                expression: generate_expression(&unary_term.expression, ctx_size_inner),
                operator: unary_term.operator.node,
            }));
            let extend_to = match unary_term.operator.node {
                UnaryOperator::Sxt => Extend::Sign(ctx_size),
                _ => Extend::Zero(ctx_size),
            };
            (kind, extend_to)
        }
    };

    Expression { kind, extend_to }
}

pub fn generate_atom<'s>(atom: &mir::Atom<'s>) -> Atom<'s> {
    match atom {
        mir::Atom::Concat(concat) => Atom::Concat(generate_concat_expr(concat)),
        mir::Atom::Register(reg) => Atom::Register(generate_register(reg)),
        mir::Atom::Bus(bus) => Atom::Bus(generate_bus(bus)),
        mir::Atom::RegisterArray(reg_array) => {
            Atom::RegisterArray(generate_register_array(reg_array))
        }
        mir::Atom::Number(number) => Atom::Number(generate_number(&number.node)),
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
        index: Box::new(generate_expression(&reg_array.index, reg_array.index_ctx_size)),
    }
}

pub fn generate_number<'s>(number: &mir::Number) -> Number {
    Number { value: number.value.clone() }
}
