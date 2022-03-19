use super::{expression::BuildExpr, Result};
use crate::symbols::Symbols;
use crate::{mir::*, util, InternalError};
use rtast::{self as ast, Either};

pub fn build<'s>(
    operations: Vec<rtast::Operation<'s>>,
    operations_post: Option<Vec<rtast::Operation<'s>>>,
    symbols: &Symbols<'_>,
) -> Result<Vec<Step<'s>>> {
    let mut build = Build::new();

    let context = &Context { is_post_pipe: false, criteria: Vec::new() };
    for operation in operations {
        build_operation(operation, symbols, context, &mut build)?;
    }

    if let Some(operations_post) = operations_post {
        let context = &Context { is_post_pipe: true, criteria: Vec::new() };
        for operation in operations_post {
            build_operation(operation, symbols, context, &mut build)?;
        }
    }

    Ok(build.finish())
}

fn build_operation<'s>(
    operation: ast::Operation<'s>,
    symbols: &Symbols<'_>,
    context: &Context,
    build: &mut Build<'s>,
) -> Result<()> {
    match operation {
        ast::Operation::Nop(nop) => build.push(Step {
            id: build.next_step_id(),
            criteria: context.criteria.clone(),
            operation: Operation::Nop(Nop { span: nop.span }),
            annotation: Annotation::new(false, context.is_post_pipe),
        }),
        ast::Operation::Goto(goto) => build.push(Step {
            id: build.next_step_id(),
            criteria: context.criteria.clone(),
            operation: Operation::Goto(Goto { label: goto.label, span: goto.span }),
            annotation: Annotation::new(false, context.is_post_pipe),
        }),
        ast::Operation::Write(write) => build.push(Step {
            id: build.next_step_id(),
            criteria: context.criteria.clone(),
            operation: Operation::Write(Write { ident: write.ident, span: write.span }),
            annotation: Annotation::new(false, context.is_post_pipe),
        }),
        ast::Operation::Read(read) => build.push(Step {
            id: build.next_step_id(),
            criteria: context.criteria.clone(),
            operation: Operation::Read(Read { ident: read.ident, span: read.span }),
            annotation: Annotation::new(false, context.is_post_pipe),
        }),
        ast::Operation::If(if_) => {
            // EvalCriterion
            let criterion_id = {
                let criterion_id = build.gen_criterion_id();
                let condition = Expression::build(if_.condition, symbols)?;

                build.push(Step {
                    id: build.next_step_id(),
                    criteria: context.criteria.clone(),
                    operation: Operation::EvalCriterion(EvalCriterion {
                        criterion_id,
                        condition: condition.inner,
                    }),
                    annotation: Annotation::new(false, context.is_post_pipe),
                });

                criterion_id
            };

            // If operations
            {
                let context = &context.with(Criterion::True(criterion_id));
                for operation in if_.operations_if {
                    build_operation(operation, symbols, context, build)?;
                }
            }

            // Else operations
            if let Some(operations_else) = if_.operations_else {
                let context = &context.with(Criterion::False(criterion_id));
                for operation in operations_else {
                    build_operation(operation, symbols, context, build)?;
                }
            }
        }
        ast::Operation::Switch(switch) => {
            // Split clauses
            let (cases, default) = split_clauses(switch.clauses)?;

            // Build an eval criterion for every case clause
            let mut eval_criteria = Vec::new();

            let switch_expression = switch.expression;
            let switch_expression_size =
                Expression::build(switch_expression.clone(), symbols)?.size; // TODO: Only get size, do not build
            let cases = cases
                .into_iter()
                .map(|(case, operations)| {
                    let criterion_id = build.gen_criterion_id();
                    eval_criteria.push(EvalCriterion {
                        criterion_id,
                        condition: Expression::build(
                            ast::BinaryTerm {
                                span: case.value.span(),
                                lhs: switch_expression.clone(),
                                rhs: case.value,
                                operator: Spanned { node: BinaryOperator::Eq, span: Span::dummy() },
                            }
                            .into(),
                            symbols,
                        )?
                        .inner,
                    });

                    Ok((criterion_id, operations))
                })
                .collect::<Result<Vec<_>>>()?;

            // Build eval criterion group
            build.push(Step {
                id: build.next_step_id(),
                criteria: context.criteria.clone(),
                operation: Operation::EvalCriterionSwitchGroup(EvalCriterionSwitchGroup {
                    eval_criteria,
                    switch_expression_size,
                    span: switch_expression.span(),
                }),
                annotation: Annotation::new(false, context.is_post_pipe),
            });

            // Build default operations
            {
                let mut context = context.clone();
                for (c_id, _) in &cases {
                    context = context.with(Criterion::False(*c_id));
                }
                for operation in default.1 {
                    build_operation(operation, symbols, &context, build)?;
                }
            }

            // Build case operations
            for (c_id, operations) in cases {
                let context = &context.with(Criterion::True(c_id));
                for operation in operations {
                    build_operation(operation, symbols, context, build)?;
                }
            }
        }
        ast::Operation::Assignment(assignment) => {
            let (lhs, lhs_size) = build_lvalue(assignment.lhs, symbols)?;
            let rhs = Expression::build(assignment.rhs, symbols)?;

            let is_unclocked_assign = match &lhs {
                Lvalue::Register(_) | Lvalue::RegisterArray(_) | Lvalue::ConcatClocked(_) => false,
                Lvalue::Bus(_) | Lvalue::ConcatUnclocked(_) => true,
            };

            build.push(Step {
                id: build.next_step_id(),
                criteria: context.criteria.clone(),
                operation: Operation::Assignment(Assignment {
                    lhs,
                    rhs: rhs.inner,
                    size: lhs_size,
                    span: assignment.span,
                }),
                annotation: Annotation::new(is_unclocked_assign, context.is_post_pipe),
            });
        }
        ast::Operation::Assert(assert) => build.push(Step {
            id: build.next_step_id(),
            criteria: context.criteria.clone(),
            operation: Operation::Assert(Assert {
                condition: Expression::build(assert.condition, symbols)?.inner,
                span: assert.span,
            }),
            annotation: Annotation::new(false, context.is_post_pipe),
        }),
    }

    Ok(())
}

