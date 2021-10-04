mod util;

use rt_easy_simulator::{Simulator, StepResultKind};

const SOURCE: &'static str = r#"
declare register X(7:0)
declare bus B(7:0), C(7:0)

X <- 12,
assert 1,
assert X < 42,
assert B > 17,
B <- 18,
assert C = "11111111",
C <- -1,
if 1 then assert X = X fi; # 1

if X > 2 then assert 2 = 5 fi; # 2
"#;

#[test]
fn assert() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    assert!(matches!(
        simulator.step(false).unwrap().unwrap().kind,
        StepResultKind::StatementEnd(..)
    ));

    // 2
    assert!(matches!(simulator.step(false).unwrap().unwrap().kind, StepResultKind::AssertError));
    assert!(simulator.step(false).unwrap().is_none());
}
