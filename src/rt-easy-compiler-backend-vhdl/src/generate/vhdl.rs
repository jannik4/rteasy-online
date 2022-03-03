use super::{
    declarations::generate_declarations,
    statement::{gen_to_std, generate_statement, GenNextStateLogic},
};
use crate::vhdl::*;
use compiler::mir;
use indexmap::{IndexMap, IndexSet};

pub fn generate_vhdl<'s>(mir: mir::Mir<'s>, module_name: String) -> Vhdl<'s> {
    // Create vhdl
    let mut vhdl = Vhdl {
        module_name,
        statements: Vec::new(),
        criteria: IndexSet::new(),
        operations: IndexSet::new(),

        declarations: generate_declarations(&mir.declarations),
    };

    // Generate statements
    let mut fix_labels_list = Vec::new();
    for (idx, mir_statement) in mir.statements.iter().enumerate() {
        let fix_labels = generate_statement(
            idx,
            mir_statement,
            mir.statements.get(idx + 1),
            &mut vhdl,
            "_GOTO_", // TODO: Make sure is not in any label, otherwise inc "_"
        );
        if let Some(fix_labels) = fix_labels {
            fix_labels_list.push(fix_labels);
        }
    }

    // Fix labels
    for (fix_label, fix) in fix_labels_list {
        for statement in &mut vhdl.statements {
            fix_labels(&mut statement.next_state_logic, &fix_label, &fix);
        }
    }

    // Loop statement
    vhdl.statements.push(Statement {
        label: Label::terminated(),
        next_state_logic: NextStateLogic::Label(Label::terminated()),
        operations: IndexMap::new(),
    });

    vhdl
}

fn fix_labels(logic: &mut NextStateLogic, fix_label: &Label, fix: &GenNextStateLogic) {
    match logic {
        NextStateLogic::Label(label) => {
            if label == fix_label {
                *logic = gen_to_std(fix.clone());
            }
        }
        NextStateLogic::Cond { conditional, default } => {
            for (_, logic) in conditional {
                fix_labels(logic, fix_label, fix);
            }
            fix_labels(&mut **default, fix_label, fix);
        }
    }
}
