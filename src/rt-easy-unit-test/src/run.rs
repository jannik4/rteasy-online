use crate::unit_test::{
    Assert, Assignment, MicroStep, OperationKind, RemoveBreakpoint, Reset, Run, SetBreakpoint,
    Step, UnitTest,
};
use anyhow::{anyhow, bail, Context, Result};
use rtast as ast;
use rtcore::common::{BusKind, RegisterKind, Span, Spanned};
use rtprogram::{Declaration, Label as ProgramLabel, Program};
use simulator::{Simulator, StepResult, StepResultKind};

// TODO: Better errrors (custom_error+pretty_print instead of anyhow)

pub fn run(program: Program, unit_test: UnitTest) -> Result<()> {
    let mut simulator = Simulator::init(program);

    for operation in unit_test.operations {
        match operation.kind {
            OperationKind::Step(Step { amount }) => {
                for _ in 0..amount.unwrap_or(1) {
                    simulator.step(false).context("Step failed")?;
                }
            }
            OperationKind::MicroStep(MicroStep { amount }) => {
                for _ in 0..amount.unwrap_or(1) {
                    simulator.micro_step(false).context("Micro Step failed")?;
                }
            }
            OperationKind::Run(Run) => {
                while !simulator.is_finished() {
                    let step_result = simulator.step(true).context("Run failed")?;
                    if matches!(
                        step_result,
                        Some(StepResult { kind: StepResultKind::Breakpoint, .. })
                    ) {
                        break;
                    }
                }
            }
            OperationKind::Reset(Reset) => simulator.reset(true),
            OperationKind::SetBreakpoint(SetBreakpoint { label }) => {
                let label = ProgramLabel(label.0);
                simulator.add_breakpoint_at_label(&label);
            }
            OperationKind::RemoveBreakpoint(RemoveBreakpoint { label }) => {
                let label = ProgramLabel(label.0);
                simulator.remove_breakpoint_at_label(&label);
            }
            OperationKind::Assignment(Assignment { assignment }) => {
                let assignment = match parser::parse_assignment(&assignment) {
                    Ok(assignment) => assignment,
                    Err(_e) => bail!("Failed to parse assignment"), // TODO: better error
                };
                exec_assignment(&mut simulator, assignment)?;
            }
            OperationKind::Assert(Assert { assert }) => {
                let assert = match parser::parse_assert(&assert) {
                    Ok(assert) => assert,
                    Err(_e) => bail!("Failed to parse assert"), // TODO: better error
                };
                exec_assert(&mut simulator, assert)?;
            }
        }
    }

    Ok(())
}

fn exec_assignment(simulator: &mut Simulator, assignment: ast::Assignment<'_>) -> Result<()> {
    // Setup
    let test_program = build_test_program(
        simulator.program().declarations(),
        ast::Operation::Assignment(assignment),
    )?;
    let mut test_simulator = setup_test_simulator(&*simulator, test_program)?;

    // Run assignment
    test_simulator.step(false)?;

    // Copy inputs back to simulator
    for bus in test_simulator.buses(BusKind::Intern) {
        let value = test_simulator.bus_value(bus).unwrap();
        simulator.write_bus(bus, value).unwrap();
    }

    Ok(())
}

fn exec_assert(simulator: &mut Simulator, assert: ast::Assert<'_>) -> Result<()> {
    // Setup
    let test_program =
        build_test_program(simulator.program().declarations(), ast::Operation::Assert(assert))?;
    let mut test_simulator = setup_test_simulator(&*simulator, test_program)?;

    // Run assert
    let step_result = test_simulator.micro_step(false)?.unwrap();
    match step_result.kind {
        StepResultKind::Void => Ok(()),
        StepResultKind::AssertError => Err(anyhow!("Assert failed")), // TODO: better error
        StepResultKind::Condition { .. }
        | StepResultKind::Pipe(..)
        | StepResultKind::StatementEnd(..)
        | StepResultKind::Breakpoint => unreachable!(),
    }
}

fn setup_test_simulator(simulator: &Simulator, test_program: Program) -> Result<Simulator> {
    let mut test_simulator = Simulator::init(test_program);

    // Copy inputs
    for bus in simulator.buses(BusKind::Input) {
        let value = simulator.bus_value(bus).unwrap();
        test_simulator.write_bus(bus, value).unwrap();
    }

    // Copy outputs
    for register in simulator.registers(RegisterKind::Output) {
        let value = simulator.register_value(register).unwrap();
        test_simulator.write_register(register, value).unwrap();
    }

    Ok(test_simulator)
}

fn build_test_program(
    declarations: &[Declaration],
    operation: ast::Operation<'_>,
) -> Result<Program> {
    let ast = ast::Ast {
        declarations: map_declarations(declarations),
        statements: vec![ast::Statement {
            label: None,
            operations: ast::Operations {
                operations: vec![operation],
                operations_post: None,
                span: Span::dummy(),
                span_pipe: None,
            },
            span: Span::dummy(),
            span_semicolon: Span::dummy(),
        }],
        trailing_label: None,
    };

    let backend = compiler_backend_simulator::BackendSimulator;
    match compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(program) => Ok(program),
        Err(_e) => Err(anyhow!("Failed to build test program")), // TODO: better error
    }
}

fn map_declarations(declarations: &[Declaration]) -> Vec<ast::Declaration<'_>> {
    declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Register(declare) => {
                let kind = declare.registers[0].kind;
                match kind {
                    RegisterKind::Intern => None,
                    RegisterKind::Output => {
                        Some(ast::Declaration::Register(ast::DeclareRegister {
                            registers: declare
                                .registers
                                .iter()
                                .map(|register| ast::RegBus {
                                    ident: spanned_dummy(ast::Ident(&register.ident.0)),
                                    range: register.range.map(spanned_dummy),
                                    span: Span::dummy(),
                                })
                                .collect(),
                            kind: RegisterKind::Output,
                            span: Span::dummy(),
                        }))
                    }
                }
            }
            Declaration::Bus(declare) => {
                let kind = declare.buses[0].kind;
                match kind {
                    BusKind::Intern => None,
                    BusKind::Input => Some(ast::Declaration::Bus(ast::DeclareBus {
                        buses: declare
                            .buses
                            .iter()
                            .map(|bus| ast::RegBus {
                                ident: spanned_dummy(ast::Ident(&bus.ident.0)),
                                range: bus.range.map(spanned_dummy),
                                span: Span::dummy(),
                            })
                            .collect(),
                        kind: BusKind::Intern, // Map to intern so we can write to the bus
                        span: Span::dummy(),
                    })),
                }
            }
            Declaration::Memory(_) | Declaration::RegisterArray(_) => None,
        })
        .collect()
}

fn spanned_dummy<T>(node: T) -> Spanned<T> {
    Spanned { node, span: Span::dummy() }
}
