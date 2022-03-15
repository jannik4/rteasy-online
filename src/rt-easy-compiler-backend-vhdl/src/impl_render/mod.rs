mod sensitivity_list;

use crate::vhdl::{
    BitRange, BusKind, Declarations, Expression, Ident, Lvalue, NextStateLogic, Operation,
    RegisterKind, Statement, Vhdl,
};
use crate::{render_as_rt::RenderAsRt, render_as_vhdl::RenderAsVhdl};
use indexmap::IndexSet;
use std::fmt::Write;
use temply::Template;

pub fn render(vhdl: &Vhdl<'_>) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();
    VhdlTemplate {
        module_name: &vhdl.module_name,
        statements: &vhdl.statements,
        criteria: &vhdl.criteria,
        operations: &vhdl.operations,
        declarations: &vhdl.declarations,
    }
    .render(&mut buffer)?;
    Ok(buffer)
}

#[derive(Debug, Template)]
#[dedent]
#[template = "./impl_render/template.vhdl"]
struct VhdlTemplate<'a> {
    module_name: &'a str,
    statements: &'a [Statement],
    criteria: &'a IndexSet<Expression<'a>>, // Index = CriterionId
    operations: &'a IndexSet<Operation<'a>>, // Index = OperationId

    declarations: &'a Declarations<'a>,
}

impl<'a> VhdlTemplate<'a> {
    fn any_port(&self) -> bool {
        self.declarations.buses.iter().any(|(_, _, kind)| *kind == BusKind::Input)
            || self.declarations.registers.iter().any(|(_, _, kind)| *kind == RegisterKind::Output)
    }

    fn ports_input(&self) -> impl Iterator<Item = (Ident<'a>, BitRange, bool)> + '_ {
        let any_output =
            self.declarations.registers.iter().any(|(_, _, kind)| *kind == RegisterKind::Output);
        let inputs = self
            .declarations
            .buses
            .iter()
            .filter(|(_, _, kind)| *kind == BusKind::Input)
            .collect::<Vec<_>>();
        let len = inputs.len();

        inputs.into_iter().enumerate().map(move |(idx, (name, range, _))| {
            let is_last = !any_output && idx == len - 1;
            (*name, *range, is_last)
        })
    }

    fn ports_output(&self) -> impl Iterator<Item = (Ident<'a>, BitRange, bool)> + '_ {
        let outputs = self
            .declarations
            .registers
            .iter()
            .filter(|(_, _, kind)| *kind == RegisterKind::Output)
            .collect::<Vec<_>>();
        let len = outputs.len();

        outputs.into_iter().enumerate().map(move |(idx, (name, range, _))| {
            let is_last = idx == len - 1;
            (*name, *range, is_last)
        })
    }

    fn operations(&self, clocked: bool) -> impl Iterator<Item = (usize, &Operation<'a>)> + '_ {
        self.operations.iter().enumerate().filter(move |(_, op)| op.is_clocked() == clocked)
    }

    fn operations_tmp_var(&self, clocked: bool) -> impl Iterator<Item = (usize, BitRange)> + '_ {
        self.operations.iter().enumerate().filter_map(move |(idx, op)| {
            if op.is_clocked() == clocked {
                match op {
                    Operation::Write(_) | Operation::Read(_) => None,
                    Operation::Assignment(assignment) => match assignment.lhs {
                        Lvalue::Register(_) | Lvalue::Bus(_) | Lvalue::RegisterArray(_) => None,
                        Lvalue::ConcatClocked(_) | Lvalue::ConcatUnclocked(_) => {
                            Some((idx, BitRange::Downto(assignment.rhs.extend_to.size() - 1, 0)))
                        }
                    },
                }
            } else {
                None
            }
        })
    }

    fn sensitivity_list_bus_mux(&self) -> String {
        let expressions = self.operations.iter().filter_map(|op| match op {
            Operation::Write(_) => None,
            Operation::Read(_) => None,
            Operation::Assignment(assignment) => {
                if op.is_clocked() {
                    None
                } else {
                    Some(&assignment.rhs)
                }
            }
        });
        let items = sensitivity_list::build(expressions);

        let mut buffer = "(c".to_string();
        for item in items {
            write!(&mut buffer, ", {}", RenderAsVhdl(&item)).unwrap();
        }
        buffer += ")";

        buffer
    }

    fn sensitivity_list_criteria_gen(&self) -> String {
        let expressions = self.criteria.iter();
        let mut items = sensitivity_list::build(expressions).into_iter();

        match items.next() {
            Some(item) => {
                let mut buffer = "(".to_string();
                write!(&mut buffer, "{}", RenderAsVhdl(&item)).unwrap();
                for item in items {
                    write!(&mut buffer, ", {}", RenderAsVhdl(&item)).unwrap();
                }
                buffer += ")";
                buffer
            }
            None => "(c) -- c is used here because vhdl simulators would otherwise \
                get stuck at an empty sensitivity list"
                .to_string(),
        }
    }
}
