use super::{
    declarations::generate_declarations,
    statement::{gen_to_std, generate_statement, GenNextStateLogic},
};
use crate::vhdl::*;
use compiler::mir;
use indexmap::{IndexMap, IndexSet};

pub fn generate_vhdl<'s>(mir: mir::Mir<'s>, module_name: String) -> Vhdl {
    // Create vhdl
    let mut vhdl = Vhdl {
        module_name,
        statements: Vec::new(),
        criteria: IndexSet::new(),
        operations: IndexSet::new(),

        declarations: generate_declarations(&mir.declarations),
    };

    // Generate statements
    let label_goto_prefix = calc_label_goto_prefix(&mir);
    let mut fix_labels_list = Vec::new();
    for (idx, mir_statement) in mir.statements.iter().enumerate() {
        let fix_labels = generate_statement(
            idx,
            mir_statement,
            mir.statements.get(idx + 1),
            &mut vhdl,
            &label_goto_prefix,
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
