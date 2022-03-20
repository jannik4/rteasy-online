use super::{
    declarations::generate_declarations, next_state_logic_deps::next_state_logic_deps,
    statement::StatementBuilder,
};
use crate::error::SynthError;
use compiler::mir;
use rtvhdl::*;
use rtvhdl::{IndexMap, IndexSet};
use std::collections::HashMap;

#[derive(Debug)]
pub struct VhdlBuilder {
    statements: Vec<Statement>,
    criteria: IndexSet<Expression>,
    operations: IndexSet<Operation>,
    declarations: Declarations,

    transform: HashMap<Label, NextStateLogic>,
    transform_goto_prefix: String,
}

impl VhdlBuilder {
    pub fn build(mir: mir::Mir<'_>) -> Result<Vhdl, SynthError> {
        // Create builder
        let mut builder = Self {
            statements: Vec::new(),
            criteria: IndexSet::new(),
            operations: IndexSet::new(),
            declarations: generate_declarations(&mir.declarations),

            transform: HashMap::new(),
            transform_goto_prefix: calc_label_goto_prefix(&mir),
        };

        // Generate statements
        for (idx, statement) in mir.statements.iter().enumerate() {
            // Labels
            let label = make_label(idx, Some(statement));
            let label_next = make_label(idx + 1, mir.statements.get(idx + 1));

            // Next state logic
            let deps = next_state_logic_deps(statement);
            let transform = match (deps.clocked, deps.unclocked) {
                (_, true) => return Err(SynthError::UnclockedGotoDependency),
                (true, false) => true,
                (false, false) => false,
            };
            if transform && idx == 0 {
                return Err(SynthError::ConditionalGotoInFirstState);
            }

            // Build
            StatementBuilder::build(
                label,
                label_next,
                &statement.steps.node,
                transform,
                &mut builder,
            );
        }

        // Transform labels
        for (from, to) in builder.transform {
            for statement in &mut builder.statements {
                transform(&mut statement.next_state_logic, &from, &to);
            }
        }

        // Add terminated statement
        builder.statements.push(Statement {
            label: Label::terminated(),
            next_state_logic: NextStateLogic::Label(Label::terminated()),
            operations: IndexMap::new(),
        });

        // Finish
        Ok(Vhdl {
            statements: builder.statements,
            criteria: builder.criteria,
            operations: builder.operations,
            declarations: builder.declarations,
        })
    }

    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn insert_criterion(&mut self, expr: Expression) -> CriterionId {
        CriterionId(self.criteria.insert_full(expr).0)
    }

    pub fn insert_operation(&mut self, op: Operation) -> OperationId {
        OperationId(self.operations.insert_full(op).0)
    }

    pub fn insert_transform(&mut self, from: Label, to: NextStateLogic) {
        self.transform.insert(from, to);
    }

    pub fn transform_goto_prefix(&self) -> &str {
        &self.transform_goto_prefix
    }

    pub fn declarations(&self) -> &Declarations {
        &self.declarations
    }
}

fn calc_label_goto_prefix(mir: &mir::Mir<'_>) -> String {
    let mut prefix = "_GOTO_".to_string();

    loop {
        let any_label_contains_prefix = mir
            .statements
            .iter()
            .filter_map(|statement| statement.label.map(|s| s.node))
            .any(|label| label.0.contains(&prefix));
        if any_label_contains_prefix {
            prefix += "_";
        } else {
            break;
        }
    }

    return prefix;
}

fn make_label(idx: usize, statement: Option<&mir::Statement<'_>>) -> Label {
    match statement {
        Some(statement) => match statement.label.as_ref() {
            Some(label) => Label::named(label.node.0),
            None => Label::unnamed(idx),
        },
        None => Label::terminated(),
    }
}

fn transform(logic: &mut NextStateLogic, from: &Label, to: &NextStateLogic) {
    match logic {
        NextStateLogic::Label(label) => {
            if label == from {
                *logic = to.clone();
            }
        }
        NextStateLogic::Cond { conditional, default } => {
            for (_, logic) in conditional {
                transform(logic, from, &to);
            }
            transform(&mut **default, from, to);
        }
    }
}
