mod util;

use rt_easy_simulator::Simulator;
use rtcore::{program::Ident, value::Value};

const SOURCE: &'static str = r#"
declare register X(7:0)
declare bus B(7:0), C(7:0)

SW1:
switch X {
    case 0 + 0x0  : X <- X + 1, goto SW1
    case 0b1 + "0": if 0 then nop else X <- X + 5, goto SW1 fi
    default       : nop, X <- X - 2
};
# --- 1 ---

SW2:
switch B(2:0).C(0) {
    case "0110": X <- 42
    default    : X <- 1
}, if 1 then B <- 0b11 fi;
# --- 2 ---

"#;

#[test]
fn switch_case() {
    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    for _ in 0..3 {
        simulator.step().unwrap();
    }
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("4").unwrap()
    );

    // 2
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_dec("42").unwrap()
    );
}