fn split_clauses<'s>(
    clauses: Vec<ast::Clause<'s>>,
) -> Result<(
    Vec<(ast::CaseClause<'s>, Vec<ast::Operation<'s>>)>,
    (ast::DefaultClause, Vec<ast::Operation<'s>>),
)> {
    let mut cases = Vec::with_capacity(clauses.len() - 1);
    let mut default = None;

    for clause in clauses {
        match clause.clause {
            Either::Left(case) => cases.push((case, clause.operations)),
            Either::Right(default_) => default = Some((default_, clause.operations)),
        }
    }

    Ok((cases, default.ok_or_else(|| InternalError("missing default clause".to_owned()))?))
}

fn build_lvalue<'s>(lvalue: ast::Lvalue<'s>, symbols: &Symbols<'_>) -> Result<(Lvalue<'s>, usize)> {
    Ok(match lvalue {
        ast::Lvalue::RegBus(reg_bus) => {
            let reg_bus = <Either<_, _>>::build(reg_bus, symbols)?;
            match reg_bus.inner {
                Either::Left(reg) => (Lvalue::Register(reg), reg_bus.size),
                Either::Right(bus) => (Lvalue::Bus(bus), reg_bus.size),
            }
        }
        ast::Lvalue::RegisterArray(reg_array) => {
            let reg_array = RegisterArray::build(reg_array, symbols)?;
            (Lvalue::RegisterArray(reg_array.inner), reg_array.size)
        }
        ast::Lvalue::Concat(concat) => {
            if util::concat_info(&concat, symbols).contains_clocked {
                let concat = ConcatLvalueClocked::build(concat, symbols)?;
                (Lvalue::ConcatClocked(concat.inner), concat.size)
            } else {
                let concat = ConcatLvalueUnclocked::build(concat, symbols)?;
                (Lvalue::ConcatUnclocked(concat.inner), concat.size)
            }
        }
    })
}

#[derive(Debug, Clone)]
struct Context {
    is_post_pipe: bool,
    criteria: Vec<Criterion>,
}

impl Context {
    fn with(&self, criterion: Criterion) -> Self {
        let mut criteria = self.criteria.clone();
        criteria.push(criterion);

        Self { is_post_pipe: self.is_post_pipe, criteria }
    }
}

use build::*;
mod build {
    use super::{CriterionId, Step, StepId};

    #[derive(Debug)]
    pub struct Build<'s> {
        steps: Vec<Step<'s>>,
        next_criterion_id: usize,
    }

    impl<'s> Build<'s> {
        pub fn new() -> Self {
            Self { steps: Vec::new(), next_criterion_id: 0 }
        }

        pub fn push(&mut self, step: Step<'s>) {
            self.steps.push(step);
        }

        pub fn next_step_id(&self) -> StepId {
            StepId(self.steps.len())
        }

        pub fn gen_criterion_id(&mut self) -> CriterionId {
            let idx = self.next_criterion_id;
            self.next_criterion_id += 1;
            CriterionId(idx)
        }

        pub fn finish(self) -> Vec<Step<'s>> {
            self.steps
        }
    }
}
