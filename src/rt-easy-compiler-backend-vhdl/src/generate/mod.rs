mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod transform;
mod util;
mod vhdl;

use self::util::CriteriaMapping;
use crate::vhdl::*;
use indexmap::{IndexMap, IndexSet};

#[derive(Debug)]
struct GenVhdl {
    statements: Vec<GenStatement>,
    criteria: IndexSet<Expression>,  // Index = CriterionId
    operations: IndexSet<Operation>, // Index = OperationId

    declarations: Declarations,
}

#[derive(Debug)]
struct GenStatement {
    label: Label,
    operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
    next_state_logic: GenNextStateLogic,

    has_pipe: bool,
    criteria_mapping: CriteriaMapping,
}

#[derive(Debug, Clone)]
struct GenNextStateLogic {
    conditional: Vec<(Or<And<Criterion>>, Label)>,
    default: Label,
}

pub fn generate<'s>(mir: compiler::mir::Mir<'s>) -> Vhdl {
    // Create gen vhdl
    let mut criteria = IndexSet::new();
    let mut operations = IndexSet::new();
    let declarations = self::declarations::generate_declarations(&mir.declarations);
    let statements = mir
        .statements
        .iter()
        .enumerate()
        .map(|(idx, mir_statement)| {
            //
            self::statement::generate_statement(
                idx,
                &mir_statement,
                mir.statements.get(idx + 1),
                &mut criteria,
                &mut operations,
                &declarations,
            )
        })
        .collect();
    let vhdl = GenVhdl { statements, criteria, operations, declarations };

    // Transform
    self::transform::transform(vhdl)
}
