mod util;

use rt_easy_simulator::{Simulator, StepResult, StepResultKind};

#[test]
fn micro_step() {
    const SOURCE: &'static str = r#"
        declare register A(7:0)
        declare bus Q(7:0)

        INIT:
        A <- 0;
        A <- 2 | if A(0) then goto INIT fi;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::Void, .. })));

    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::StatementEnd(..), .. })));

    // 2
    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::Void, .. })));

    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::Pipe(..), .. })));

    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::Condition { .. }, .. })));

    let step_result = simulator.micro_step(false).unwrap();
    assert!(matches!(step_result, Some(StepResult { kind: StepResultKind::StatementEnd(..), .. })));
}
