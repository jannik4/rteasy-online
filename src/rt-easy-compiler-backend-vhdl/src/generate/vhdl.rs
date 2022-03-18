// use super::{declarations::generate_declarations, statement::generate_statement};
// use crate::vhdl::*;
// use compiler::mir;
// use indexmap::{IndexMap, IndexSet};
//
// pub fn generate_vhdl<'s>(mir: mir::Mir<'s>) -> Vhdl {
//     // Create vhdl
//     let mut vhdl = Vhdl {
//         statements: Vec::new(),
//         criteria: IndexSet::new(),
//         operations: IndexSet::new(),
//
//         declarations: generate_declarations(&mir.declarations),
//     };
//
//     // Generate statements
//     let label_goto_prefix = calc_label_goto_prefix(&mir);
//     let mut transform_labels_list = Vec::new();
//     for (idx, mir_statement) in mir.statements.iter().enumerate() {
//         let transform_labels = generate_statement(
//             idx,
//             mir_statement,
//             mir.statements.get(idx + 1),
//             &mut vhdl,
//             &label_goto_prefix,
//         );
//         if let Some(transform_labels) = transform_labels {
//             transform_labels_list.push(transform_labels);
//         }
//     }
//
//     // Transform labels
//     for (transform_label, fix) in transform_labels_list {
//         for statement in &mut vhdl.statements {
//             transform_labels(&mut statement.next_state_logic, &transform_label, fix.clone());
//         }
//     }
//
//     // Loop statement
//     vhdl.statements.push(Statement {
//         label: Label::terminated(),
//         next_state_logic: NextStateLogic::Label(Label::terminated()),
//         operations: IndexMap::new(),
//     });
//
//     vhdl
// }
//
// fn transform_labels(logic: &mut NextStateLogic, transform_label: &Label, fix: NextStateLogic) {
//     match logic {
//         NextStateLogic::Label(label) => {
//             if label == transform_label {
//                 *logic = fix;
//             }
//         }
//         NextStateLogic::Cond { conditional, default } => {
//             for (_, logic) in conditional {
//                 transform_labels(logic, transform_label, fix.clone());
//             }
//             transform_labels(&mut **default, transform_label, fix);
//         }
//     }
// }
//
// fn calc_label_goto_prefix(mir: &mir::Mir<'_>) -> String {
//     let mut prefix = "_GOTO_".to_string();
//
//     loop {
//         let any_label_contains_prefix = mir
//             .statements
//             .iter()
//             .filter_map(|statement| statement.label.map(|s| s.node))
//             .any(|label| label.0.contains(&prefix));
//         if any_label_contains_prefix {
//             prefix += "_";
//         } else {
//             break;
//         }
//     }
//
//     return prefix;
// }
