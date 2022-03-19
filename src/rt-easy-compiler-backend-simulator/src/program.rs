use crate::{gen_label, Generate, Result};
use compiler::mir;
use rtprogram::*;

impl Generate<mir::Mir<'_>> for Program {
    fn generate(mir: mir::Mir<'_>) -> Result<Self> {
        let declarations = Generate::generate(mir.declarations)?;
        let statements = Generate::generate(mir.statements)?;
        Ok(Program::new_unchecked(declarations, statements))
    }
}

impl Generate<mir::Statement<'_>> for Statement {
    fn generate(statement: mir::Statement<'_>) -> Result<Self> {
        Ok(Statement {
            label: statement.label.map(|s| Spanned { node: gen_label(s.node), span: s.span }),
            steps: {
                let split_at = statement
                    .steps
                    .node
                    .iter()
                    .position(|step| step.annotation.is_post_pipe)
                    .unwrap_or(statement.steps.node.len());

                Spanned {
                    node: SplitVec::new(Generate::generate(statement.steps.node)?, split_at),
                    span: statement.steps.span,
                }
            },
            span: statement.span,
            span_semicolon: statement.span_semicolon,
            span_pipe: statement.span_pipe,
        })
    }
}

impl Generate<mir::Step<'_>> for Step {
    fn generate(step: mir::Step<'_>) -> Result<Self> {
        Ok(Step {
            criteria: Generate::generate(step.criteria)?,
            operation: Generate::generate(step.operation)?,
        })
    }
}

impl Generate<mir::CriterionId> for CriterionId {
    fn generate(criterion_id: mir::CriterionId) -> Result<Self> {
        Ok(CriterionId(criterion_id.0))
    }
}

impl Generate<mir::Criterion> for Criterion {
    fn generate(criterion: mir::Criterion) -> Result<Self> {
        Ok(match criterion {
            mir::Criterion::True(id) => Criterion::True(Generate::generate(id)?),
            mir::Criterion::False(id) => Criterion::False(Generate::generate(id)?),
        })
    }
}
